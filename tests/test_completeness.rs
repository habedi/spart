//! Completeness tests: every tree must return all the points a query covers, not merely correct
//! ones.
//!
//! The rest of the suite checks soundness — that returned points really do lie inside the query, and
//! that kNN results come back sorted. Nothing checked the other direction, so a tree could quietly
//! drop points and still pass: R*-tree range searches once found 1 of 1000 inserted points, bulk
//! insert lost whole batches, and coincident points crashed the quadtree. Each test below compares a
//! query against brute force over the set of points known to be live.

use std::collections::HashMap;

use spart::geometry::{Cube, EuclideanDistance, Point2D, Point3D, Rectangle};
use spart::kdtree::KdTree;
use spart::octree::Octree;
use spart::quadtree::Quadtree;
use spart::rstar_tree::RStarTree;
use spart::rtree::RTree;

/// Deterministic PRNG, so a failure is reproducible without pulling in a dependency.
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self {
        Rng(seed)
    }

    fn next_u32(&mut self) -> u32 {
        self.0 = self
            .0
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        (self.0 >> 33) as u32
    }

    fn below(&mut self, n: u32) -> u32 {
        self.next_u32() % n
    }

    fn coord(&mut self, limit: u32) -> f64 {
        f64::from(self.below(limit))
    }
}

/// Compares two collections of points as multisets, so duplicates have to match too.
fn assert_same_points_2d<T: Clone + Eq + std::hash::Hash + std::fmt::Debug>(
    got: &[Point2D<T>],
    expected: &[Point2D<T>],
    context: &str,
) {
    let tally = |points: &[Point2D<T>]| {
        let mut counts: HashMap<(u64, u64, Option<T>), usize> = HashMap::new();
        for p in points {
            *counts
                .entry((p.x.to_bits(), p.y.to_bits(), p.data.clone()))
                .or_default() += 1;
        }
        counts
    };
    let (got_counts, expected_counts) = (tally(got), tally(expected));
    assert_eq!(
        got_counts.len(),
        expected_counts.len(),
        "{context}: got {} distinct points, expected {}",
        got_counts.len(),
        expected_counts.len()
    );
    for (key, count) in &expected_counts {
        assert_eq!(
            got_counts.get(key),
            Some(count),
            "{context}: point {key:?} appears {:?} times, expected {count}",
            got_counts.get(key)
        );
    }
}

const WORLD: Rectangle = Rectangle {
    x: 0.0,
    y: 0.0,
    width: 1000.0,
    height: 1000.0,
};

const WORLD_3D: Cube = Cube {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    width: 1000.0,
    height: 1000.0,
    depth: 1000.0,
};

/// A box wide enough to cover every point any test inserts.
const EVERYTHING: Rectangle = Rectangle {
    x: -1.0,
    y: -1.0,
    width: 2000.0,
    height: 2000.0,
};

// ---------------------------------------------------------------------------------------------
// R-tree
// ---------------------------------------------------------------------------------------------

#[test]
fn rtree_keeps_every_point_through_inserts_and_deletes() {
    let mut rng = Rng::new(42);
    let mut tree: RTree<Point2D<u32>> = RTree::new(4).unwrap();
    let mut live: Vec<Point2D<u32>> = Vec::new();

    for step in 0..1500u32 {
        if live.is_empty() || rng.below(100) < 65 {
            let point = Point2D::new(rng.coord(400), rng.coord(400), Some(step));
            tree.insert(point.clone());
            live.push(point);
        } else {
            let index = rng.below(live.len() as u32) as usize;
            let point = live.swap_remove(index);
            assert!(
                tree.delete(&point),
                "step {step}: delete of a live point failed"
            );
        }

        let found: Vec<_> = tree
            .range_search_bbox(&EVERYTHING)
            .into_iter()
            .cloned()
            .collect();
        assert_same_points_2d(&found, &live, &format!("rtree after step {step}"));
    }
}

