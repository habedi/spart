#![allow(dead_code)]

use spart::bsp_tree::{Point2DBSP, Point3DBSP};
use spart::geometry::{Point2D, Point3D};
use tracing::{debug, info};

pub const BENCH_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);
pub const BENCH_NUM_INSERT: i32 = 100;
pub const BENCH_NODE_CAPACITY: usize = 5;

pub const BENCH_BOUNDARY: spart::geometry::Cube = spart::geometry::Cube {
    x: 0.0,
    y: 0.0,
    z: 0.0,
    width: 1000.0,
    height: 1000.0,
    depth: 1000.0,
};

pub const BENCH_KNN_SIZE: usize = 3;
pub const BENCH_RANGE_RADIUS: f64 = 30.0;

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

pub fn generate_2d_data_wrapped() -> Vec<Point2DBSP<i32>> {
    info!(
        "Generating wrapped 2D data for BSPTree with {} points",
        BENCH_NUM_INSERT
    );
    let data: Vec<Point2DBSP<i32>> = (0..BENCH_NUM_INSERT)
        .map(|i| {
            let pt = Point2DBSP {
                point: Point2D::new(i as f64, i as f64, Some(i)),
            };
            debug!("Generated wrapped 2D point: {:?}", pt);
            pt
        })
        .collect();
    info!(
        "Finished generating wrapped 2D data ({} points)",
        data.len()
    );
    data
}

pub fn generate_3d_data_wrapped() -> Vec<Point3DBSP<i32>> {
    info!(
        "Generating wrapped 3D data for BSPTree with {} points",
        BENCH_NUM_INSERT
    );
    let data: Vec<Point3DBSP<i32>> = (0..BENCH_NUM_INSERT)
        .map(|i| {
            let pt = Point3DBSP {
                point: Point3D::new(i as f64, i as f64, i as f64, Some(i)),
            };
            debug!("Generated wrapped 3D point: {:?}", pt);
            pt
        })
        .collect();
    info!(
        "Finished generating wrapped 3D data ({} points)",
        data.len()
    );
    data
}
