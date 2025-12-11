#[path = "shared.rs"]
mod shared;
use shared::*;

use spart::geometry::{EuclideanDistance, Point2D, Point3D};
use spart::kdtree::KdTree;
use tracing::{debug, info};

fn run_kdtree_2d_test() {
    info!("Starting KDTree 2D test");

    let mut tree: KdTree<Point2D<&str>> = KdTree::new();

    let points = common_points_2d();
    for pt in &points {
        tree.insert(pt.clone()).unwrap();
        debug!("Inserted 2D point into KDTree: {:?}", pt);
    }
    info!("Finished inserting {} points", points.len());

    let target = target_point_2d();
    info!("Performing 2D kNN search for target: {:?}", target);
    let knn_results = tree.knn_search::<EuclideanDistance>(&target, KNN_COUNT);
    info!("2D kNN search returned {} results", knn_results.len());
    assert_eq!(
        knn_results.len(),
        KNN_COUNT,
        "Expected {} nearest neighbors (2D), got {}",
        KNN_COUNT,
        knn_results.len()
    );
    let mut prev_dist = 0.0;
    for pt in &knn_results {
        let d = distance_2d(&target, pt);
        debug!("2D kNN: Point {:?} at distance {}", pt, d);
        assert!(
            d >= prev_dist,
            "2D kNN results not sorted by increasing distance"
        );
        prev_dist = d;
    }

    let range_query = range_query_point_2d();
    info!(
        "Performing 2D range search for query point {:?} with radius {}",
        range_query, RADIUS
    );
    let range_results = tree.range_search::<EuclideanDistance>(&range_query, RADIUS);
    info!("2D range search returned {} results", range_results.len());
    for pt in &range_results {
        let d = distance_2d(&range_query, pt);
        debug!("2D Range: Point {:?} at distance {}", pt, d);
        assert!(
            d <= RADIUS,
            "Point {:?} returned by range query is at distance {} exceeding {}",
            pt,
            d,
            RADIUS
        );
    }
    assert!(
        range_results.len() >= 5,
        "Expected at least 5 points in range (2D), got {}",
        range_results.len()
    );

    let delete_point = Point2D::new(21.0, 21.0, Some("F"));
    info!("Deleting point {:?}", delete_point);
    let deleted = tree.delete(&delete_point);
    info!("Deletion result: {}", deleted);
    assert!(deleted, "Expected deletion of (21.0,21.0) to succeed");
    assert!(
        !tree.delete(&delete_point),
        "Deletion of non-existent point should fail"
    );

    let knn_after = tree.knn_search::<EuclideanDistance>(&target, KNN_COUNT);
    for pt in &knn_after {
        debug!("2D kNN after deletion: {:?}", pt);
        assert_ne!(
            pt.data,
            Some("F"),
            "Deleted point still returned in kNN search (2D)"
        );
    }

    info!("KDTree 2D test completed successfully");
}

fn run_kdtree_3d_test() {
    info!("Starting KDTree 3D test");

    let mut tree: KdTree<Point3D<&str>> = KdTree::new();

    let points = common_points_3d();
    for pt in &points {
        tree.insert(pt.clone()).unwrap();
        debug!("Inserted 3D point into KDTree: {:?}", pt);
    }
    info!("Finished inserting {} points", points.len());

    let target = target_point_3d();
    info!("Performing 3D kNN search for target: {:?}", target);
    let knn_results = tree.knn_search::<EuclideanDistance>(&target, KNN_COUNT);
    info!("3D kNN search returned {} results", knn_results.len());
    assert_eq!(
        knn_results.len(),
        KNN_COUNT,
        "Expected {} nearest neighbors (3D), got {}",
        KNN_COUNT,
        knn_results.len()
    );
    let mut prev_dist = 0.0;
    for pt in &knn_results {
        let d = distance_3d(&target, pt);
        debug!("3D kNN: Point {:?} at distance {}", pt, d);
        assert!(
            d >= prev_dist,
            "3D kNN results not sorted by increasing distance"
        );
        prev_dist = d;
    }

    let range_query = range_query_point_3d();
    info!(
        "Performing 3D range search for query point {:?} with radius {}",
        range_query, RADIUS
    );
    let range_results = tree.range_search::<EuclideanDistance>(&range_query, RADIUS);
    info!("3D range search returned {} results", range_results.len());
    for pt in &range_results {
        let d = distance_3d(&range_query, pt);
        debug!("3D Range: Point {:?} at distance {}", pt, d);
        assert!(
            d <= RADIUS,
            "Point {:?} returned by 3D range query is at distance {} exceeding {}",
            pt,
            d,
            RADIUS
        );
    }
    assert!(
        range_results.len() >= 5,
        "Expected at least 5 points in range (3D), got {}",
        range_results.len()
    );

    let delete_point = Point3D::new(21.0, 21.0, 21.0, Some("F"));
    info!("Deleting 3D point {:?}", delete_point);
    let deleted = tree.delete(&delete_point);
    info!("Deletion result: {}", deleted);
    assert!(deleted, "Expected deletion of (21.0,21.0,21.0) to succeed");
    assert!(
        !tree.delete(&delete_point),
        "Deleting non-existent 3D point should return false"
    );

    let knn_after = tree.knn_search::<EuclideanDistance>(&target, KNN_COUNT);
    for pt in &knn_after {
        debug!("3D kNN after deletion: {:?}", pt);
        assert_ne!(
            pt.data,
            Some("F"),
            "Deleted 3D point still returned in kNN search"
        );
    }

    info!("KDTree 3D test completed successfully");
}