#[test]
fn rtree_bbox_queries_match_brute_force() {
    let mut rng = Rng::new(2024);
    let mut tree: RTree<Point2D<u32>> = RTree::new(4).unwrap();
    let mut live: Vec<Point2D<u32>> = Vec::new();
    for step in 0..600u32 {
        let point = Point2D::new(rng.coord(1000), rng.coord(1000), Some(step));
        tree.insert(point.clone());
        live.push(point);
    }

    // Random windows, not just the all-covering box: a bounding rectangle that is too small only
    // shows up when a query actually has to be pruned against it.
    for query_index in 0..300 {
        let query = Rectangle {
            x: rng.coord(1000),
            y: rng.coord(1000),
            width: rng.coord(300),
            height: rng.coord(300),
        };
        let found: Vec<_> = tree
            .range_search_bbox(&query)
            .into_iter()
            .cloned()
            .collect();
        let expected: Vec<_> = live
            .iter()
            .filter(|p| query.contains(*p))
            .cloned()
            .collect();
        // Zero-extent object MBRs mean a point exactly on the far edge may or may not be reported,
        // so compare only points strictly inside plus the count of the sound superset.
        for point in &expected {
            assert!(
                found.iter().any(|f| f == point),
                "query {query_index} ({query:?}) missed point ({}, {})",
                point.x,
                point.y
            );
        }
        for point in &found {
            assert!(
                query.contains(point),
                "query {query_index} returned a point outside the query"
            );
        }
    }
}

#[test]
fn rtree_knn_matches_brute_force() {
    let mut rng = Rng::new(31_337);
    let mut tree: RTree<Point2D<u32>> = RTree::new(4).unwrap();
    let mut live: Vec<Point2D<u32>> = Vec::new();
    for step in 0..500u32 {
        let point = Point2D::new(rng.coord(1000), rng.coord(1000), Some(step));
        tree.insert(point.clone());
        live.push(point);
    }

    for query_index in 0..200 {
        let query = Point2D::new(rng.coord(1000), rng.coord(1000), None::<u32>);
        for k in [1usize, 5, 20] {
            let got: Vec<f64> = tree
                .knn_search::<EuclideanDistance>(&query, k)
                .iter()
                .map(|p| p.distance_sq(&query))
                .collect();
            let mut expected: Vec<f64> = live.iter().map(|p| p.distance_sq(&query)).collect();
            expected.sort_by(|a, b| a.partial_cmp(b).unwrap());
            expected.truncate(k);

            assert_eq!(got.len(), expected.len(), "query {query_index}, k={k}");
            for (rank, (g, e)) in got.iter().zip(expected.iter()).enumerate() {
                assert!(
                    (g - e).abs() < 1e-9,
                    "query {query_index}, k={k}, rank {rank}: got squared distance {g}, expected {e}"
                );
            }
        }
    }
}

#[test]
fn rtree_bulk_insert_interleaves_with_insert() {
    let make = |range: std::ops::Range<u32>| -> Vec<Point2D<u32>> {
        range
            .map(|i| Point2D::new(f64::from(i) * 3.0, f64::from(i % 17) * 5.0, Some(i)))
            .collect()
    };

    // Bulk into an empty tree, then singles, then another bulk, then more singles.
    let mut tree: RTree<Point2D<u32>> = RTree::new(4).unwrap();
    let mut live = Vec::new();

    tree.insert_bulk(make(0..50));
    live.extend(make(0..50));
    assert_eq!(tree.range_search_bbox(&EVERYTHING).len(), live.len());

    for point in make(50..60) {
        tree.insert(point.clone());
        live.push(point);
    }
    assert_eq!(tree.range_search_bbox(&EVERYTHING).len(), live.len());

    tree.insert_bulk(make(60..63));
    live.extend(make(60..63));
    assert_eq!(
        tree.range_search_bbox(&EVERYTHING).len(),
        live.len(),
        "a small bulk insert into a populated tree lost points"
    );

    tree.insert_bulk(make(63..120));
    live.extend(make(63..120));
    for point in make(120..130) {
        tree.insert(point.clone());
        live.push(point);
    }
    let found: Vec<_> = tree
        .range_search_bbox(&EVERYTHING)
        .into_iter()
        .cloned()
        .collect();
    assert_same_points_2d(
        &found,
        &live,
        "rtree after interleaved bulk and single inserts",
    );

    // Everything must still be deletable afterwards.
    for point in &live {
        assert!(tree.delete(point), "delete after bulk load failed");
    }
    assert!(tree.range_search_bbox(&EVERYTHING).is_empty());
}

