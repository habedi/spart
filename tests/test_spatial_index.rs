//! One harness, every tree.
//!
//! These tests are written against [`SpatialIndex`] rather than against a concrete tree, so each
//! assertion runs five times over the quadtree, octree, Kd-tree, R-tree, and R*-tree. Before the
//! trait existed the same checks had to be copied per tree, which is how the trees drifted into five
//! different contracts for the same operations in the first place.

use spart::errors::SpartError;
use spart::geometry::{Cube, EuclideanDistance, Point2D, Point3D, Rectangle};
use spart::index::SpatialIndex;
use spart::kdtree::KdTree;
use spart::octree::Octree;
use spart::quadtree::Quadtree;
use spart::rstar_tree::RStarTree;
use spart::rtree::RTree;

const BOUNDARY_2D: Rectangle = Rectangle {
    x: 0.0,
    y: 0.0,
    width: 100.0,
    height: 100.0,
};

const BOUNDARY_3D: Cube = Cube {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    width: 100.0,
    height: 100.0,
    depth: 100.0,
};

fn points_2d(n: u32) -> Vec<Point2D<u32>> {
    (0..n)
        .map(|i| Point2D::new(f64::from(i * 7 % 100), f64::from(i * 13 % 100), Some(i)))
        .collect()
}

fn points_3d(n: u32) -> Vec<Point3D<u32>> {
    (0..n)
        .map(|i| {
            Point3D::new(
                f64::from(i * 7 % 100),
                f64::from(i * 13 % 100),
                f64::from(i * 11 % 100),
                Some(i),
            )
        })
        .collect()
}

/// Exercises the contract every index must satisfy.
///
/// `everything` has to be a volume covering every item in `items`.
fn check_contract<I>(mut index: I, items: Vec<I::Item>, everything: &I::Volume, name: &str)
where
    I: SpatialIndex,
    I::Item: Clone + PartialEq + std::fmt::Debug,
    EuclideanDistance: spart::geometry::DistanceMetric<I::Item>,
{
    assert!(index.is_empty(), "{name}: a fresh index should be empty");
    assert_eq!(index.len(), 0, "{name}: a fresh index should have length 0");
    assert!(
        index.range_search_bbox(everything).is_empty(),
        "{name}: a fresh index should answer no results"
    );
    assert!(
        index
            .knn_search::<EuclideanDistance>(&items[0], 3)
            .is_empty(),
        "{name}: a fresh index should have no neighbors"
    );

    // Insertion reports acceptance and moves the count.
    for (i, item) in items.iter().enumerate() {
        assert_eq!(
            SpatialIndex::insert(&mut index, item.clone()),
            Ok(true),
            "{name}: item {i} should have been accepted"
        );
        assert_eq!(index.len(), i + 1, "{name}: length after {} inserts", i + 1);
    }
    assert!(!index.is_empty(), "{name}: a populated index is not empty");

    // Everything inserted is present and findable.
    for (i, item) in items.iter().enumerate() {
        assert!(index.contains(item), "{name}: item {i} should be present");
    }
    assert_eq!(
        index.range_search_bbox(everything).len(),
        items.len(),
        "{name}: a covering box query must return every item"
    );

    // A nearest-neighbor search finds the item itself first, and respects k.
    for (i, item) in items.iter().enumerate() {
        let nearest = index.knn_search::<EuclideanDistance>(item, 1);
        assert_eq!(nearest.len(), 1, "{name}: item {i} nearest count");
        assert_eq!(nearest[0], item, "{name}: item {i} is its own nearest");
    }
    for k in [0usize, 1, 3, items.len(), items.len() + 5] {
        let found = index.knn_search::<EuclideanDistance>(&items[0], k);
        assert_eq!(
            found.len(),
            k.min(items.len()),
            "{name}: knn with k={k} should return min(k, len) items"
        );
    }

    // A radius of zero finds the item, and a negative radius finds nothing.
    assert!(
        index
            .range_search::<EuclideanDistance>(&items[0], 0.0)
            .iter()
            .any(|found| *found == &items[0]),
        "{name}: a zero radius should still match the query point"
    );
    assert!(
        index
            .range_search::<EuclideanDistance>(&items[0], -1.0)
            .is_empty(),
        "{name}: a negative radius should match nothing"
    );

    // Deletion removes exactly one item and is reported honestly.
    let mut remaining = items.len();
    for (i, item) in items.iter().enumerate() {
        assert!(
            index.delete(item),
            "{name}: delete of item {i} should succeed"
        );
        remaining -= 1;
        assert_eq!(index.len(), remaining, "{name}: length after deleting {i}");
        assert!(
            !index.delete(item),
            "{name}: item {i} should not be deletable twice"
        );
    }
    assert!(index.is_empty(), "{name}: emptied index should be empty");
    assert!(index.range_search_bbox(everything).is_empty());

    // The index is reusable after being emptied, and `clear` empties it again.
    for item in &items {
        assert_eq!(SpatialIndex::insert(&mut index, item.clone()), Ok(true));
    }
    assert_eq!(index.len(), items.len(), "{name}: refilled length");
    index.clear();
    assert!(index.is_empty(), "{name}: clear should empty the index");
    assert_eq!(index.len(), 0);
    assert!(index.range_search_bbox(everything).is_empty());

    // Bulk insertion reports its count and agrees with the individual path.
    assert_eq!(
        SpatialIndex::insert_bulk(&mut index, items.clone()),
        Ok(items.len()),
        "{name}: insert_bulk should report every stored item"
    );
    assert_eq!(index.len(), items.len(), "{name}: length after insert_bulk");
    assert_eq!(index.range_search_bbox(everything).len(), items.len());
    for item in &items {
        assert!(
            index.contains(item),
            "{name}: bulk-inserted item should be present"
        );
    }
}

