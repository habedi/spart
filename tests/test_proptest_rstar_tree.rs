//! Property-based tests for RStarTree

use proptest::prelude::*;
use spart::geometry::{Cube, EuclideanDistance, Point2D, Point3D, Rectangle};
use spart::rstar_tree::RStarTree;

prop_compose! {
    fn arb_point_2d_bounded()(x in 0.0..100.0, y in 0.0..100.0) -> Point2D<i32> {
        Point2D::new(x, y, Some(0))
    }
}

prop_compose! {
    fn arb_point_3d_bounded()(x in 0.0..100.0, y in 0.0..100.0, z in 0.0..100.0) -> Point3D<i32> {
        Point3D::new(x, y, z, Some(0))
    }
}

proptest! {
    #[test]
    fn test_rstar_tree_2d_insert_find(points in prop::collection::vec(arb_point_2d_bounded(), 1..30)) {
        let mut tree: RStarTree<Point2D<i32>> = RStarTree::new(4).unwrap();

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
    fn test_rstar_tree_2d_knn_sorted_by_distance(
        points in prop::collection::vec(arb_point_2d_bounded(), 5..30)
    ) {
        let mut tree: RStarTree<Point2D<i32>> = RStarTree::new(4).unwrap();

        for point in &points {
            tree.insert(point.clone());
        }

        let target = Point2D::new(50.0, 50.0, Some(0));
        let k = 5.min(points.len());
        let results = tree.knn_search::<EuclideanDistance>(&target, k);

        // Check that results are sorted by distance
        for i in 1..results.len() {
            let d1 = target.distance_sq(results[i - 1]);
            let d2 = target.distance_sq(results[i]);
            prop_assert!(d1 <= d2 + 1e-9, "kNN results should be sorted by distance");
        }
    }

    #[test]
    fn test_rstar_tree_2d_range_search_bbox(
        points in prop::collection::vec(arb_point_2d_bounded(), 5..30)
    ) {
        let mut tree: RStarTree<Point2D<i32>> = RStarTree::new(4).unwrap();

        for point in &points {
            tree.insert(point.clone());
        }

        let query_rect = Rectangle {
            x: 25.0,
            y: 25.0,
            width: 50.0,
            height: 50.0,
        };
        let results = tree.range_search_bbox(&query_rect);

        // All results should be within the query rectangle
        for point in &results {
            prop_assert!(query_rect.contains(point), "All points should be within query rectangle");
        }
    }

    #[test]
    fn test_rstar_tree_3d_insert_find(points in prop::collection::vec(arb_point_3d_bounded(), 1..30)) {
        let mut tree: RStarTree<Point3D<i32>> = RStarTree::new(4).unwrap();

        for point in &points {
            tree.insert(point.clone());
        }

        // Each inserted point should be findable
        for point in &points {
            let results = tree.knn_search::<EuclideanDistance>(point, 1);
            prop_assert!(!results.is_empty(), "Should find the point");

            if let Some(nearest) = results.first() {
                let dist = point.distance_sq(nearest);
                prop_assert!(dist < 1e-9, "Nearest point should be the inserted point itself");
            }
        }
    }

    #[test]
    fn test_rstar_tree_3d_range_search_bbox(
        points in prop::collection::vec(arb_point_3d_bounded(), 5..30)
    ) {
        let mut tree: RStarTree<Point3D<i32>> = RStarTree::new(4).unwrap();

        for point in &points {
            tree.insert(point.clone());
        }

        let query_cube = Cube {
            x: 25.0,
            y: 25.0,
            z: 25.0,
            width: 50.0,
            height: 50.0,
            depth: 50.0,
        };
        let results = tree.range_search_bbox(&query_cube);

        // All results should be within the query cube
        for point in &results {
            prop_assert!(query_cube.contains(point), "All points should be within query cube");
        }
    }
}
