//! Property-based tests for Quadtree

use proptest::prelude::*;
use spart::geometry::{EuclideanDistance, Point2D, Rectangle};
use spart::quadtree::Quadtree;

prop_compose! {
    fn arb_point_2d_in_boundary()(x in 0.0..100.0, y in 0.0..100.0) -> Point2D<i32> {
        Point2D::new(x, y, Some(0))
    }
}

proptest! {
    #[test]
    fn test_quadtree_insert_point_can_be_found(
        points in prop::collection::vec(arb_point_2d_in_boundary(), 1..30)
    ) {
        let boundary = Rectangle { x: 0.0, y: 0.0, width: 100.0, height: 100.0 };
        let mut tree = Quadtree::new(&boundary, 4).unwrap();

        for point in &points {
            tree.insert(point.clone());
        }

        // Each inserted point should be findable via kNN
        for point in &points {
            let results = tree.knn_search::<EuclideanDistance>(point, 1);
            prop_assert!(!results.is_empty(), "Should find at least the point itself");

            if let Some(nearest) = results.first() {
                let dist = point.distance_sq(nearest);
                prop_assert!(dist < 1e-9, "Nearest point should be the inserted point itself");
            }
        }
    }

    #[test]
    fn test_quadtree_knn_sorted_by_distance(
        points in prop::collection::vec(arb_point_2d_in_boundary(), 5..30)
    ) {
        let boundary = Rectangle { x: 0.0, y: 0.0, width: 100.0, height: 100.0 };
        let mut tree = Quadtree::new(&boundary, 4).unwrap();

        for point in &points {
            tree.insert(point.clone());
        }

        let target = Point2D::new(50.0, 50.0, Some(0));
        let k = 5.min(points.len());
        let results = tree.knn_search::<EuclideanDistance>(&target, k);

        // Check that results are sorted by distance
        for i in 1..results.len() {
            let d1 = target.distance_sq(&results[i - 1]);
            let d2 = target.distance_sq(&results[i]);
            prop_assert!(d1 <= d2 + 1e-9, "kNN results should be sorted by distance");
        }
    }
}