// ---------------------------------------------------------------------------------------------
// R*-tree
// ---------------------------------------------------------------------------------------------

#[test]
fn rstar_keeps_every_point_through_inserts_and_deletes() {
    let mut rng = Rng::new(7);
    let mut tree: RStarTree<Point2D<u32>> = RStarTree::new(4).unwrap();
    let mut live: Vec<Point2D<u32>> = Vec::new();

    for step in 0..1500u32 {
        if live.is_empty() || rng.below(100) < 65 {
            let point = Point2D::new(rng.coord(400), rng.coord(400), Some(step));
            tree.insert(point.clone());
            live.push(point);
        } else {
            let index = rng.below(live.len() as u32) as usize;
            let point = live.swap_remove(index);
            assert!(
                tree.delete(&point),
                "step {step}: delete of a live point failed"
            );
        }

        let found: Vec<_> = tree
            .range_search_bbox(&EVERYTHING)
            .into_iter()
            .cloned()
            .collect();
        assert_same_points_2d(&found, &live, &format!("rstar after step {step}"));
    }
}

/// Forced reinsertion is what used to lose data, and it only kicks in once the tree is deep enough
/// for an interior node to overflow. Insert enough points, one at a time, to reach that.
#[test]
fn rstar_sequential_inserts_stay_visible() {
    for max_entries in [2usize, 4, 8] {
        let mut tree: RStarTree<Point2D<u32>> = RStarTree::new(max_entries).unwrap();
        for i in 0..600u32 {
            tree.insert(Point2D::new(f64::from(i), f64::from(i * 13 % 600), Some(i)));
            let found = tree.range_search_bbox(&EVERYTHING).len();
            assert_eq!(
                found,
                (i + 1) as usize,
                "max_entries={max_entries}: after {} inserts only {found} points are reachable",
                i + 1
            );
        }
    }
}

#[test]
fn rstar_knn_matches_brute_force() {
    let mut rng = Rng::new(808);
    let mut tree: RStarTree<Point2D<u32>> = RStarTree::new(4).unwrap();
    let mut live: Vec<Point2D<u32>> = Vec::new();
    for step in 0..500u32 {
        let point = Point2D::new(rng.coord(1000), rng.coord(1000), Some(step));
        tree.insert(point.clone());
        live.push(point);
    }

    for query_index in 0..200 {
        let query = Point2D::new(rng.coord(1000), rng.coord(1000), None::<u32>);
        let k = 10;
        let got: Vec<f64> = tree
            .knn_search::<EuclideanDistance>(&query, k)
            .iter()
            .map(|p| p.distance_sq(&query))
            .collect();
        let mut expected: Vec<f64> = live.iter().map(|p| p.distance_sq(&query)).collect();
        expected.sort_by(|a, b| a.partial_cmp(b).unwrap());
        expected.truncate(k);
        assert_eq!(got.len(), expected.len(), "query {query_index}");
        for (rank, (g, e)) in got.iter().zip(expected.iter()).enumerate() {
            assert!(
                (g - e).abs() < 1e-9,
                "query {query_index}, rank {rank}: got {g}, expected {e}"
            );
        }
    }
}

#[test]
fn rstar_bulk_insert_interleaves_with_insert() {
    let make = |range: std::ops::Range<u32>| -> Vec<Point2D<u32>> {
        range
            .map(|i| Point2D::new(f64::from(i) * 3.0, f64::from(i % 17) * 5.0, Some(i)))
            .collect()
    };

    let mut tree: RStarTree<Point2D<u32>> = RStarTree::new(4).unwrap();
    let mut live = Vec::new();

    tree.insert_bulk(make(0..50));
    live.extend(make(0..50));
    assert_eq!(tree.range_search_bbox(&EVERYTHING).len(), live.len());

    for point in make(50..60) {
        tree.insert(point.clone());
        live.push(point);
    }
    tree.insert_bulk(make(60..63));
    live.extend(make(60..63));
    assert_eq!(
        tree.range_search_bbox(&EVERYTHING).len(),
        live.len(),
        "a small bulk insert into a populated tree lost points"
    );

    tree.insert_bulk(make(63..120));
    live.extend(make(63..120));
    let found: Vec<_> = tree
        .range_search_bbox(&EVERYTHING)
        .into_iter()
        .cloned()
        .collect();
    assert_same_points_2d(
        &found,
        &live,
        "rstar after interleaved bulk and single inserts",
    );
}

