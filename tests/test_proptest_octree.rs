//! Property-based tests for Octree

use proptest::prelude::*;
use spart::geometry::{Cube, EuclideanDistance, Point3D};
use spart::octree::Octree;

prop_compose! {
    fn arb_point_3d_in_boundary()(x in 0.0..100.0, y in 0.0..100.0, z in 0.0..100.0) -> Point3D<i32> {
        Point3D::new(x, y, z, Some(0))
    }
}

proptest! {
    #[test]
    fn test_octree_insert_point_can_be_found(
        points in prop::collection::vec(arb_point_3d_in_boundary(), 1..30)
    ) {
        let boundary = Cube { x: 0.0, y: 0.0, z: 0.0, width: 100.0, height: 100.0, depth: 100.0 };
        let mut tree = Octree::new(&boundary, 4).unwrap();

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
    fn test_octree_knn_sorted_by_distance(
        points in prop::collection::vec(arb_point_3d_in_boundary(), 5..30)
    ) {
        let boundary = Cube { x: 0.0, y: 0.0, z: 0.0, width: 100.0, height: 100.0, depth: 100.0 };
        let mut tree = Octree::new(&boundary, 4).unwrap();

        for point in &points {
            tree.insert(point.clone());
        }

        let target = Point3D::new(50.0, 50.0, 50.0, Some(0));
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