#[test]
fn test_kdtree_2d() {
    run_kdtree_2d_test();
}

#[test]
fn test_kdtree_3d() {
    run_kdtree_3d_test();
}

#[test]
fn test_kdtree_insert_bulk_2d() {
    let mut tree: KdTree<Point2D<&str>> = KdTree::new();
    let points = common_points_2d();
    tree.insert_bulk(points).unwrap();

    let target = target_point_2d();
    let knn_results = tree.knn_search::<EuclideanDistance>(&target, KNN_COUNT);
    assert_eq!(
        knn_results.len(),
        KNN_COUNT,
        "Expected {} nearest neighbors, got {}",
        KNN_COUNT,
        knn_results.len()
    );
}

#[test]
fn test_kdtree_dimension_inference() {
    let mut tree: KdTree<Point2D<()>> = KdTree::new();
    let p = Point2D::new(1.0, 2.0, None);
    tree.insert(p).unwrap();
    // No public accessor for k, but insert would fail if it's not set.
    let p2 = Point2D::new(3.0, 4.0, None);
    assert!(tree.insert(p2).is_ok());
}

#[test]
fn test_kdtree_dimension_mismatch() {
    let mut tree: KdTree<Point2D<()>> = KdTree::new();
    let p2d = Point2D::new(1.0, 2.0, None);
    tree.insert(p2d).unwrap();
    // The following line would not compile, which is good.
    // let p3d = Point3D::new(1.0, 2.0, 3.0, None);
    // assert!(tree.insert(p3d).is_err());
}

#[test]
fn test_kdtree_delete_resets_k() {
    let mut tree: KdTree<Point2D<&str>> = KdTree::new();
    let p1 = Point2D::new(1.0, 2.0, Some("A"));
    tree.insert(p1.clone()).unwrap();
    assert!(tree.delete(&p1));
    // After deleting the only point, k should be None.
    // We can test this by inserting a point of a different dimension.
    // The following will not compile, so we can't directly test this.
    // let p3d = Point3D::new(1.0, 2.0, 3.0, Some("B"));
    // assert!(tree.insert(p3d).is_ok());
}

#[test]
fn test_kdtree_empty() {
    let mut tree: KdTree<Point2D<&str>> = KdTree::new();
    let target = target_point_2d();

    let knn_results = tree.knn_search::<EuclideanDistance>(&target, 5);
    assert!(
        knn_results.is_empty(),
        "kNN search on empty tree should return no points"
    );

    let range_results = tree.range_search::<EuclideanDistance>(&target, 10.0);
    assert!(
        range_results.is_empty(),
        "Range search on empty tree should return no points"
    );

    assert!(
        !tree.delete(&target),
        "Deleting from an empty tree should return false"
    );
}