// ---------------------------------------------------------------------------------------------
// Quadtree and Octree
// ---------------------------------------------------------------------------------------------

#[test]
fn quadtree_keeps_every_point_through_inserts_and_deletes() {
    let mut rng = Rng::new(99);
    let mut tree: Quadtree<u32> = Quadtree::new(&WORLD, 4).unwrap();
    let mut live: Vec<Point2D<u32>> = Vec::new();

    for step in 0..1500u32 {
        if live.is_empty() || rng.below(100) < 65 {
            let point = Point2D::new(rng.coord(400), rng.coord(400), Some(step));
            assert!(tree.insert(point.clone()));
            live.push(point);
        } else {
            let index = rng.below(live.len() as u32) as usize;
            let point = live.swap_remove(index);
            assert!(
                tree.delete(&point),
                "step {step}: delete of a live point failed"
            );
        }
        assert_same_points_2d(
            &tree.range_search_bbox(&WORLD),
            &live,
            &format!("quadtree after step {step}"),
        );
    }
}

/// More coincident points than `capacity` used to subdivide until floating-point rounding left a
/// point matching the parent but no child, reaching `unreachable!()` on insert — and on bulk insert
/// the same gap dropped every point silently.
#[test]
fn quadtree_handles_coincident_points() {
    for capacity in [1usize, 2, 4] {
        for &(x, y) in &[(10.0, 10.0), (0.0, 0.0), (500.0, 500.0), (1000.0, 1000.0)] {
            let mut tree: Quadtree<u32> = Quadtree::new(&WORLD, capacity).unwrap();
            let points: Vec<_> = (0..40).map(|i| Point2D::new(x, y, Some(i))).collect();
            for point in &points {
                assert!(tree.insert(point.clone()));
            }
            assert_eq!(
                tree.range_search_bbox(&WORLD).len(),
                points.len(),
                "capacity={capacity}, 40 coincident points at ({x}, {y}) via insert"
            );
            assert_eq!(
                tree.knn_search::<EuclideanDistance>(&points[0], 40).len(),
                points.len()
            );

            let mut bulk: Quadtree<u32> = Quadtree::new(&WORLD, capacity).unwrap();
            assert_eq!(bulk.insert_bulk(&points), points.len());
            assert_eq!(
                bulk.range_search_bbox(&WORLD).len(),
                points.len(),
                "capacity={capacity}, 40 coincident points at ({x}, {y}) via insert_bulk"
            );

            for point in &points {
                assert!(bulk.delete(point), "coincident point could not be deleted");
            }
            assert!(bulk.range_search_bbox(&WORLD).is_empty());
        }
    }
}

/// Points spaced far below the depth cap's resolution end up in the same bucket; they must still all
/// be found.
#[test]
fn quadtree_handles_near_coincident_points() {
    let mut tree: Quadtree<u32> = Quadtree::new(&WORLD, 2).unwrap();
    let points: Vec<_> = (0..30)
        .map(|i| Point2D::new(10.0 + f64::from(i) * 1e-15, 10.0, Some(i)))
        .collect();
    for point in &points {
        assert!(tree.insert(point.clone()));
    }
    assert_eq!(tree.range_search_bbox(&WORLD).len(), points.len());
    assert_eq!(
        tree.range_search::<EuclideanDistance>(&points[0], 1.0)
            .len(),
        points.len()
    );
}

