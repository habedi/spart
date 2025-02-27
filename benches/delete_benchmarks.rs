#[path = "shared.rs"]
mod shared;
use shared::*;

use criterion::{black_box, criterion_group, Criterion};
use spart::bsp_tree::{Point2DBSP, Point3DBSP};
use spart::geometry::{Point2D, Point3D, Rectangle};
use spart::{bsp_tree, kd_tree, octree, quadtree, r_tree};
use tracing::info;

/// Helper function that benchmarks a deletion function on a given dataset.
///
/// # Arguments
///
/// * `bench_name` - The name of the benchmark.
/// * `points` - A slice of points to be inserted into the tree.
/// * `delete_fn` - A closure that takes the points slice and performs tree insertion and deletion.
fn bench_delete_tree<P>(bench_name: &str, points: &[P], delete_fn: impl Fn(&[P])) {
    let mut cc = configure_criterion();
    cc.bench_function(bench_name, |b| {
        b.iter(|| {
            info!("Running deletion benchmark: {}", bench_name);
            delete_fn(black_box(points));
        })
    });
}

/// Deletes all points from a 2D Quadtree.
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

/// Deletes all points from a 3D Octree.
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

/// Deletes all points from a 2D KdTree.
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

/// Deletes all points from a 3D KdTree.
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

/// Deletes all points from a 2D RTree.
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

/// Deletes all points from a 3D RTree.
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

/// Deletes all points from a 2D BSPTree.
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

/// Deletes all points from a 3D BSPTree.
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
    bench_delete_tree("delete_2d_quadtree", &points, delete_2d_quadtree);
}

fn benchmark_delete_octree_3d(_c: &mut Criterion) {
    let points = generate_3d_data();
    info!("Benchmark 'delete_3d_octree' started");
    bench_delete_tree("delete_3d_octree", &points, delete_3d_octree);
}

fn benchmark_delete_kdtree_2d(_c: &mut Criterion) {
    let points = generate_2d_data();
    info!("Benchmark 'delete_2d_kdtree' started");
    bench_delete_tree("delete_2d_kdtree", &points, delete_2d_kdtree);
}

fn benchmark_delete_kdtree_3d(_c: &mut Criterion) {
    let points = generate_3d_data();
    info!("Benchmark 'delete_3d_kdtree' started");
    bench_delete_tree("delete_3d_kdtree", &points, delete_3d_kdtree);
}

fn benchmark_delete_rtree_2d(_c: &mut Criterion) {
    let points = generate_2d_data();
    info!("Benchmark 'delete_2d_rtree' started");
    bench_delete_tree("delete_2d_rtree", &points, delete_2d_rtree);
}

fn benchmark_delete_rtree_3d(_c: &mut Criterion) {
    let points = generate_3d_data();
    info!("Benchmark 'delete_3d_rtree' started");
    bench_delete_tree("delete_3d_rtree", &points, delete_3d_rtree);
}

fn benchmark_delete_bsptree_2d(_c: &mut Criterion) {
    let points = generate_2d_data_wrapped();
    info!("Benchmark 'delete_2d_bsptree' started");
    bench_delete_tree("delete_2d_bsptree", &points, delete_2d_bsptree);
}

fn benchmark_delete_bsptree_3d(_c: &mut Criterion) {
    let points = generate_3d_data_wrapped();
    info!("Benchmark 'delete_3d_bsptree' started");
    bench_delete_tree("delete_3d_bsptree", &points, delete_3d_bsptree);
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
