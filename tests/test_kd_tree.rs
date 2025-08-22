#[path = "shared.rs"]
mod shared;
use shared::*;

use spart::geometry::{EuclideanDistance, Point2D, Point3D};
use spart::kd_tree::KdTree;
use tracing::{debug, info};

fn run_kdtree_2d_test() {
    info!("Starting KDTree 2D test");

    let mut tree: KdTree<Point2D<&str>> = KdTree::new(2);

    let points = common_points_2d();
    for pt in &points {
        tree.insert(pt.clone());
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

    let mut tree: KdTree<Point3D<&str>> = KdTree::new(3);

    let points = common_points_3d();
    for pt in &points {
        tree.insert(pt.clone());
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
    let mut tree: KdTree<Point2D<&str>> = KdTree::new(2);
    let mut points = common_points_2d();
    tree.insert_bulk(&mut points);

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
fn test_kdtree_empty() {
    let mut tree: KdTree<Point2D<&str>> = KdTree::new(2);
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
    let mut tree: KdTree<Point2D<&str>> = KdTree::new(2);
    let mut points = common_points_2d();
    tree.insert_bulk(&mut points);

    let target = target_point_2d();
    let num_points = points.len();

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
    let mut tree: KdTree<Point2D<&str>> = KdTree::new(2);
    let mut points = common_points_2d();
    tree.insert_bulk(&mut points);

    let target = Point2D::new(10.0, 10.0, Some("A"));
    tree.insert(target.clone());

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
    let mut tree: KdTree<Point2D<&str>> = KdTree::new(2);
    let p1 = Point2D::new(10.0, 10.0, Some("A"));
    let p2 = Point2D::new(10.0, 10.0, Some("A"));
    tree.insert(p1.clone());
    tree.insert(p2.clone());

    let target = Point2D::new(10.0, 10.0, None);
    let results = tree.knn_search::<EuclideanDistance>(&target, 2);
    assert_eq!(results.len(), 2, "kNN should return duplicate points");

    let deleted = tree.delete(&p1);
    assert!(deleted, "Deleting a duplicate point should succeed");

    let results_after_delete = tree.knn_search::<EuclideanDistance>(&target, 2);
    assert!(
        results_after_delete.is_empty(),
        "Deleting a point should remove all its duplicates"
    );
}

#[test]
fn test_kdtree_insert_bulk_3d() {
    let mut tree: KdTree<Point3D<&str>> = KdTree::new(3);
    let mut points = common_points_3d();
    tree.insert_bulk(&mut points);

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