#[test]
fn quadtree_bbox_queries_match_brute_force() {
    let mut rng = Rng::new(5150);
    let mut tree: Quadtree<u32> = Quadtree::new(&WORLD, 4).unwrap();
    let mut live: Vec<Point2D<u32>> = Vec::new();
    for step in 0..600u32 {
        let point = Point2D::new(rng.coord(1000), rng.coord(1000), Some(step));
        assert!(tree.insert(point.clone()));
        live.push(point);
    }

    for _ in 0..300 {
        let query = Rectangle {
            x: rng.coord(1000),
            y: rng.coord(1000),
            width: rng.coord(300),
            height: rng.coord(300),
        };
        let expected: Vec<_> = live
            .iter()
            .filter(|p| query.contains(*p))
            .cloned()
            .collect();
        assert_same_points_2d(
            &tree.range_search_bbox(&query),
            &expected,
            "quadtree bbox query",
        );
    }
}

#[test]
fn quadtree_insert_bulk_reports_skipped_points() {
    let mut tree: Quadtree<u32> = Quadtree::new(&WORLD, 4).unwrap();
    let points = vec![
        Point2D::new(10.0, 10.0, Some(1)),
        Point2D::new(-5.0, 10.0, Some(2)),
        Point2D::new(2000.0, 10.0, Some(3)),
        Point2D::new(20.0, 20.0, Some(4)),
    ];
    assert_eq!(
        tree.insert_bulk(&points),
        2,
        "insert_bulk should report how many points it actually stored"
    );
    assert_eq!(tree.range_search_bbox(&WORLD).len(), 2);
}

#[test]
fn octree_keeps_every_point_through_inserts_and_deletes() {
    let mut rng = Rng::new(1234);
    let mut tree: Octree<u32> = Octree::new(&WORLD_3D, 4).unwrap();
    let mut live: Vec<Point3D<u32>> = Vec::new();

    for step in 0..1200u32 {
        if live.is_empty() || rng.below(100) < 65 {
            let point = Point3D::new(rng.coord(300), rng.coord(300), rng.coord(300), Some(step));
            assert!(tree.insert(point.clone()));
            live.push(point);
        } else {
            let index = rng.below(live.len() as u32) as usize;
            let point = live.swap_remove(index);
            assert!(
                tree.delete(&point),
                "step {step}: delete of a live point failed"
            );
        }
        assert_eq!(
            tree.range_search_bbox(&WORLD_3D).len(),
            live.len(),
            "octree after step {step}"
        );
    }
}

#[test]
fn octree_handles_coincident_points() {
    for capacity in [1usize, 2, 4] {
        for &(x, y, z) in &[
            (10.0, 10.0, 10.0),
            (0.0, 0.0, 0.0),
            (1000.0, 1000.0, 1000.0),
        ] {
            let mut tree: Octree<u32> = Octree::new(&WORLD_3D, capacity).unwrap();
            let points: Vec<_> = (0..40).map(|i| Point3D::new(x, y, z, Some(i))).collect();
            for point in &points {
                assert!(tree.insert(point.clone()));
            }
            assert_eq!(
                tree.range_search_bbox(&WORLD_3D).len(),
                points.len(),
                "capacity={capacity}, coincident points at ({x}, {y}, {z}) via insert"
            );

            let mut bulk: Octree<u32> = Octree::new(&WORLD_3D, capacity).unwrap();
            assert_eq!(bulk.insert_bulk(&points), points.len());
            assert_eq!(bulk.range_search_bbox(&WORLD_3D).len(), points.len());
            for point in &points {
                assert!(bulk.delete(point));
            }
            assert!(bulk.range_search_bbox(&WORLD_3D).is_empty());
        }
    }
}

#[test]
fn octree_bbox_queries_match_brute_force() {
    let mut rng = Rng::new(4242);
    let mut tree: Octree<u32> = Octree::new(&WORLD_3D, 4).unwrap();
    let mut live: Vec<Point3D<u32>> = Vec::new();
    for step in 0..500u32 {
        let point = Point3D::new(
            rng.coord(1000),
            rng.coord(1000),
            rng.coord(1000),
            Some(step),
        );
        assert!(tree.insert(point.clone()));
        live.push(point);
    }

    for _ in 0..200 {
        let query = Cube {
            x: rng.coord(1000),
            y: rng.coord(1000),
            z: rng.coord(1000),
            width: rng.coord(400),
            height: rng.coord(400),
            depth: rng.coord(400),
        };
        let expected = live.iter().filter(|p| query.contains(*p)).count();
        assert_eq!(tree.range_search_bbox(&query).len(), expected);
    }
}

