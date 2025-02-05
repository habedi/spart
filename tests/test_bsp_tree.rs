use spart::bsp_tree::{BSPTree, BSPTreeObject, Point2DBSP, Point3DBSP};
use spart::geometry::{Point2D, Point3D};
mod utils;
use tracing::{debug, info};
use utils::*;

#[test]
fn test_bsptree_2d() {
    info!("Starting test_bsptree_2d");

    let mut tree: BSPTree<Point2DBSP<&str>> = BSPTree::new(CAPACITY);

    let points: Vec<Point2DBSP<&str>> = common_points_2d()
        .into_iter()
        .map(|pt| Point2DBSP { point: pt })
        .collect();
    for pt in &points {
        tree.insert(pt.clone());
        debug!("Inserted 2D BSPTree point: {:?}", pt);
    }
    info!("Finished inserting {} points", points.len());

    let target = Point2DBSP {
        point: Point2D {
            x: target_point_2d().x,
            y: target_point_2d().y,
            data: Some("Target"),
        },
    };
    info!("Performing 2D BSPTree kNN search for target: {:?}", target);
    let knn_results = tree.knn_search(&target.mbr(), KNN_COUNT);
    info!(
        "BSPTree 2D kNN search returned {} results",
        knn_results.len()
    );
    assert_eq!(
        knn_results.len(),
        KNN_COUNT,
        "Expected {} nearest neighbors (BSPTree 2D), got {}",
        KNN_COUNT,
        knn_results.len()
    );

    let rect = query_rect();
    info!(
        "Performing 2D BSPTree range search with query rectangle: {:?}",
        rect
    );
    let range_results = tree.range_search(&rect);
    info!(
        "BSPTree 2D range search returned {} results",
        range_results.len()
    );
    for pt in &range_results {
        let p = &pt.point;
        debug!("BSPTree 2D range result: {:?}", p);
        assert!(
            p.x >= rect.x
                && p.x <= rect.x + rect.width
                && p.y >= rect.y
                && p.y <= rect.y + rect.height,
            "BSPTree 2D point {:?} is outside the query rectangle {:?}",
            p,
            rect
        );
    }
    assert!(
        range_results.len() >= 5,
        "Expected at least 5 points in BSPTree 2D range, got {}",
        range_results.len()
    );

    let delete_point = Point2DBSP {
        point: Point2D::new(21.0, 21.0, Some("F")),
    };
    info!("Deleting BSPTree 2D point {:?}", delete_point);
    let deleted = tree.delete(&delete_point);
    info!("Deletion result: {}", deleted);
    assert!(deleted, "Expected deletion in BSPTree 2D to succeed");
    assert!(
        !tree.delete(&delete_point),
        "Deleting non-existent BSPTree 2D point should return false"
    );

    let knn_after = tree.knn_search(&target.mbr(), KNN_COUNT);
    for pt in &knn_after {
        debug!("BSPTree 2D kNN after deletion: {:?}", pt);
        assert_ne!(
            pt.point.data,
            Some("F"),
            "Deleted BSPTree 2D point still returned in kNN search"
        );
    }

    info!("test_bsptree_2d completed successfully");
}

#[test]
fn test_bsptree_3d() {
    info!("Starting test_bsptree_3d");

    let mut tree: BSPTree<Point3DBSP<&str>> = BSPTree::new(CAPACITY);

    let points: Vec<Point3DBSP<&str>> = common_points_3d()
        .into_iter()
        .map(|pt| Point3DBSP { point: pt })
        .collect();
    for pt in &points {
        tree.insert(pt.clone());
        debug!("Inserted 3D BSPTree point: {:?}", pt);
    }
    info!("Finished inserting {} points", points.len());

    let target = Point3DBSP {
        point: Point3D {
            x: target_point_3d().x,
            y: target_point_3d().y,
            z: target_point_3d().z,
            data: Some("Target"),
        },
    };
    info!("Performing 3D BSPTree kNN search for target: {:?}", target);
    let knn_results = tree.knn_search(&target.mbr(), KNN_COUNT);
    info!(
        "BSPTree 3D kNN search returned {} results",
        knn_results.len()
    );
    assert_eq!(
        knn_results.len(),
        KNN_COUNT,
        "Expected {} nearest neighbors (BSPTree 3D), got {}",
        KNN_COUNT,
        knn_results.len()
    );

    let cube = query_cube();
    info!(
        "Performing 3D BSPTree range search with query cube: {:?}",
        cube
    );
    let range_results = tree.range_search(&cube);
    info!(
        "BSPTree 3D range search returned {} results",
        range_results.len()
    );
    for pt in &range_results {
        let p = &pt.point;
        debug!("BSPTree 3D range result: {:?}", p);
        assert!(
            p.x >= cube.x
                && p.x <= cube.x + cube.width
                && p.y >= cube.y
                && p.y <= cube.y + cube.height
                && p.z >= cube.z
                && p.z <= cube.z + cube.depth,
            "BSPTree 3D point {:?} is outside the query cube {:?}",
            p,
            cube
        );
    }
    assert!(
        range_results.len() >= 5,
        "Expected at least 5 points in BSPTree 3D range, got {}",
        range_results.len()
    );

    let delete_point = Point3DBSP {
        point: Point3D::new(21.0, 21.0, 21.0, Some("F")),
    };
    info!("Deleting BSPTree 3D point {:?}", delete_point);
    let deleted = tree.delete(&delete_point);
    info!("Deletion result: {}", deleted);
    assert!(deleted, "Expected deletion in BSPTree 3D to succeed");
    assert!(
        !tree.delete(&delete_point),
        "Deleting non-existent BSPTree 3D point should return false"
    );

    let knn_after = tree.knn_search(&target.mbr(), KNN_COUNT);
    for pt in &knn_after {
        debug!("BSPTree 3D kNN after deletion: {:?}", pt);
        assert_ne!(
            pt.point.data,
            Some("F"),
            "Deleted BSPTree 3D point still returned in kNN search"
        );
    }

    info!("test_bsptree_3d completed successfully");
}
