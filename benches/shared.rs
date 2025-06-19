#![allow(dead_code)]

//! Shared utilities for benchmarks in Spart.
//!
//! This module provides common constants, sample data generators, and helper functions
//! used in benchmark tests. It includes benchmark parameters (e.g. number of insertions,
//! node capacity), boundary definitions, and functions for generating 2D and 3D data,
//! both in raw and BSP‑wrapped formats.

use criterion::Criterion;
use spart::geometry::{Cube, Point2D, Point3D};
use tracing::{debug, info};

//
// Benchmark Parameters
//
pub const BENCH_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);
pub const BENCH_NUM_INSERT: i32 = 10;
pub const BENCH_NODE_CAPACITY: usize = 5;

pub const BENCH_KNN_SIZE: usize = 3;
pub const BENCH_RANGE_RADIUS: f64 = 30.0;

//
// Boundary Definitions
//
pub const BENCH_BOUNDARY: Cube = Cube {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    width: 100.0,
    height: 100.0,
    depth: 100.0,
};

//
// Data Generation Functions (Raw Data)
//
pub fn generate_2d_data() -> Vec<Point2D<i32>> {
    info!("Generating 2D data with {} points", BENCH_NUM_INSERT);
    let data: Vec<Point2D<i32>> = (0..BENCH_NUM_INSERT)
        .map(|i| {
            let pt = Point2D::new(i as f64, i as f64, Some(i));
            debug!("Generated 2D point: {:?}", pt);
            pt
        })
        .collect();
    info!("Finished generating 2D data ({} points)", data.len());
    data
}

pub fn generate_3d_data() -> Vec<Point3D<i32>> {
    info!("Generating 3D data with {} points", BENCH_NUM_INSERT);
    let data: Vec<Point3D<i32>> = (0..BENCH_NUM_INSERT)
        .map(|i| {
            let pt = Point3D::new(i as f64, i as f64, i as f64, Some(i));
            debug!("Generated 3D point: {:?}", pt);
            pt
        })
        .collect();
    info!("Finished generating 3D data ({} points)", data.len());
    data
}

// Configure Criterion with a timeout for benchmarks
pub fn configure_criterion() -> Criterion {
    Criterion::default().measurement_time(BENCH_TIMEOUT)
}
