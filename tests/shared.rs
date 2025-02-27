#![allow(dead_code)]

//! Shared test utilities for Spart.
//!
//! This module provides common constants, sample data, and helper functions that are used
//! across multiple tests. It includes test parameters (e.g. capacity, radius), functions that
//! return target or query points for 2D and 3D tests, and distance functions for comparing points.

use spart::geometry::{Cube, Point2D, Point3D, Rectangle};

//
// Constants
//
pub const CAPACITY: usize = 4;

pub const BOUNDARY_RECT: Rectangle = Rectangle {
    x: 0.0,
    y: 0.0,
    width: 100.0,
    height: 100.0,
};

pub const BOUNDARY_CUBE: Cube = Cube {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    width: 100.0,
    height: 100.0,
    depth: 100.0,
};

pub const RADIUS: f64 = 30.0;
pub const KNN_COUNT: usize = 2;

//
// Query Points
//
pub fn target_point_2d() -> Point2D<&'static str> {
    Point2D {
        x: 35.0,
        y: 45.0,
        data: None,
    }
}

pub fn target_point_3d() -> Point3D<&'static str> {
    Point3D {
        x: 35.0,
        y: 45.0,
        z: 35.0,
        data: None,
    }
}

pub fn range_query_point_2d() -> Point2D<&'static str> {
    Point2D {
        x: 20.0,
        y: 20.0,
        data: None,
    }
}

pub fn range_query_point_3d() -> Point3D<&'static str> {
    Point3D {
        x: 20.0,
        y: 20.0,
        z: 20.0,
        data: None,
    }
}

//
// Query Volumes
//
pub fn query_rect() -> Rectangle {
    Rectangle {
        x: 20.0 - RADIUS,
        y: 20.0 - RADIUS,
        width: 2.0 * RADIUS,
        height: 2.0 * RADIUS,
    }
}

pub fn query_cube() -> Cube {
    Cube {
        x: 20.0 - RADIUS,
        y: 20.0 - RADIUS,
        z: 20.0 - RADIUS,
        width: 2.0 * RADIUS,
        height: 2.0 * RADIUS,
        depth: 2.0 * RADIUS,
    }
}

//
// Common Points
//
pub fn common_points_2d() -> Vec<Point2D<&'static str>> {
    vec![
        Point2D::new(11.0, 11.0, Some("A")),
        Point2D::new(51.0, 51.0, Some("B")),
        Point2D::new(31.0, 41.0, Some("C")),
        Point2D::new(71.0, 81.0, Some("D")),
        Point2D::new(81.0, 91.0, Some("E")),
        Point2D::new(21.0, 21.0, Some("F")),
        Point2D::new(22.0, 22.0, Some("G")),
        Point2D::new(23.0, 23.0, Some("H")),
        Point2D::new(24.0, 24.0, Some("I")),
        Point2D::new(25.0, 25.0, Some("J")),
        Point2D::new(26.0, 26.0, Some("K")),
    ]
}

pub fn common_points_3d() -> Vec<Point3D<&'static str>> {
    vec![
        Point3D::new(11.0, 11.0, 11.0, Some("A")),
        Point3D::new(51.0, 51.0, 51.0, Some("B")),
        Point3D::new(31.0, 41.0, 21.0, Some("C")),
        Point3D::new(71.0, 81.0, 91.0, Some("D")),
        Point3D::new(81.0, 91.0, 71.0, Some("E")),
        Point3D::new(21.0, 21.0, 21.0, Some("F")),
        Point3D::new(22.0, 22.0, 22.0, Some("G")),
        Point3D::new(23.0, 23.0, 23.0, Some("H")),
        Point3D::new(24.0, 24.0, 24.0, Some("I")),
        Point3D::new(25.0, 25.0, 25.0, Some("J")),
        Point3D::new(26.0, 26.0, 26.0, Some("K")),
    ]
}

//
// Distance Functions
//
pub fn distance_2d(a: &Point2D<impl std::fmt::Debug>, b: &Point2D<impl std::fmt::Debug>) -> f64 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

pub fn distance_3d(a: &Point3D<impl std::fmt::Debug>, b: &Point3D<impl std::fmt::Debug>) -> f64 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2) + (a.z - b.z).powi(2)).sqrt()
}