#[test]
fn quadtree_honors_the_contract() {
    let tree: Quadtree<u32> = Quadtree::new(&BOUNDARY_2D, 4).unwrap();
    check_contract(tree, points_2d(40), &BOUNDARY_2D, "Quadtree");
}

#[test]
fn octree_honors_the_contract() {
    let tree: Octree<u32> = Octree::new(&BOUNDARY_3D, 4).unwrap();
    check_contract(tree, points_3d(40), &BOUNDARY_3D, "Octree");
}

#[test]
fn kdtree_2d_honors_the_contract() {
    let tree: KdTree<Point2D<u32>> = KdTree::new();
    check_contract(tree, points_2d(40), &BOUNDARY_2D, "KdTree2D");
}

#[test]
fn kdtree_3d_honors_the_contract() {
    let tree: KdTree<Point3D<u32>> = KdTree::new();
    check_contract(tree, points_3d(40), &BOUNDARY_3D, "KdTree3D");
}

#[test]
fn rtree_2d_honors_the_contract() {
    let tree: RTree<Point2D<u32>> = RTree::new(4).unwrap();
    check_contract(tree, points_2d(40), &BOUNDARY_2D, "RTree2D");
}

#[test]
fn rtree_3d_honors_the_contract() {
    let tree: RTree<Point3D<u32>> = RTree::new(4).unwrap();
    check_contract(tree, points_3d(40), &BOUNDARY_3D, "RTree3D");
}

#[test]
fn rstar_tree_2d_honors_the_contract() {
    let tree: RStarTree<Point2D<u32>> = RStarTree::new(4).unwrap();
    check_contract(tree, points_2d(40), &BOUNDARY_2D, "RStarTree2D");
}

#[test]
fn rstar_tree_3d_honors_the_contract() {
    let tree: RStarTree<Point3D<u32>> = RStarTree::new(4).unwrap();
    check_contract(tree, points_3d(40), &BOUNDARY_3D, "RStarTree3D");
}

/// The bounded trees decline a point outside their boundary rather than erroring or panicking.
#[test]
fn bounded_trees_decline_out_of_bounds_points() {
    let mut quadtree: Quadtree<u32> = Quadtree::new(&BOUNDARY_2D, 4).unwrap();
    assert_eq!(
        SpatialIndex::insert(&mut quadtree, Point2D::new(1000.0, 1000.0, Some(1))),
        Ok(false)
    );
    assert!(quadtree.is_empty());
    assert_eq!(
        SpatialIndex::insert_bulk(
            &mut quadtree,
            vec![
                Point2D::new(10.0, 10.0, Some(1)),
                Point2D::new(1000.0, 1000.0, Some(2)),
            ]
        ),
        Ok(1),
        "insert_bulk counts only the points it stored"
    );
    assert_eq!(quadtree.len(), 1);

    let mut octree: Octree<u32> = Octree::new(&BOUNDARY_3D, 4).unwrap();
    assert_eq!(
        SpatialIndex::insert(&mut octree, Point3D::new(1000.0, 1000.0, 1000.0, Some(1))),
        Ok(false)
    );
    assert!(octree.is_empty());
}

/// The unbounded trees accept anything a bounded tree would refuse.
#[test]
fn unbounded_trees_accept_any_coordinates() {
    let far = Point2D::new(-1.0e9, 1.0e9, Some(1));

    let mut kdtree: KdTree<Point2D<u32>> = KdTree::new();
    assert_eq!(SpatialIndex::insert(&mut kdtree, far.clone()), Ok(true));
    assert!(kdtree.contains(&far));

    let mut rtree: RTree<Point2D<u32>> = RTree::new(4).unwrap();
    assert_eq!(SpatialIndex::insert(&mut rtree, far.clone()), Ok(true));
    assert!(rtree.contains(&far));

    let mut rstar: RStarTree<Point2D<u32>> = RStarTree::new(4).unwrap();
    assert_eq!(SpatialIndex::insert(&mut rstar, far.clone()), Ok(true));
    assert!(rstar.contains(&far));
}

