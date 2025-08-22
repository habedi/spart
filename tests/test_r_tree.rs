#[path = "shared.rs"]
mod shared;
use shared::*;

use spart::geometry::{EuclideanDistance, Point2D, Point3D};
use spart::r_tree::RTree;
use tracing::{debug, info};

fn run_rtree_2d_test() {
    info!("Starting RTree 2D test");

    let mut tree: RTree<Point2D<&str>> = RTree::new(CAPACITY).unwrap();

    let points = common_points_2d();
    for pt in &points {
        tree.insert(pt.clone());
        debug!("Inserted 2D point into RTree: {:?}", pt);
    }
    info!("Finished inserting {} points", points.len());

    let target = target_point_2d();
    info!("Performing 2D kNN search for target: {:?}", target);
    let knn_results = tree.knn_search::<EuclideanDistance>(&target, KNN_COUNT);
    info!("2D kNN search returned {} results", knn_results.len());
    assert_eq!(
        knn_results.len(),
        KNN_COUNT,
        "Expected {} nearest neighbors (RTree 2D), got {}",
        KNN_COUNT,
        knn_results.len()
    );
    let mut prev_dist = 0.0;
    for pt in &knn_results {
        let d = distance_2d(&target, pt);
        debug!("RTree 2D kNN: Point {:?} at distance {}", pt, d);
        assert!(
            d >= prev_dist,
            "RTree 2D kNN results not sorted by increasing distance"
        );
        prev_dist = d;
    }

    let rect = query_rect();
    info!(
        "Performing 2D range search with query rectangle: {:?}",
        rect
    );
    let range_results_bbox = tree.range_search_bbox(&rect);
    info!(
        "2D range search (bbox) returned {} results",
        range_results_bbox.len()
    );
    for pt in &range_results_bbox {
        debug!("RTree 2D range result: {:?}", pt);
        assert!(
            pt.x >= rect.x
                && pt.x <= rect.x + rect.width
                && pt.y >= rect.y
                && pt.y <= rect.y + rect.height,
            "Point {:?} lies outside the query rectangle {:?}",
            pt,
            rect
        );
    }
    assert!(
        range_results_bbox.len() >= 5,
        "Expected at least 5 points in RTree 2D range, got {}",
        range_results_bbox.len()
    );

    // Perform a range search using a radius.
    let range_results_radius = tree.range_search::<EuclideanDistance>(&target, 5.0);
    info!(
        "2D range search (radius) returned {} results",
        range_results_radius.len()
    );
    for pt in &range_results_radius {
        debug!("RTree 2D range (radius) result: {:?}", pt);
        let d = distance_2d(&target, pt);
        assert!(d <= 5.0, "Point {:?} is outside the radius 5.0", pt);
    }

    let delete_point = Point2D::new(21.0, 21.0, Some("F"));
    info!("Deleting point {:?}", delete_point);
    let deleted = tree.delete(&delete_point);
    info!("Deletion result: {}", deleted);
    assert!(deleted, "Deletion in RTree 2D should succeed");
    assert!(
        !tree.delete(&delete_point),
        "Deleting non-existent point should return false"
    );

    let knn_after = tree.knn_search::<EuclideanDistance>(&target, KNN_COUNT);
    for pt in &knn_after {
        debug!("RTree 2D kNN after deletion: {:?}", pt);
        assert_ne!(
            pt.data,
            Some("F"),
            "Deleted point still present in RTree 2D kNN search"
        );
    }

    info!("RTree 2D test completed successfully");
}

fn run_rtree_3d_test() {
    info!("Starting RTree 3D test");

    let mut tree: RTree<Point3D<&str>> = RTree::new(CAPACITY).unwrap();

    let points = common_points_3d();
    for pt in &points {
        tree.insert(pt.clone());
        debug!("Inserted 3D point into RTree: {:?}", pt);
    }
    info!("Finished inserting {} points", points.len());

    let target = target_point_3d();
    info!("Performing 3D kNN search for target: {:?}", target);
    let knn_results = tree.knn_search::<EuclideanDistance>(&target, KNN_COUNT);
    info!("3D kNN search returned {} results", knn_results.len());
    assert_eq!(
        knn_results.len(),
        KNN_COUNT,
        "Expected {} nearest neighbors (RTree 3D), got {}",
        KNN_COUNT,
        knn_results.len()
    );
    let mut prev_dist = 0.0;
    for pt in &knn_results {
        let d = distance_3d(&target, pt);
        debug!("RTree 3D kNN: Point {:?} at distance {}", pt, d);
        assert!(
            d >= prev_dist,
            "RTree 3D kNN results not sorted by increasing distance"
        );
        prev_dist = d;
    }

    let cube = query_cube();
    info!("Performing 3D range search with query cube: {:?}", cube);
    let range_results_bbox = tree.range_search_bbox(&cube);
    info!(
        "3D range search (bbox) returned {} results",
        range_results_bbox.len()
    );
    for pt in &range_results_bbox {
        debug!("RTree 3D range result: {:?}", pt);
        assert!(
            pt.x >= cube.x
                && pt.x <= cube.x + cube.width
                && pt.y >= cube.y
                && pt.y <= cube.y + cube.height
                && pt.z >= cube.z
                && pt.z <= cube.z + cube.depth,
            "Point {:?} lies outside the query cube {:?}",
            pt,
            cube
        );
    }
    assert!(
        range_results_bbox.len() >= 5,
        "Expected at least 5 points in RTree 3D range, got {}",
        range_results_bbox.len()
    );

    // Perform a range search using a radius.
    let range_results_radius = tree.range_search::<EuclideanDistance>(&target, 5.0);
    info!(
        "3D range search (radius) returned {} results",
        range_results_radius.len()
    );
    for pt in &range_results_radius {
        debug!("RTree 3D range result: {:?}", pt);
        let d = distance_3d(&target, pt);
        assert!(d <= 5.0, "Point {:?} is outside the radius 5.0", pt);
    }

    let delete_point = Point3D::new(21.0, 21.0, 21.0, Some("F"));
    info!("Deleting 3D point {:?}", delete_point);
    let deleted = tree.delete(&delete_point);
    info!("Deletion result: {}", deleted);
    assert!(deleted, "Deletion in RTree 3D should succeed");
    assert!(
        !tree.delete(&delete_point),
        "Deleting non-existent 3D point should return false"
    );

    let knn_after = tree.knn_search::<EuclideanDistance>(&target, KNN_COUNT);
    for pt in &knn_after {
        debug!("RTree 3D kNN after deletion: {:?}", pt);
        assert_ne!(
            pt.data,
            Some("F"),
            "Deleted 3D point still present in RTree 3D kNN search"
        );
    }

    info!("RTree 3D test completed successfully");
}

