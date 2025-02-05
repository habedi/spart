use spart::geometry::{Point2D, Point3D};
use spart::r_tree::RTree;
mod utils;
use tracing::{debug, info};
use utils::*;

#[test]
fn test_rtree_2d() {
    info!("Starting test_rtree_2d");

    let mut tree: RTree<Point2D<&str>> = RTree::new(CAPACITY);

    let points = common_points_2d();
    for pt in &points {
        tree.insert(pt.clone());
        debug!("Inserted 2D point into RTree: {:?}", pt);
    }
    info!("Finished inserting {} points", points.len());

    let target = target_point_2d();
    info!("Performing 2D kNN search for target: {:?}", target);
    let knn_results = tree.knn_search(&target, KNN_COUNT);
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
        assert!(d >= prev_dist, "RTree 2D kNN results not sorted");
        prev_dist = d;
    }

    let rect = query_rect();
    info!(
        "Performing 2D range search with query rectangle: {:?}",
        rect
    );
    let range_results = tree.range_search(&rect);
    info!("2D range search returned {} results", range_results.len());
    for pt in &range_results {
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
        range_results.len() >= 5,
        "Expected at least 5 points in RTree 2D range, got {}",
        range_results.len()
    );

    let delete_point = Point2D::new(21.0, 21.0, Some("F"));
    info!("Deleting point {:?}", delete_point);
    let deleted = tree.delete(&delete_point);
    info!("Deletion result: {}", deleted);
    assert!(deleted, "Deletion in RTree 2D should succeed");
    assert!(
        !tree.delete(&delete_point),
        "Deleting non-existent point should return false"
    );

    let knn_after = tree.knn_search(&target, KNN_COUNT);
    for pt in &knn_after {
        debug!("RTree 2D kNN after deletion: {:?}", pt);
        assert_ne!(
            pt.data,
            Some("F"),
            "Deleted point still present in RTree 2D kNN search"
        );
    }

    info!("test_rtree_2d completed successfully");
}

#[test]
fn test_rtree_3d() {
    info!("Starting test_rtree_3d");

    let mut tree: RTree<Point3D<&str>> = RTree::new(CAPACITY);

    let points = common_points_3d();
    for pt in &points {
        tree.insert(pt.clone());
        debug!("Inserted 3D point into RTree: {:?}", pt);
    }
    info!("Finished inserting {} points", points.len());

    let target = target_point_3d();
    info!("Performing 3D kNN search for target: {:?}", target);
    let knn_results = tree.knn_search(&target, KNN_COUNT);
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
        assert!(d >= prev_dist, "RTree 3D kNN results not sorted");
        prev_dist = d;
    }

    let cube = query_cube();
    info!("Performing 3D range search with query cube: {:?}", cube);
    let range_results = tree.range_search(&cube);
    info!("3D range search returned {} results", range_results.len());
    for pt in &range_results {
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
        range_results.len() >= 5,
        "Expected at least 5 points in RTree 3D range, got {}",
        range_results.len()
    );

    let delete_point = Point3D::new(21.0, 21.0, 21.0, Some("F"));
    info!("Deleting 3D point {:?}", delete_point);
    let deleted = tree.delete(&delete_point);
    info!("Deletion result: {}", deleted);
    assert!(deleted, "Deletion in RTree 3D should succeed");
    assert!(
        !tree.delete(&delete_point),
        "Deleting non-existent 3D point should return false"
    );

    let knn_after = tree.knn_search(&target, KNN_COUNT);
    for pt in &knn_after {
        debug!("RTree 3D kNN after deletion: {:?}", pt);
        assert_ne!(
            pt.data,
            Some("F"),
            "Deleted point still returned in RTree 3D kNN search"
        );
    }

    info!("test_rtree_3d completed successfully");
}