/// A Kd-tree is the only index that can reject an item outright, and it must not be left changed.
#[test]
fn kdtree_rejects_a_mismatched_dimension_without_changing() {
    let mut tree: KdTree<Point2D<u32>> = KdTree::with_dimension(3);
    let point = Point2D::new(1.0, 2.0, Some(1));

    let outcome = SpatialIndex::insert(&mut tree, point.clone());
    assert!(
        matches!(
            outcome,
            Err(SpartError::DimensionMismatch {
                expected: 3,
                actual: 2
            })
        ),
        "expected a dimension mismatch, got {outcome:?}"
    );
    assert!(
        tree.is_empty(),
        "a rejected insert must leave the tree empty"
    );

    let outcome = SpatialIndex::insert_bulk(&mut tree, vec![point]);
    assert!(matches!(outcome, Err(SpartError::DimensionMismatch { .. })));
    assert!(
        tree.is_empty(),
        "a rejected batch must leave the tree untouched"
    );
}

/// Duplicates are stored independently, and one delete removes one copy.
#[test]
fn every_index_keeps_duplicates_separate() {
    fn check<I>(mut index: I, item: I::Item, everything: &I::Volume, name: &str)
    where
        I: SpatialIndex,
        I::Item: Clone + PartialEq + std::fmt::Debug,
    {
        for _ in 0..3 {
            assert_eq!(SpatialIndex::insert(&mut index, item.clone()), Ok(true));
        }
        assert_eq!(index.len(), 3, "{name}: three duplicates stored");
        assert_eq!(index.range_search_bbox(everything).len(), 3);

        assert!(index.delete(&item));
        assert_eq!(index.len(), 2, "{name}: one delete removes one copy");
        assert_eq!(index.range_search_bbox(everything).len(), 2);
        assert!(index.contains(&item), "{name}: copies remain");
    }

    let point = Point2D::new(10.0, 10.0, Some(1u32));
    check(
        Quadtree::<u32>::new(&BOUNDARY_2D, 4).unwrap(),
        point.clone(),
        &BOUNDARY_2D,
        "Quadtree",
    );
    check(
        KdTree::<Point2D<u32>>::new(),
        point.clone(),
        &BOUNDARY_2D,
        "KdTree2D",
    );
    check(
        RTree::<Point2D<u32>>::new(4).unwrap(),
        point.clone(),
        &BOUNDARY_2D,
        "RTree2D",
    );
    check(
        RStarTree::<Point2D<u32>>::new(4).unwrap(),
        point,
        &BOUNDARY_2D,
        "RStarTree2D",
    );

    // The 3D trees too: coincident points are what drive the octree into its depth-capped bucket
    // branch, which is the path that touches the subtree counts.
    let point = Point3D::new(10.0, 10.0, 10.0, Some(1u32));
    check(
        Octree::<u32>::new(&BOUNDARY_3D, 4).unwrap(),
        point.clone(),
        &BOUNDARY_3D,
        "Octree",
    );
    check(
        KdTree::<Point3D<u32>>::new(),
        point.clone(),
        &BOUNDARY_3D,
        "KdTree3D",
    );
    check(
        RTree::<Point3D<u32>>::new(4).unwrap(),
        point.clone(),
        &BOUNDARY_3D,
        "RTree3D",
    );
    check(
        RStarTree::<Point3D<u32>>::new(4).unwrap(),
        point,
        &BOUNDARY_3D,
        "RStarTree3D",
    );
}

