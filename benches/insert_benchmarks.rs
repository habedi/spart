#[path = "shared.rs"]
mod shared;
use shared::*;

use criterion::{black_box, criterion_group, Criterion};
use spart::bsp_tree::{Point2DBSP, Point3DBSP};
use spart::geometry::{Point2D, Point3D, Rectangle};
use spart::{bsp_tree, kd_tree, octree, quadtree, r_tree};
use tracing::info;

/// A generic helper that benchmarks an insertion function.
///
/// # Arguments
///
/// * `bench_name` - The name of the benchmark.
/// * `points` - A slice of points to be inserted.
/// * `insert_fn` - A closure that takes a slice of points and performs insertion.
/// * `cc` - A mutable reference to a configured Criterion instance.
fn bench_insert<P>(bench_name: &str, points: &[P], insert_fn: impl Fn(&[P]), cc: &mut Criterion) {
    cc.bench_function(bench_name, |b| {
        b.iter(|| {
            // Use black_box to avoid optimizations.
            insert_fn(black_box(points))
        })
    });
}

fn insert_2d_quadtree(points: &[Point2D<i32>]) {
    info!("Starting insertion for 2D Quadtree");
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
    info!("Finished insertion for 2D Quadtree");
}

fn insert_3d_octree(points: &[Point3D<i32>]) {
    info!("Starting insertion for 3D Octree");
    let boundary = BENCH_BOUNDARY;
    let mut tree = octree::Octree::new(&boundary, BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    info!("Finished insertion for 3D Octree");
}

fn insert_2d_kdtree(points: &[Point2D<i32>]) {
    info!("Starting insertion for 2D KdTree");
    let mut tree = kd_tree::KdTree::new(2);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    info!("Finished insertion for 2D KdTree");
}

fn insert_3d_kdtree(points: &[Point3D<i32>]) {
    info!("Starting insertion for 3D KdTree");
    let mut tree = kd_tree::KdTree::new(3);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    info!("Finished insertion for 3D KdTree");
}

fn insert_2d_rtree(points: &[Point2D<i32>]) {
    info!("Starting insertion for 2D RTree");
    let mut tree = r_tree::RTree::new(BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    info!("Finished insertion for 2D RTree");
}

fn insert_3d_rtree(points: &[Point3D<i32>]) {
    info!("Starting insertion for 3D RTree");
    let mut tree = r_tree::RTree::new(BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    info!("Finished insertion for 3D RTree");
}

fn insert_2d_bsptree(points: &[Point2DBSP<i32>]) {
    info!("Starting insertion for 2D BSPTree");
    let mut tree = bsp_tree::BSPTree::new(BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    info!("Finished insertion for 2D BSPTree");
}

fn insert_3d_bsptree(points: &[Point3DBSP<i32>]) {
    info!("Starting insertion for 3D BSPTree");
    let mut tree = bsp_tree::BSPTree::new(BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    info!("Finished insertion for 3D BSPTree");
}

fn bench_insert_quadtree_2d(_c: &mut Criterion) {
    let points = generate_2d_data();
    info!("Benchmark 'insert_2d_quadtree' started");
    let mut cc = configure_criterion();
    bench_insert("insert_2d_quadtree", &points, insert_2d_quadtree, &mut cc);
}

fn bench_insert_octree_3d(_c: &mut Criterion) {
    let points = generate_3d_data();
    info!("Benchmark 'insert_3d_octree' started");
    let mut cc = configure_criterion();
    bench_insert("insert_3d_octree", &points, insert_3d_octree, &mut cc);
}

fn bench_insert_kdtree_2d(_c: &mut Criterion) {
    let points = generate_2d_data();
    info!("Benchmark 'insert_2d_kdtree' started");
    let mut cc = configure_criterion();
    bench_insert("insert_2d_kdtree", &points, insert_2d_kdtree, &mut cc);
}

fn bench_insert_kdtree_3d(_c: &mut Criterion) {
    let points = generate_3d_data();
    info!("Benchmark 'insert_3d_kdtree' started");
    let mut cc = configure_criterion();
    bench_insert("insert_3d_kdtree", &points, insert_3d_kdtree, &mut cc);
}

fn bench_insert_rtree_2d(_c: &mut Criterion) {
    let points = generate_2d_data();
    info!("Benchmark 'insert_2d_rtree' started");
    let mut cc = configure_criterion();
    bench_insert("insert_2d_rtree", &points, insert_2d_rtree, &mut cc);
}

fn bench_insert_rtree_3d(_c: &mut Criterion) {
    let points = generate_3d_data();
    info!("Benchmark 'insert_3d_rtree' started");
    let mut cc = configure_criterion();
    bench_insert("insert_3d_rtree", &points, insert_3d_rtree, &mut cc);
}

fn bench_insert_bsptree_2d(_c: &mut Criterion) {
    let points = generate_2d_data_wrapped();
    info!("Benchmark 'insert_2d_bsptree' started");
    let mut cc = configure_criterion();
    bench_insert("insert_2d_bsptree", &points, insert_2d_bsptree, &mut cc);
}

fn bench_insert_bsptree_3d(_c: &mut Criterion) {
    let points = generate_3d_data_wrapped();
    info!("Benchmark 'insert_3d_bsptree' started");
    let mut cc = configure_criterion();
    bench_insert("insert_3d_bsptree", &points, insert_3d_bsptree, &mut cc);
}

criterion_group!(
    benches,
    bench_insert_quadtree_2d,
    bench_insert_octree_3d,
    bench_insert_kdtree_2d,
    bench_insert_kdtree_3d,
    bench_insert_rtree_2d,
    bench_insert_rtree_3d,
    bench_insert_bsptree_2d,
    bench_insert_bsptree_3d
);
