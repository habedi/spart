//! Property-based tests for KdTree

use proptest::prelude::*;
use spart::geometry::{DistanceMetric, EuclideanDistance, Point2D, Point3D};
use spart::kdtree::KdTree;
use std::cmp::Ordering;

prop_compose! {
    fn arb_point_2d()(x in -1000.0..1000.0, y in -1000.0..1000.0) -> (f64, f64) {
        (x, y)
    }
}

prop_compose! {
    fn arb_point_3d()(x in -1000.0..1000.0, y in -1000.0..1000.0, z in -1000.0..1000.0) -> (f64, f64, f64) {
        (x, y, z)
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

fn brute_knn_distances_2d(points: &[Point2D<i32>], target: &Point2D<i32>, k: usize) -> Vec<f64> {
    let mut distances: Vec<f64> = points
        .iter()
        .map(|p| EuclideanDistance::distance_sq(target, p))
        .collect();
    distances.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    distances.into_iter().take(k).collect()
}

fn brute_knn_distances_3d(points: &[Point3D<i32>], target: &Point3D<i32>, k: usize) -> Vec<f64> {
    let mut distances: Vec<f64> = points
        .iter()
        .map(|p| EuclideanDistance::distance_sq(target, p))
        .collect();
    distances.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
    distances.into_iter().take(k).collect()
}

proptest! {
    #[test]
    fn test_kdtree_knn_matches_bruteforce_2d(
        coords in prop::collection::vec(arb_point_2d(), 1..40),
        target_coords in arb_point_2d(),
        k in 1usize..15
    ) {
        let points = points_2d_from_coords(&coords);
        let target = Point2D::new(target_coords.0, target_coords.1, Some(-1));
        let mut tree = KdTree::new();
        tree.insert_bulk(points.clone()).unwrap();

        let k = k.min(points.len());
        let knn = tree.knn_search::<EuclideanDistance>(&target, k);
        let brute_distances = brute_knn_distances_2d(&points, &target, k);
        let knn_distances: Vec<f64> = knn
            .iter()
            .map(|p| EuclideanDistance::distance_sq(&target, p))
            .collect();

        prop_assert_eq!(knn_distances.len(), k);
        for i in 1..knn_distances.len() {
            prop_assert!(knn_distances[i - 1] <= knn_distances[i] + 1e-9);
        }
        for (got, expected) in knn_distances.iter().zip(brute_distances.iter()) {
            prop_assert!((got - expected).abs() <= 1e-9);
        }
    }

    #[test]
    fn test_kdtree_range_matches_bruteforce_2d(
        coords in prop::collection::vec(arb_point_2d(), 1..40),
        target_coords in arb_point_2d(),
        radius in 0.0..150.0
    ) {
        let points = points_2d_from_coords(&coords);
        let target = Point2D::new(target_coords.0, target_coords.1, Some(-1));
        let mut tree = KdTree::new();
        tree.insert_bulk(points.clone()).unwrap();

        let results = tree.range_search::<EuclideanDistance>(&target, radius);
        let mut expected_ids: Vec<i32> = points
            .iter()
            .filter(|p| EuclideanDistance::distance_sq(&target, p) <= radius * radius)
            .map(|p| p.data.expect("data assigned"))
            .collect();
        let mut result_ids: Vec<i32> = results
            .iter()
            .map(|p| p.data.expect("data assigned"))
            .collect();
        expected_ids.sort();
        result_ids.sort();

        prop_assert_eq!(result_ids, expected_ids);
    }

    #[test]
    fn test_kdtree_knn_matches_bruteforce_3d(
        coords in prop::collection::vec(arb_point_3d(), 1..30),
        target_coords in arb_point_3d(),
        k in 1usize..10
    ) {
        let points = points_3d_from_coords(&coords);
        let target = Point3D::new(target_coords.0, target_coords.1, target_coords.2, Some(-1));
        let mut tree = KdTree::new();
        tree.insert_bulk(points.clone()).unwrap();

        let k = k.min(points.len());
        let knn = tree.knn_search::<EuclideanDistance>(&target, k);
        let brute_distances = brute_knn_distances_3d(&points, &target, k);
        let knn_distances: Vec<f64> = knn
            .iter()
            .map(|p| EuclideanDistance::distance_sq(&target, p))
            .collect();

        prop_assert_eq!(knn_distances.len(), k);
        for i in 1..knn_distances.len() {
            prop_assert!(knn_distances[i - 1] <= knn_distances[i] + 1e-9);
        }
        for (got, expected) in knn_distances.iter().zip(brute_distances.iter()) {
            prop_assert!((got - expected).abs() <= 1e-9);
        }
    }

    #[test]
    fn test_kdtree_range_matches_bruteforce_3d(
        coords in prop::collection::vec(arb_point_3d(), 1..30),
        target_coords in arb_point_3d(),
        radius in 0.0..150.0
    ) {
        let points = points_3d_from_coords(&coords);
        let target = Point3D::new(target_coords.0, target_coords.1, target_coords.2, Some(-1));
        let mut tree = KdTree::new();
        tree.insert_bulk(points.clone()).unwrap();

        let results = tree.range_search::<EuclideanDistance>(&target, radius);
        let mut expected_ids: Vec<i32> = points
            .iter()
            .filter(|p| EuclideanDistance::distance_sq(&target, p) <= radius * radius)
            .map(|p| p.data.expect("data assigned"))
            .collect();
        let mut result_ids: Vec<i32> = results
            .iter()
            .map(|p| p.data.expect("data assigned"))
            .collect();
        expected_ids.sort();
        result_ids.sort();

        prop_assert_eq!(result_ids, expected_ids);
    }
}
