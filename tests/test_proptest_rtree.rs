//! Property-based tests for RTree

use proptest::prelude::*;
use spart::geometry::{Cube, DistanceMetric, EuclideanDistance, Point2D, Point3D, Rectangle};
use spart::rtree::RTree;

prop_compose! {
    fn arb_point_2d()(x in -100.0..100.0, y in -100.0..100.0) -> (f64, f64) {
        (x, y)
    }
}

prop_compose! {
    fn arb_point_3d()(x in -100.0..100.0, y in -100.0..100.0, z in -100.0..100.0) -> (f64, f64, f64) {
        (x, y, z)
    }
}

prop_compose! {
    fn arb_rectangle()(x in -100.0..100.0, y in -100.0..100.0, width in 1.0..200.0, height in 1.0..200.0) -> Rectangle {
        Rectangle { x, y, width, height }
    }
}

prop_compose! {
    fn arb_cube()(x in -100.0..100.0, y in -100.0..100.0, z in -100.0..100.0, width in 1.0..200.0, height in 1.0..200.0, depth in 1.0..200.0) -> Cube {
        Cube { x, y, z, width, height, depth }
    }
}

fn points_2d_from_coords(coords: &[(f64, f64)]) -> Vec<Point2D<i32>> {
    coords
        .iter()
        .enumerate()
        .map(|(idx, (x, y))| Point2D::new(*x, *y, Some(idx as i32)))
        .collect()
}

fn points_3d_from_coords(coords: &[(f64, f64, f64)]) -> Vec<Point3D<i32>> {
    coords
        .iter()
        .enumerate()
        .map(|(idx, (x, y, z))| Point3D::new(*x, *y, *z, Some(idx as i32)))
        .collect()
}

proptest! {
    #[test]
    fn test_rtree_knn_finds_inserted_point_2d(
        coords in prop::collection::vec(arb_point_2d(), 1..30)
    ) {
        let points = points_2d_from_coords(&coords);
        let mut tree: RTree<Point2D<i32>> = RTree::new(4).unwrap();
        for point in &points {
            tree.insert(point.clone());
        }

        for point in &points {
            let results = tree.knn_search::<EuclideanDistance>(point, 1);
            prop_assert_eq!(results.len(), 1);
            let nearest = results[0];
            let dist = EuclideanDistance::distance_sq(point, nearest);
            prop_assert!(dist <= 1e-9);
        }
    }

    #[test]
    fn test_rtree_range_search_bbox_2d(
        coords in prop::collection::vec(arb_point_2d(), 1..40),
        query in arb_rectangle()
    ) {
        let points = points_2d_from_coords(&coords);
        let mut tree: RTree<Point2D<i32>> = RTree::new(4).unwrap();
        for point in &points {
            tree.insert(point.clone());
        }

        let results = tree.range_search_bbox(&query);
        for point in results {
            prop_assert!(query.contains(point));
        }
    }

    #[test]
    fn test_rtree_range_search_radius_2d(
        coords in prop::collection::vec(arb_point_2d(), 1..40),
        target_coords in arb_point_2d(),
        radius in 0.0..150.0
    ) {
        let points = points_2d_from_coords(&coords);
        let target = Point2D::new(target_coords.0, target_coords.1, Some(-1));
        let mut tree: RTree<Point2D<i32>> = RTree::new(4).unwrap();
        for point in &points {
            tree.insert(point.clone());
        }

        let results = tree.range_search::<EuclideanDistance>(&target, radius);
        for point in results {
            let dist = EuclideanDistance::distance_sq(&target, point);
            prop_assert!(dist <= radius * radius + 1e-9);
        }
    }

    #[test]
    fn test_rtree_delete_removes_point_2d(
        coords in prop::collection::vec(arb_point_2d(), 1..30)
    ) {
        let points = points_2d_from_coords(&coords);
        let mut tree: RTree<Point2D<i32>> = RTree::new(4).unwrap();
        for point in &points {
            tree.insert(point.clone());
        }

        let to_delete = points[0].clone();
        prop_assert!(tree.delete(&to_delete));
        let results = tree.range_search::<EuclideanDistance>(&to_delete, 0.0);
        prop_assert!(results.is_empty());
    }

    #[test]
    fn test_rtree_knn_finds_inserted_point_3d(
        coords in prop::collection::vec(arb_point_3d(), 1..25)
    ) {
        let points = points_3d_from_coords(&coords);
        let mut tree: RTree<Point3D<i32>> = RTree::new(4).unwrap();
        for point in &points {
            tree.insert(point.clone());
        }

        for point in &points {
            let results = tree.knn_search::<EuclideanDistance>(point, 1);
            prop_assert_eq!(results.len(), 1);
            let nearest = results[0];
            let dist = EuclideanDistance::distance_sq(point, nearest);
            prop_assert!(dist <= 1e-9);
        }
    }

    #[test]
    fn test_rtree_range_search_bbox_3d(
        coords in prop::collection::vec(arb_point_3d(), 1..30),
        query in arb_cube()
    ) {
        let points = points_3d_from_coords(&coords);
        let mut tree: RTree<Point3D<i32>> = RTree::new(4).unwrap();
        for point in &points {
            tree.insert(point.clone());
        }

        let results = tree.range_search_bbox(&query);
        for point in results {
            prop_assert!(query.contains(point));
        }
    }

    #[test]
    fn test_rtree_range_search_radius_3d(
        coords in prop::collection::vec(arb_point_3d(), 1..30),
        target_coords in arb_point_3d(),
        radius in 0.0..150.0
    ) {
        let points = points_3d_from_coords(&coords);
        let target = Point3D::new(target_coords.0, target_coords.1, target_coords.2, Some(-1));
        let mut tree: RTree<Point3D<i32>> = RTree::new(4).unwrap();
        for point in &points {
            tree.insert(point.clone());
        }

        let results = tree.range_search::<EuclideanDistance>(&target, radius);
        for point in results {
            let dist = EuclideanDistance::distance_sq(&target, point);
            prop_assert!(dist <= radius * radius + 1e-9);
        }
    }
}