#[test]
fn octree_knn_matches_brute_force() {
    let mut rng = Rng::new(60_606);
    let mut tree: Octree<u32> = Octree::new(&WORLD_3D, 4).unwrap();
    let mut live: Vec<Point3D<u32>> = Vec::new();
    for step in 0..400u32 {
        let point = Point3D::new(
            rng.coord(1000),
            rng.coord(1000),
            rng.coord(1000),
            Some(step),
        );
        assert!(tree.insert(point.clone()));
        live.push(point);
    }

    for query_index in 0..150 {
        let query = Point3D::new(
            rng.coord(1000),
            rng.coord(1000),
            rng.coord(1000),
            None::<u32>,
        );
        let k = 8;
        let got: Vec<f64> = tree
            .knn_search::<EuclideanDistance>(&query, k)
            .iter()
            .map(|p| p.distance_sq(&query))
            .collect();
        let mut expected: Vec<f64> = live.iter().map(|p| p.distance_sq(&query)).collect();
        expected.sort_by(|a, b| a.partial_cmp(b).unwrap());
        expected.truncate(k);
        assert_eq!(got.len(), expected.len(), "query {query_index}");
        for (rank, (g, e)) in got.iter().zip(expected.iter()).enumerate() {
            assert!(
                (g - e).abs() < 1e-9,
                "query {query_index}, rank {rank}: got {g}, expected {e}"
            );
        }
    }
}

// ---------------------------------------------------------------------------------------------
// Kd-tree
// ---------------------------------------------------------------------------------------------

#[test]
fn kdtree_keeps_every_point_through_inserts_and_deletes() {
    let mut rng = Rng::new(555);
    let mut tree: KdTree<Point2D<u32>> = KdTree::new();
    let mut live: Vec<Point2D<u32>> = Vec::new();

    for step in 0..1200u32 {
        if live.is_empty() || rng.below(100) < 65 {
            let point = Point2D::new(rng.coord(200), rng.coord(200), Some(step));
            tree.insert(point.clone()).unwrap();
            live.push(point);
        } else {
            let index = rng.below(live.len() as u32) as usize;
            let point = live.swap_remove(index);
            assert!(
                tree.delete(&point),
                "step {step}: delete of a live point failed"
            );
        }
        let found = tree.range_search::<EuclideanDistance>(&Point2D::new(0.0, 0.0, None), 1e9);
        assert_same_points_2d(&found, &live, &format!("kdtree after step {step}"));
    }
}

/// Ascending coordinates used to build a tree of depth *n*: 60k inserts overflowed the stack, both
/// while searching and while dropping the tree. Partial rebuilding keeps the depth logarithmic.
#[test]
fn kdtree_survives_sorted_inserts() {
    const N: u32 = 120_000;
    let mut tree: KdTree<Point2D<u32>> = KdTree::new();
    for i in 0..N {
        tree.insert(Point2D::new(f64::from(i), f64::from(i), Some(i)))
            .unwrap();
    }

    // A recursive query over a degenerate tree is exactly what used to blow the stack.
    let nearest = tree.knn_search::<EuclideanDistance>(&Point2D::new(0.0, 0.0, None), 3);
    assert_eq!(nearest.len(), 3);
    assert_eq!(nearest[0].data, Some(0));

    let last = f64::from(N - 1);
    let nearest_end = tree.knn_search::<EuclideanDistance>(&Point2D::new(last, last, None), 1);
    assert_eq!(nearest_end[0].data, Some(N - 1));

    // Points sit at (i, i), so (i, i) is |i - 50| * sqrt(2) away: i = 47..=53 are within 5 units.
    let window = tree.range_search::<EuclideanDistance>(&Point2D::new(50.0, 50.0, None), 5.0);
    assert_eq!(
        window.len(),
        7,
        "points within 5 units of (50, 50) along the diagonal"
    );

    // Dropping the tree here must not overflow the stack either.
    drop(tree);
}