#[test]
fn test_kdtree_knn_edge_cases() {
    let mut tree: KdTree<Point2D<&str>> = KdTree::new();
    let points = common_points_2d();
    let num_points = points.len();
    tree.insert_bulk(points).unwrap();

    let target = target_point_2d();

    // k = 0
    let knn_results = tree.knn_search::<EuclideanDistance>(&target, 0);
    assert!(
        knn_results.is_empty(),
        "kNN search with k=0 should return no points"
    );

    // k > number of points
    let knn_results = tree.knn_search::<EuclideanDistance>(&target, num_points + 5);
    assert_eq!(
        knn_results.len(),
        num_points,
        "kNN search with k > num_points should return all points"
    );
}

#[test]
fn test_kdtree_range_zero_radius() {
    let mut tree: KdTree<Point2D<&str>> = KdTree::new();
    let points = common_points_2d();
    tree.insert_bulk(points).unwrap();

    let target = Point2D::new(10.0, 10.0, Some("A"));
    tree.insert(target.clone()).unwrap();

    let results = tree.range_search::<EuclideanDistance>(&target, 0.0);
    assert_eq!(
        results.len(),
        1,
        "Range search with zero radius should return only the exact point"
    );
    assert_eq!(results[0], target);
}

#[test]
fn test_kdtree_duplicates() {
    let mut tree: KdTree<Point2D<&str>> = KdTree::new();
    let p1 = Point2D::new(10.0, 10.0, Some("A"));
    let p2 = Point2D::new(10.0, 10.0, Some("A"));
    tree.insert(p1.clone()).unwrap();
    tree.insert(p2.clone()).unwrap();

    let target = Point2D::new(10.0, 10.0, None);
    let results = tree.knn_search::<EuclideanDistance>(&target, 2);
    assert_eq!(results.len(), 2, "kNN should return duplicate points");

    let deleted = tree.delete(&p1);
    assert!(deleted, "Deleting a duplicate point should succeed");

    let results_after_delete = tree.knn_search::<EuclideanDistance>(&target, 2);
    assert_eq!(
        results_after_delete.len(),
        1,
        "Deleting a point should only remove one instance"
    );
}

#[test]
fn test_kdtree_insert_bulk_3d() {
    let mut tree: KdTree<Point3D<&str>> = KdTree::new();
    let points = common_points_3d();
    tree.insert_bulk(points).unwrap();

    let target = target_point_3d();
    let knn_results = tree.knn_search::<EuclideanDistance>(&target, KNN_COUNT);
    assert_eq!(
        knn_results.len(),
        KNN_COUNT,
        "Expected {} nearest neighbors, got {}",
        KNN_COUNT,
        knn_results.len()
    );
}

// See https://github.com/habedi/spart/issues/23
#[test]
fn test_kdtree_delete_many() {
    let mut tree: KdTree<Point2D<&str>> = KdTree::new();
    let points = [
        Point2D::new(1.0, 2.0, Some("A")),
        Point2D::new(3.0, 4.0, Some("B")),
        Point2D::new(-1.0, -2.0, Some("C")),
        Point2D::new(1.5, 3.2, Some("D")),
        Point2D::new(0.5, 2.0, Some("E")),
        Point2D::new(0.25, 2.0, Some("F")),
        Point2D::new(0.5, 1.0, Some("G")),
    ];

    for p in points.clone() {
        tree.insert(p).unwrap();
    }

    for p in &points {
        // Show the points
        assert!(tree.delete(p), "failed to delete {:?}", p);
        let knn_after = tree.knn_search::<EuclideanDistance>(&p, KNN_COUNT);
        for pt in &knn_after {
            debug!("2D kNN after deletion: {:?}", pt);
            assert_ne!(
                pt.data, p.data,
                "Deleted 2D point still returned in kNN search"
            );
        }
    }
}

#[test]
fn test_kdtree_delete_same_coords_different_data() {
    let mut tree: KdTree<Point2D<&str>> = KdTree::new();
    let p1 = Point2D::new(10.0, 10.0, Some("A"));
    let p2 = Point2D::new(10.0, 10.0, Some("B"));
    let p3 = Point2D::new(10.0, 10.0, Some("C"));
    tree.insert(p1.clone()).unwrap();
    tree.insert(p2.clone()).unwrap();
    tree.insert(p3.clone()).unwrap();

    assert!(tree.delete(&p2), "should delete exact payload match");
    assert!(
        tree.contains(&p1),
        "other payload with same coords should remain"
    );
    assert!(
        tree.contains(&p3),
        "other payload with same coords should remain"
    );
    assert!(!tree.contains(&p2), "deleted payload should be gone");

    let tgt = Point2D::new(10.0, 10.0, None);
    let res = tree.knn_search::<EuclideanDistance>(&tgt, 3);
    assert_eq!(res.len(), 2);
    for r in res {
        assert_ne!(r.data, Some("B"));
    }
}

