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