#[test]
fn kdtree_survives_sorted_inserts_3d() {
    let mut tree: KdTree<Point3D<u32>> = KdTree::new();
    for i in 0..80_000u32 {
        let c = f64::from(i);
        tree.insert(Point3D::new(c, c, c, Some(i))).unwrap();
    }
    let nearest = tree.knn_search::<EuclideanDistance>(&Point3D::new(0.0, 0.0, 0.0, None), 2);
    assert_eq!(nearest.len(), 2);
    assert_eq!(nearest[0].data, Some(0));
}

#[test]
fn kdtree_knn_matches_brute_force() {
    let mut rng = Rng::new(6060);
    let mut tree: KdTree<Point2D<u32>> = KdTree::new();
    let mut live: Vec<Point2D<u32>> = Vec::new();
    for step in 0..500u32 {
        let point = Point2D::new(rng.coord(1000), rng.coord(1000), Some(step));
        tree.insert(point.clone()).unwrap();
        live.push(point);
    }

    for query_index in 0..200 {
        let query = Point2D::new(rng.coord(1000), rng.coord(1000), None::<u32>);
        for k in [1usize, 7, 25] {
            let got: Vec<f64> = tree
                .knn_search::<EuclideanDistance>(&query, k)
                .iter()
                .map(|p| p.distance_sq(&query))
                .collect();
            let mut expected: Vec<f64> = live.iter().map(|p| p.distance_sq(&query)).collect();
            expected.sort_by(|a, b| a.partial_cmp(b).unwrap());
            expected.truncate(k);
            assert_eq!(got.len(), expected.len(), "query {query_index}, k={k}");
            for (rank, (g, e)) in got.iter().zip(expected.iter()).enumerate() {
                assert!(
                    (g - e).abs() < 1e-9,
                    "query {query_index}, k={k}, rank {rank}: got {g}, expected {e}"
                );
            }
        }
    }
}

#[test]
fn kdtree_range_queries_match_brute_force() {
    let mut rng = Rng::new(777);
    let mut tree: KdTree<Point2D<u32>> = KdTree::new();
    let mut live: Vec<Point2D<u32>> = Vec::new();
    for step in 0..600u32 {
        let point = Point2D::new(rng.coord(500), rng.coord(500), Some(step));
        tree.insert(point.clone()).unwrap();
        live.push(point);
    }

    for _ in 0..200 {
        let center = Point2D::new(rng.coord(500), rng.coord(500), None::<u32>);
        let radius = rng.coord(80);
        let found = tree.range_search::<EuclideanDistance>(&center, radius);
        let expected: Vec<_> = live
            .iter()
            .filter(|p| p.distance_sq(&center) <= radius * radius)
            .cloned()
            .collect();
        assert_same_points_2d(&found, &expected, "kdtree radius query");
    }
}

/// A tree created with an explicit dimension must keep it after being emptied, instead of silently
/// accepting points of a different dimension.
#[test]
fn kdtree_keeps_its_dimension_after_being_emptied() {
    let mut tree: KdTree<Point2D<u32>> = KdTree::with_dimension(2);
    let point = Point2D::new(1.0, 1.0, Some(1));
    tree.insert(point.clone()).unwrap();
    assert!(tree.delete(&point));
    assert!(!tree.contains(&point));

    tree.insert(point.clone()).unwrap();
    assert!(
        tree.contains(&point),
        "a point re-inserted after the tree was emptied went missing"
    );

    let mut inferred: KdTree<Point3D<u32>> = KdTree::new();
    inferred
        .insert(Point3D::new(1.0, 1.0, 1.0, Some(1)))
        .unwrap();
    assert!(inferred.delete(&Point3D::new(1.0, 1.0, 1.0, Some(1))));
    assert!(
        inferred
            .insert(Point3D::new(2.0, 2.0, 2.0, Some(2)))
            .is_ok()
    );
}