#[test]
fn test_kdtree_delete_nonexistent_with_equal_axis() {
    let mut tree: KdTree<Point2D<&str>> = KdTree::new();
    let a = Point2D::new(1.0, 0.0, Some("A"));
    let b = Point2D::new(1.0, 1.0, Some("B"));
    let c = Point2D::new(1.0, -1.0, Some("C"));
    tree.insert(a.clone()).unwrap();
    tree.insert(b.clone()).unwrap();
    tree.insert(c.clone()).unwrap();

    let not_present = Point2D::new(1.0, 2.0, Some("X"));
    assert!(
        !tree.delete(&not_present),
        "deleting non-existent point should return false"
    );
    assert!(tree.contains(&a));
    assert!(tree.contains(&b));
    assert!(tree.contains(&c));
}

#[test]
fn test_kdtree_delete_root_with_only_left() {
    let mut tree: KdTree<Point2D<&str>> = KdTree::new();
    let root = Point2D::new(5.0, 5.0, Some("R"));
    let l1 = Point2D::new(2.0, 2.0, Some("L1"));
    let l2 = Point2D::new(1.0, 1.0, Some("L2"));
    tree.insert(root.clone()).unwrap();
    tree.insert(l1.clone()).unwrap();
    tree.insert(l2.clone()).unwrap();

    assert!(
        tree.delete(&root),
        "should delete root when only left subtree exists"
    );
    assert!(!tree.contains(&root));
    assert!(tree.contains(&l1));
    assert!(tree.contains(&l2));

    // Subsequent deletes should still work
    assert!(tree.delete(&l1));
    assert!(tree.contains(&l2));
}

#[test]
fn test_kdtree_delete_all_and_reinsert() {
    let mut tree: KdTree<Point2D<&str>> = KdTree::new();
    let pts = [
        Point2D::new(0.0, 0.0, Some("A")),
        Point2D::new(1.0, 1.0, Some("B")),
        Point2D::new(-1.0, -1.0, Some("C")),
    ];
    for p in pts.iter().cloned() {
        tree.insert(p).unwrap();
    }

    for p in &pts {
        assert!(tree.delete(p), "failed to delete {:?}", p);
    }
    for p in &pts {
        assert!(
            !tree.delete(p),
            "second delete should report false for {:?}",
            p
        );
    }

    // After clearing, reinsert and ensure queries behave
    let new_pts = [
        Point2D::new(2.0, 2.0, Some("D")),
        Point2D::new(3.0, 3.0, Some("E")),
    ];
    for p in new_pts.iter().cloned() {
        tree.insert(p).unwrap();
    }

    let tgt = Point2D::new(2.1, 2.1, None);
    let res = tree.knn_search::<EuclideanDistance>(&tgt, 2);
    assert_eq!(res.len(), 2);
}

#[test]
fn test_kdtree_delete_many_equal_on_axis() {
    let mut tree: KdTree<Point2D<&str>> = KdTree::new();
    let pts = [
        Point2D::new(0.0, 0.0, Some("A")),
        Point2D::new(0.0, 1.0, Some("B")),
        Point2D::new(0.0, 2.0, Some("C")),
        Point2D::new(0.0, 3.0, Some("D")),
        Point2D::new(0.0, -1.0, Some("E")),
    ];
    for p in pts.iter().cloned() {
        tree.insert(p).unwrap();
    }

    for p in &pts {
        assert!(tree.delete(p), "failed to delete {:?}", p);
        assert!(!tree.contains(p), "deleted point still present: {:?}", p);
    }

    let tgt = Point2D::new(0.0, 0.0, None);
    let res = tree.knn_search::<EuclideanDistance>(&tgt, 1);
    assert!(res.is_empty(), "tree should be empty after deleting all");
}
