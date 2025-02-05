use criterion::{black_box, criterion_group, criterion_main, Criterion};
use spart::bsp_tree::{Point2DBSP, Point3DBSP};
use spart::geometry::{Point2D, Point3D, Rectangle};
use spart::{bsp_tree, kd_tree, octree, quadtree, r_tree};
use tracing::info;
mod utils;
use utils::*;

pub fn configure_criterion() -> Criterion {
    Criterion::default().measurement_time(BENCH_TIMEOUT)
}

fn delete_2d_quadtree(points: &[Point2D<i32>]) {
    info!("Starting deletion for 2D Quadtree");
    let boundary = Rectangle {
        x: BENCH_BOUNDARY.x,
        y: BENCH_BOUNDARY.y,
        width: BENCH_BOUNDARY.width,
        height: BENCH_BOUNDARY.height,
    };
    let mut tree = quadtree::Quadtree::new(&boundary, BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    for point in points.iter() {
        tree.delete(&point.clone());
    }
    info!("Finished deletion for 2D Quadtree");
}

fn delete_3d_octree(points: &[Point3D<i32>]) {
    info!("Starting deletion for 3D Octree");
    let boundary = BENCH_BOUNDARY;
    let mut tree = octree::Octree::new(&boundary, BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    for point in points.iter() {
        tree.delete(&point.clone());
    }
    info!("Finished deletion for 3D Octree");
}

fn delete_2d_kdtree(points: &[Point2D<i32>]) {
    info!("Starting deletion for 2D KdTree");
    let mut tree = kd_tree::KdTree::new(2);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    for point in points.iter() {
        tree.delete(&point.clone());
    }
    info!("Finished deletion for 2D KdTree");
}

fn delete_3d_kdtree(points: &[Point3D<i32>]) {
    info!("Starting deletion for 3D KdTree");
    let mut tree = kd_tree::KdTree::new(3);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    for point in points.iter() {
        tree.delete(&point.clone());
    }
    info!("Finished deletion for 3D KdTree");
}

fn delete_2d_rtree(points: &[Point2D<i32>]) {
    info!("Starting deletion for 2D RTree");
    let mut tree = r_tree::RTree::new(BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    for point in points.iter() {
        tree.delete(&point.clone());
    }
    info!("Finished deletion for 2D RTree");
}

fn delete_3d_rtree(points: &[Point3D<i32>]) {
    info!("Starting deletion for 3D RTree");
    let mut tree = r_tree::RTree::new(BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    for point in points.iter() {
        tree.delete(&point.clone());
    }
    info!("Finished deletion for 3D RTree");
}

fn delete_2d_bsptree(points: &[Point2DBSP<i32>]) {
    info!("Starting deletion for 2D BSPTree");
    let mut tree = bsp_tree::BSPTree::new(BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    for point in points.iter() {
        tree.delete(&point.clone());
    }
    info!("Finished deletion for 2D BSPTree");
}

fn delete_3d_bsptree(points: &[Point3DBSP<i32>]) {
    info!("Starting deletion for 3D BSPTree");
    let mut tree = bsp_tree::BSPTree::new(BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    for point in points.iter() {
        tree.delete(&point.clone());
    }
    info!("Finished deletion for 3D BSPTree");
}

fn benchmark_delete_quadtree_2d(_c: &mut Criterion) {
    let points = generate_2d_data();
    info!("Benchmark 'delete_2d_quadtree' started");
    let mut cc = configure_criterion();
    cc.bench_function("delete_2d_quadtree", |b| {
        b.iter(|| delete_2d_quadtree(black_box(&points)))
    });
}

fn benchmark_delete_octree_3d(_c: &mut Criterion) {
    let points = generate_3d_data();
    info!("Benchmark 'delete_3d_octree' started");
    let mut cc = configure_criterion();
    cc.bench_function("delete_3d_octree", |b| {
        b.iter(|| delete_3d_octree(black_box(&points)))
    });
}

fn benchmark_delete_kdtree_2d(_c: &mut Criterion) {
    let points = generate_2d_data();
    info!("Benchmark 'delete_2d_kdtree' started");
    let mut cc = configure_criterion();
    cc.bench_function("delete_2d_kdtree", |b| {
        b.iter(|| delete_2d_kdtree(black_box(&points)))
    });
}

fn benchmark_delete_kdtree_3d(_c: &mut Criterion) {
    let points = generate_3d_data();
    info!("Benchmark 'delete_3d_kdtree' started");
    let mut cc = configure_criterion();
    cc.bench_function("delete_3d_kdtree", |b| {
        b.iter(|| delete_3d_kdtree(black_box(&points)))
    });
}

fn benchmark_delete_rtree_2d(_c: &mut Criterion) {
    let points = generate_2d_data();
    info!("Benchmark 'delete_2d_rtree' started");
    let mut cc = configure_criterion();
    cc.bench_function("delete_2d_rtree", |b| {
        b.iter(|| delete_2d_rtree(black_box(&points)))
    });
}

fn benchmark_delete_rtree_3d(_c: &mut Criterion) {
    let points = generate_3d_data();
    info!("Benchmark 'delete_3d_rtree' started");
    let mut cc = configure_criterion();
    cc.bench_function("delete_3d_rtree", |b| {
        b.iter(|| delete_3d_rtree(black_box(&points)))
    });
}

fn benchmark_delete_bsptree_2d(_c: &mut Criterion) {
    let points = generate_2d_data_wrapped();
    info!("Benchmark 'delete_2d_bsptree' started");
    let mut cc = configure_criterion();
    cc.bench_function("delete_2d_bsptree", |b| {
        b.iter(|| delete_2d_bsptree(black_box(&points)))
    });
}

fn benchmark_delete_bsptree_3d(_c: &mut Criterion) {
    let points = generate_3d_data_wrapped();
    info!("Benchmark 'delete_3d_bsptree' started");
    let mut cc = configure_criterion();
    cc.bench_function("delete_3d_bsptree", |b| {
        b.iter(|| delete_3d_bsptree(black_box(&points)))
    });
}

criterion_group!(
    benches,
    benchmark_delete_quadtree_2d,
    benchmark_delete_octree_3d,
    benchmark_delete_kdtree_2d,
    benchmark_delete_kdtree_3d,
    benchmark_delete_rtree_2d,
    benchmark_delete_rtree_3d,
    benchmark_delete_bsptree_2d,
    benchmark_delete_bsptree_3d
);
criterion_main!(benches);
