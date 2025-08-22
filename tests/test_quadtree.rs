#[path = "shared.rs"]
mod shared;
use shared::*;

use spart::geometry::EuclideanDistance;
use spart::quadtree::Quadtree;
use tracing::{debug, info};

fn run_quadtree_2d_test() {
    info!("Starting Quadtree 2D test");

    let boundary = BOUNDARY_RECT;
    let mut tree = Quadtree::new(&boundary, CAPACITY);
    info!("Created quadtree with boundary: {:?}", boundary);

    let points = common_points_2d();
    for pt in &points {
        tree.insert(pt.clone());
        debug!("Inserted point: {:?}", pt);
    }
    info!("Finished inserting {} points", points.len());

    // kNN search test
    let target = target_point_2d();
    info!("Performing kNN search for target: {:?}", target);
    let knn_results = tree.knn_search::<EuclideanDistance>(&target, KNN_COUNT);
    info!("kNN search returned {} results", knn_results.len());
    assert_eq!(
        knn_results.len(),
        KNN_COUNT,
        "Expected {} nearest neighbors, got {}",
        KNN_COUNT,
        knn_results.len()
    );
    let mut prev_dist = 0.0;
    for pt in &knn_results {
        let d = distance_2d(&target, pt);
        debug!("kNN: Point {:?} at distance {}", pt, d);
        assert!(
            d >= prev_dist,
            "kNN results not sorted by increasing distance"
        );
        prev_dist = d;
    }

    // Range search test
    let range_query = range_query_point_2d();
    info!(
        "Performing range search for query point {:?} with radius {}",
        range_query, RADIUS
    );
    let range_results = tree.range_search::<EuclideanDistance>(&range_query, RADIUS);
    info!("Range search returned {} points", range_results.len());
    for pt in &range_results {
        let d = distance_2d(&range_query, pt);
        debug!("Range: Point {:?} at distance {}", pt, d);
        assert!(
            d <= RADIUS,
            "Point {:?} is at distance {} which exceeds radius {}",
            pt,
            d,
            RADIUS
        );
    }
    assert!(
        range_results.len() >= 5,
        "Expected at least 5 points in range, got {}",
        range_results.len()
    );

    info!("Quadtree 2D test completed successfully");
}

#[test]
fn test_quadtree_2d() {
    run_quadtree_2d_test();
}