/// Enough coincident points to push the bounded trees past their depth cap, where a node stops
/// subdividing and keeps points directly. The subtree counts have to stay exact through that.
#[test]
fn bounded_trees_count_correctly_past_the_depth_cap() {
    let mut quadtree: Quadtree<u32> = Quadtree::new(&BOUNDARY_2D, 1).unwrap();
    let points: Vec<_> = (0..30).map(|i| Point2D::new(10.0, 10.0, Some(i))).collect();
    for (i, point) in points.iter().enumerate() {
        assert_eq!(SpatialIndex::insert(&mut quadtree, point.clone()), Ok(true));
        assert_eq!(
            quadtree.len(),
            i + 1,
            "quadtree len after {} inserts",
            i + 1
        );
    }
    for (i, point) in points.iter().enumerate() {
        assert!(quadtree.delete(point));
        assert_eq!(quadtree.len(), points.len() - i - 1);
    }
    assert!(quadtree.is_empty());

    let mut octree: Octree<u32> = Octree::new(&BOUNDARY_3D, 1).unwrap();
    let points: Vec<_> = (0..30)
        .map(|i| Point3D::new(10.0, 10.0, 10.0, Some(i)))
        .collect();
    for (i, point) in points.iter().enumerate() {
        assert_eq!(SpatialIndex::insert(&mut octree, point.clone()), Ok(true));
        assert_eq!(octree.len(), i + 1, "octree len after {} inserts", i + 1);
    }
    for (i, point) in points.iter().enumerate() {
        assert!(octree.delete(point));
        assert_eq!(octree.len(), points.len() - i - 1);
    }
    assert!(octree.is_empty());
}

/// A query for zero neighbors must not walk the tree.
///
/// `KnnHeap::worst` reports the distance to beat, and when no neighbors are wanted that has to be
/// negative infinity so every prune test fails closed. Reporting infinity instead made each tree
/// traverse itself in full to return an empty vector.
#[test]
fn a_zero_neighbor_query_prunes_immediately() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    static CALLS: AtomicUsize = AtomicUsize::new(0);

    struct Counting;
    impl<T> spart::geometry::DistanceMetric<Point2D<T>> for Counting {
        fn distance_sq(a: &Point2D<T>, b: &Point2D<T>) -> f64 {
            CALLS.fetch_add(1, Ordering::Relaxed);
            <EuclideanDistance as spart::geometry::DistanceMetric<Point2D<T>>>::distance_sq(a, b)
        }
    }

    fn check<I>(mut index: I, points: Vec<Point2D<u32>>, name: &str)
    where
        I: SpatialIndex<Item = Point2D<u32>>,
    {
        for point in points {
            let _ = SpatialIndex::insert(&mut index, point);
        }
        CALLS.store(0, Ordering::Relaxed);
        let found = index.knn_search::<Counting>(&Point2D::new(1.0, 1.0, None), 0);
        let calls = CALLS.load(Ordering::Relaxed);
        assert!(found.is_empty(), "{name}: k=0 must return nothing");
        assert_eq!(
            calls, 0,
            "{name}: k=0 computed {calls} distances, so it walked the tree"
        );
    }

    let points = points_2d(500);
    check(
        Quadtree::<u32>::new(&BOUNDARY_2D, 4).unwrap(),
        points.clone(),
        "Quadtree",
    );
    check(KdTree::<Point2D<u32>>::new(), points.clone(), "KdTree");
    check(
        RTree::<Point2D<u32>>::new(4).unwrap(),
        points.clone(),
        "RTree",
    );
    check(
        RStarTree::<Point2D<u32>>::new(4).unwrap(),
        points,
        "RStarTree",
    );
}

/// `range_search_bbox` must mean containment on every tree.
///
/// The R-tree family prunes on bounding-volume intersection, so an inflated volume for a single
/// point made it report points just outside the query while the point-exact trees did not.
#[test]
fn box_queries_are_exact_on_every_tree() {
    // Just outside the query's low edge, by less than the old point extent of 1e-10.
    let outside = Point2D::new(10.0 - 5e-11, 5.0, Some(1u32));
    let inside = Point2D::new(10.0, 5.0, Some(2u32));
    let query = Rectangle {
        x: 10.0,
        y: 0.0,
        width: 10.0,
        height: 10.0,
    };
    assert!(!query.contains(&outside), "the probe point must be outside");
    assert!(query.contains(&inside), "the control point must be inside");

    fn check<I>(mut index: I, items: &[Point2D<u32>], query: &I::Volume, name: &str)
    where
        I: SpatialIndex<Item = Point2D<u32>>,
    {
        for item in items {
            let _ = SpatialIndex::insert(&mut index, item.clone());
        }
        let found = index.range_search_bbox(query);
        assert_eq!(
            found.len(),
            1,
            "{name}: a box query returned {} items, expected only the contained one",
            found.len()
        );
        assert_eq!(found[0].data, Some(2), "{name}: wrong item returned");
    }

    let items = [outside, inside];
    check(
        Quadtree::<u32>::new(&BOUNDARY_2D, 4).unwrap(),
        &items,
        &query,
        "Quadtree",
    );
    check(KdTree::<Point2D<u32>>::new(), &items, &query, "KdTree");
    check(
        RTree::<Point2D<u32>>::new(4).unwrap(),
        &items,
        &query,
        "RTree",
    );
    check(
        RStarTree::<Point2D<u32>>::new(4).unwrap(),
        &items,
        &query,
        "RStarTree",
    );
}