#[test]
fn test_rtree_2d() {
    run_rtree_2d_test();
}

#[test]
fn test_rtree_3d() {
    run_rtree_3d_test();
}

#[test]
fn test_rtree_insert_bulk_2d() {
    let mut tree: RTree<Point2D<&str>> = RTree::new(CAPACITY).unwrap();
    let points = common_points_2d();
    tree.insert_bulk(points);

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
fn test_rtree_delete_underflow() {
    let mut tree: RTree<Point2D<i32>> = RTree::new(4).unwrap();
    let points: Vec<_> = (0..10)
        .map(|i| Point2D::new(i as f64, i as f64, Some(i)))
        .collect();

    for p in &points {
        tree.insert(p.clone());
    }

    // min_entries is (4 * 0.4).ceil() = 2.
    // Deleting points to trigger underflow.
    // Let's say we have a node with [p0, p1, p2, p3] and another with [p4, p5, p6, p7]
    // and a root.
    // If we delete p0, p1, p2, one of the nodes will underflow.
    assert!(tree.delete(&points[0]));
    assert!(tree.delete(&points[1]));
    assert!(tree.delete(&points[2]));

    // After deletions, check if the remaining points are still there.
    let all_points = tree.range_search_bbox(&spart::geometry::Rectangle {
        x: -1.0,
        y: -1.0,
        width: 12.0,
        height: 12.0,
    });
    assert_eq!(all_points.len(), 7);

    for i in 3..10 {
        assert!(tree.delete(&points[i]));
    }

    let all_points_after_all_deleted = tree.range_search_bbox(&spart::geometry::Rectangle {
        x: -1.0,
        y: -1.0,
        width: 12.0,
        height: 12.0,
    });
    assert!(all_points_after_all_deleted.is_empty());
}

#[test]
fn test_rtree_empty() {
    let mut tree: RTree<Point2D<&str>> = RTree::new(CAPACITY).unwrap();
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
fn test_rtree_knn_edge_cases() {
    let mut tree: RTree<Point2D<&str>> = RTree::new(CAPACITY).unwrap();
    let points = common_points_2d();
    tree.insert_bulk(points.clone());

    let target = target_point_2d();
    let num_points = points.len();

    let knn_results = tree.knn_search::<EuclideanDistance>(&target, 0);
    assert!(
        knn_results.is_empty(),
        "kNN search with k=0 should return no points"
    );

    let knn_results = tree.knn_search::<EuclideanDistance>(&target, num_points + 5);
    assert_eq!(
        knn_results.len(),
        num_points,
        "kNN search with k > num_points should return all points"
    );
}

#[test]
fn test_rtree_range_zero_radius() {
    let mut tree: RTree<Point2D<&str>> = RTree::new(CAPACITY).unwrap();
    let points = common_points_2d();
    tree.insert_bulk(points.clone());

    let target = points[0].clone();
    let results = tree.range_search::<EuclideanDistance>(&target, 0.0);
    assert_eq!(
        results.len(),
        1,
        "Range search with zero radius should return only the exact point"
    );
    assert_eq!(*results[0], target);
}

#[test]
fn test_rtree_duplicates() {
    let mut tree: RTree<Point2D<&str>> = RTree::new(CAPACITY).unwrap();
    let p1 = common_points_2d()[0].clone();
    let p2 = p1.clone();
    tree.insert(p1.clone());
    tree.insert(p2.clone());

    let target = p1.clone();
    let results = tree.knn_search::<EuclideanDistance>(&target, 2);
    assert_eq!(results.len(), 2, "kNN should return duplicate points");

    assert!(
        tree.delete(&p1),
        "Deleting a duplicate point should succeed"
    );

    let results_after_delete = tree.knn_search::<EuclideanDistance>(&target, 2);
    assert!(
        results_after_delete.is_empty(),
        "Deleting a point should remove all its duplicates"
    );
}

#[test]
fn test_rtree_insert_bulk_3d() {
    let mut tree: RTree<Point3D<&str>> = RTree::new(CAPACITY).unwrap();
    let points = common_points_3d();
    tree.insert_bulk(points);

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
