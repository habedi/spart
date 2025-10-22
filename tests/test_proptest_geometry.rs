//! Property-based tests for geometry primitives

use proptest::prelude::*;
use spart::geometry::{Cube, Point2D, Point3D, Rectangle};

// Generators for geometric primitives
prop_compose! {
    fn arb_point_2d()(x in -1000.0..1000.0, y in -1000.0..1000.0) -> Point2D<i32> {
        Point2D::new(x, y, Some(0))
    }
}

prop_compose! {
    fn arb_point_3d()(x in -1000.0..1000.0, y in -1000.0..1000.0, z in -1000.0..1000.0) -> Point3D<i32> {
        Point3D::new(x, y, z, Some(0))
    }
}

prop_compose! {
    fn arb_rectangle()(
        x in -500.0..500.0,
        y in -500.0..500.0,
        width in 1.0..500.0,
        height in 1.0..500.0
    ) -> Rectangle {
        Rectangle { x, y, width, height }
    }
}

prop_compose! {
    fn arb_cube()(
        x in -500.0..500.0,
        y in -500.0..500.0,
        z in -500.0..500.0,
        width in 1.0..500.0,
        height in 1.0..500.0,
        depth in 1.0..500.0
    ) -> Cube {
        Cube { x, y, z, width, height, depth }
    }
}

proptest! {
    #[test]
    fn test_point_2d_distance_symmetry(p1 in arb_point_2d(), p2 in arb_point_2d()) {
        let d1 = p1.distance_sq(&p2);
        let d2 = p2.distance_sq(&p1);
        prop_assert!((d1 - d2).abs() < 1e-10, "Distance should be symmetric");
    }

    #[test]
    fn test_point_2d_distance_non_negative(p1 in arb_point_2d(), p2 in arb_point_2d()) {
        let dist = p1.distance_sq(&p2);
        prop_assert!(dist >= 0.0, "Distance squared must be non-negative");
    }

    #[test]
    fn test_point_2d_self_distance_zero(p in arb_point_2d()) {
        let dist = p.distance_sq(&p);
        prop_assert!(dist.abs() < 1e-10, "Distance to self should be zero");
    }

    #[test]
    fn test_point_3d_distance_symmetry(p1 in arb_point_3d(), p2 in arb_point_3d()) {
        let d1 = p1.distance_sq(&p2);
        let d2 = p2.distance_sq(&p1);
        prop_assert!((d1 - d2).abs() < 1e-10, "Distance should be symmetric");
    }

    #[test]
    fn test_rectangle_contains_center(rect in arb_rectangle()) {
        let center: Point2D<()> = Point2D::new(
            rect.x + rect.width / 2.0,
            rect.y + rect.height / 2.0,
            None
        );
        prop_assert!(rect.contains(&center), "Rectangle should contain its center");
    }

    #[test]
    fn test_rectangle_area_positive(rect in arb_rectangle()) {
        prop_assert!(rect.area() > 0.0, "Rectangle area must be positive");
    }

    #[test]
    fn test_rectangle_union_contains_both(r1 in arb_rectangle(), r2 in arb_rectangle()) {
        let union = r1.union(&r2);

        // Check corners of r1
        let p1: Point2D<()> = Point2D::new(r1.x, r1.y, None);
        let p2: Point2D<()> = Point2D::new(r1.x + r1.width, r1.y + r1.height, None);

        prop_assert!(union.contains(&p1), "Union should contain r1's min corner");
        prop_assert!(union.contains(&p2), "Union should contain r1's max corner");

        // Check corners of r2
        let p3: Point2D<()> = Point2D::new(r2.x, r2.y, None);
        let p4: Point2D<()> = Point2D::new(r2.x + r2.width, r2.y + r2.height, None);

        prop_assert!(union.contains(&p3), "Union should contain r2's min corner");
        prop_assert!(union.contains(&p4), "Union should contain r2's max corner");
    }

    #[test]
    fn test_rectangle_intersects_self(rect in arb_rectangle()) {
        prop_assert!(rect.intersects(&rect), "Rectangle should intersect itself");
    }

    #[test]
    fn test_rectangle_intersects_symmetric(r1 in arb_rectangle(), r2 in arb_rectangle()) {
        prop_assert_eq!(r1.intersects(&r2), r2.intersects(&r1), "Intersection should be symmetric");
    }

    #[test]
    fn test_cube_contains_center(cube in arb_cube()) {
        let center: Point3D<()> = Point3D::new(
            cube.x + cube.width / 2.0,
            cube.y + cube.height / 2.0,
            cube.z + cube.depth / 2.0,
            None
        );
        prop_assert!(cube.contains(&center), "Cube should contain its center");
    }

    #[test]
    fn test_cube_intersects_self(cube in arb_cube()) {
        prop_assert!(cube.intersects(&cube), "Cube should intersect itself");
    }

    #[test]
    fn test_cube_intersects_symmetric(c1 in arb_cube(), c2 in arb_cube()) {
        prop_assert_eq!(c1.intersects(&c2), c2.intersects(&c1), "Intersection should be symmetric");
    }
}
