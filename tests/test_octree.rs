#[path = "shared.rs"]
mod shared;
use shared::*;

use spart::geometry::EuclideanDistance;
use spart::octree::Octree;
use tracing::{debug, info};

fn run_octree_3d_test() {
    info!("Starting Octree 3D test");

    // Create an octree with the shared cube boundary and capacity.
    let boundary = BOUNDARY_CUBE;
    let mut tree = Octree::new(&boundary, CAPACITY).unwrap();
    info!("Created octree with boundary: {:?}", boundary);

    // Insert common 3D points into the octree.
    let points = common_points_3d();
    for pt in &points {
        tree.insert(pt.clone());
        debug!("Inserted 3D point: {:?}", pt);
    }
    info!("Finished inserting {} points", points.len());

    // Perform a kNN search.
    let target = target_point_3d();
    info!("Performing 3D kNN search for target: {:?}", target);
    let knn_results = tree.knn_search::<EuclideanDistance>(&target, KNN_COUNT);
    info!("3D kNN search returned {} results", knn_results.len());
    assert_eq!(
        knn_results.len(),
        KNN_COUNT,
        "Expected {} nearest neighbors in 3D, got {}",
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

    // Perform a range search using a query point and radius.
    let range_query = range_query_point_3d();
    info!(
        "Performing 3D range search for query point {:?} with radius {}",
        range_query, RADIUS
    );
    let range_results = tree.range_search::<EuclideanDistance>(&range_query, RADIUS);
    info!("3D range search returned {} points", range_results.len());
    for pt in &range_results {
        let d = distance_3d(&range_query, pt);
        debug!("3D Range: Point {:?} at distance {}", pt, d);
        assert!(
            d <= RADIUS,
            "Point {:?} is at distance {} exceeding radius {}",
            pt,
            d,
            RADIUS
        );
    }
    assert!(
        range_results.len() >= 5,
        "Expected at least 5 points in 3D range search, got {}",
        range_results.len()
    );

    info!("Octree 3D test completed successfully");
}

#[test]
fn test_octree_3d() {
    run_octree_3d_test();
}

#[test]
fn test_octree_insert_bulk() {
    let boundary = BOUNDARY_CUBE;
    let mut tree = Octree::new(&boundary, CAPACITY).unwrap();
    let points = common_points_3d();
    tree.insert_bulk(&points);

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

#[test]
fn test_octree_delete() {
    let boundary = BOUNDARY_CUBE;
    let mut tree = Octree::new(&boundary, CAPACITY).unwrap();
    let points = common_points_3d();
    tree.insert_bulk(&points);

    let point_to_delete = points[3].clone();
    assert!(
        tree.delete(&point_to_delete),
        "Deletion of existing point should succeed"
    );

    let results = tree.knn_search::<EuclideanDistance>(&point_to_delete, 1);
    assert_ne!(
        results[0], point_to_delete,
        "Deleted point should not be found"
    );

    assert!(
        !tree.delete(&point_to_delete),
        "Deletion of non-existent point should fail"
    );
}

#[test]
fn test_octree_empty() {
    let boundary = BOUNDARY_CUBE;
    let mut tree: Octree<&str> = Octree::new(&boundary, CAPACITY).unwrap();
    let target = target_point_3d();

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
fn test_octree_knn_edge_cases() {
    let boundary = BOUNDARY_CUBE;
    let mut tree = Octree::new(&boundary, CAPACITY).unwrap();
    let points = common_points_3d();
    tree.insert_bulk(&points);

    let target = target_point_3d();
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
fn test_octree_range_zero_radius() {
    let boundary = BOUNDARY_CUBE;
    let mut tree = Octree::new(&boundary, CAPACITY).unwrap();
    let points = common_points_3d();
    tree.insert_bulk(&points);

    let target = points[0].clone();
    let results = tree.range_search::<EuclideanDistance>(&target, 0.0);
    assert_eq!(
        results.len(),
        1,
        "Range search with zero radius should return only the exact point"
    );
    assert_eq!(results[0], target);
}

#[test]
fn test_octree_duplicates() {
    let boundary = BOUNDARY_CUBE;
    let mut tree = Octree::new(&boundary, CAPACITY).unwrap();
    let p1 = common_points_3d()[0].clone();
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