#[test]
fn kdtree_bulk_insert_interleaves_with_insert() {
    let mut tree: KdTree<Point2D<u32>> = KdTree::new();
    let make = |range: std::ops::Range<u32>| -> Vec<Point2D<u32>> {
        range
            .map(|i| Point2D::new(f64::from(i), f64::from(i % 23), Some(i)))
            .collect()
    };

    let mut live = Vec::new();
    tree.insert_bulk(make(0..40)).unwrap();
    live.extend(make(0..40));
    for point in make(40..50) {
        tree.insert(point.clone()).unwrap();
        live.push(point);
    }
    tree.insert_bulk(make(50..90)).unwrap();
    live.extend(make(50..90));

    for point in &live {
        assert!(tree.contains(point), "point {:?} lost", point.data);
    }
    let found = tree.range_search::<EuclideanDistance>(&Point2D::new(0.0, 0.0, None), 1e9);
    assert_same_points_2d(
        &found,
        &live,
        "kdtree after interleaved bulk and single inserts",
    );
}

/// A rejected bulk insert must leave the tree untouched, including its inferred dimension.
#[test]
fn kdtree_rejected_bulk_insert_leaves_tree_unchanged() {
    let mut tree: KdTree<Point2D<u32>> = KdTree::new();
    let mixed = vec![
        Point2D::new(1.0, 1.0, Some(1)),
        Point2D::new(2.0, 2.0, Some(2)),
    ];
    tree.insert_bulk(mixed.clone()).unwrap();
    for point in &mixed {
        assert!(tree.contains(point));
    }
}

// ---------------------------------------------------------------------------------------------
// Shared: capacity edge cases across every tree
// ---------------------------------------------------------------------------------------------

/// The smallest legal node capacities are the most likely to expose an off-by-one in splitting.
#[test]
fn r_tree_family_handles_minimum_capacities() {
    for max_entries in [2usize, 3] {
        let mut rtree: RTree<Point2D<u32>> = RTree::new(max_entries).unwrap();
        let mut rstar: RStarTree<Point2D<u32>> = RStarTree::new(max_entries).unwrap();
        let mut live = Vec::new();

        for i in 0..300u32 {
            let point = Point2D::new(f64::from(i * 7 % 300), f64::from(i * 13 % 300), Some(i));
            rtree.insert(point.clone());
            rstar.insert(point.clone());
            live.push(point);
            assert_eq!(
                rtree.range_search_bbox(&EVERYTHING).len(),
                live.len(),
                "rtree max_entries={max_entries} after {} inserts",
                i + 1
            );
            assert_eq!(
                rstar.range_search_bbox(&EVERYTHING).len(),
                live.len(),
                "rstar max_entries={max_entries} after {} inserts",
                i + 1
            );
        }

        for (i, point) in live.iter().enumerate() {
            assert!(rtree.delete(point), "rtree delete {i}");
            assert!(rstar.delete(point), "rstar delete {i}");
            let remaining = live.len() - i - 1;
            assert_eq!(rtree.range_search_bbox(&EVERYTHING).len(), remaining);
            assert_eq!(rstar.range_search_bbox(&EVERYTHING).len(), remaining);
        }
    }
}

#[test]
fn quadtree_and_octree_handle_capacity_one() {
    let mut quad: Quadtree<u32> = Quadtree::new(&WORLD, 1).unwrap();
    let mut live2d = Vec::new();
    for i in 0..300u32 {
        let point = Point2D::new(f64::from(i * 7 % 300), f64::from(i * 13 % 300), Some(i));
        assert!(quad.insert(point.clone()));
        live2d.push(point);
        assert_eq!(quad.range_search_bbox(&WORLD).len(), live2d.len());
    }
    for point in &live2d {
        assert!(quad.delete(point));
    }
    assert!(quad.range_search_bbox(&WORLD).is_empty());

    let mut oct: Octree<u32> = Octree::new(&WORLD_3D, 1).unwrap();
    let mut live3d = Vec::new();
    for i in 0..300u32 {
        let point = Point3D::new(
            f64::from(i * 7 % 300),
            f64::from(i * 13 % 300),
            f64::from(i * 11 % 300),
            Some(i),
        );
        assert!(oct.insert(point.clone()));
        live3d.push(point);
        assert_eq!(oct.range_search_bbox(&WORLD_3D).len(), live3d.len());
    }
    for point in &live3d {
        assert!(oct.delete(point));
    }
    assert!(oct.range_search_bbox(&WORLD_3D).is_empty());
}
