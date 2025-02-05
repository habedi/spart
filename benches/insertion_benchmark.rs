use criterion::{black_box, criterion_group, criterion_main, Criterion};
use spart::geometry::{Cube, Point2D, Point3D, Rectangle};
use spart::{kdtree, octree, quadtree, rtree};

/// Generate a vector of 1000 2D points.
fn generate_2d_data() -> Vec<Point2D<i32>> {
    (0..1000)
        .map(|i| Point2D::new(i as f64, i as f64, Some(i)))
        .collect()
}

/// Generate a vector of 1000 3D points.
fn generate_3d_data() -> Vec<Point3D<i32>> {
    (0..1000)
        .map(|i| Point3D::new(i as f64, i as f64, i as f64, Some(i)))
        .collect()
}

/// Insert points into a Quadtree (2D).
fn insert_2d_quadtree(points: &[Point2D<i32>]) {
    let boundary = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 1000.0,
        height: 1000.0,
    };
    let mut tree = quadtree::Quadtree::new(&boundary, 5);
    for point in points.iter() {
        // Clone the point to get an owned value.
        tree.insert(point.clone());
    }
}

/// Insert points into an Octree (3D).
fn insert_3d_octree(points: &[Point3D<i32>]) {
    let boundary = Cube {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        width: 1000.0,
        height: 1000.0,
        depth: 1000.0,
    };
    let mut tree = octree::Octree::new(&boundary, 5);
    for point in points.iter() {
        tree.insert(point.clone());
    }
}

/// Insert points into a KdTree (2D).
fn insert_2d_kdtree(points: &[Point2D<i32>]) {
    let mut tree = kdtree::KdTree::new(2);
    for point in points.iter() {
        tree.insert(point.clone());
    }
}

/// Insert points into a KdTree (3D).
fn insert_3d_kdtree(points: &[Point3D<i32>]) {
    let mut tree = kdtree::KdTree::new(3);
    for point in points.iter() {
        tree.insert(point.clone());
    }
}

/// Insert points into an RTree (2D).
fn insert_2d_rtree(points: &[Point2D<i32>]) {
    let mut tree = rtree::RTree::new(5);
    for point in points.iter() {
        tree.insert(point.clone());
    }
}

/// Insert points into an RTree (3D).
fn insert_3d_rtree(points: &[Point3D<i32>]) {
    let mut tree = rtree::RTree::new(5);
    for point in points.iter() {
        tree.insert(point.clone());
    }
}

/// Benchmark insertion into a 2D Quadtree.
fn bench_insert_quadtree_2d(c: &mut Criterion) {
    let points = generate_2d_data();
    c.bench_function("insert_2d_quadtree", |b| {
        b.iter(|| insert_2d_quadtree(black_box(&points)))
    });
}

/// Benchmark insertion into a 3D Octree.
fn bench_insert_octree_3d(c: &mut Criterion) {
    let points = generate_3d_data();
    c.bench_function("insert_3d_octree", |b| {
        b.iter(|| insert_3d_octree(black_box(&points)))
    });
}

/// Benchmark insertion into a 2D KdTree.
fn bench_insert_kdtree_2d(c: &mut Criterion) {
    let points = generate_2d_data();
    c.bench_function("insert_2d_kdtree", |b| {
        b.iter(|| insert_2d_kdtree(black_box(&points)))
    });
}

/// Benchmark insertion into a 3D KdTree.
fn bench_insert_kdtree_3d(c: &mut Criterion) {
    let points = generate_3d_data();
    c.bench_function("insert_3d_kdtree", |b| {
        b.iter(|| insert_3d_kdtree(black_box(&points)))
    });
}

/// Benchmark insertion into a 2D RTree.
fn bench_insert_rtree_2d(c: &mut Criterion) {
    let points = generate_2d_data();
    c.bench_function("insert_2d_rtree", |b| {
        b.iter(|| insert_2d_rtree(black_box(&points)))
    });
}

/// Benchmark insertion into a 3D RTree.
fn bench_insert_rtree_3d(c: &mut Criterion) {
    let points = generate_3d_data();
    c.bench_function("insert_3d_rtree", |b| {
        b.iter(|| insert_3d_rtree(black_box(&points)))
    });
}

criterion_group!(
    benches,
    bench_insert_quadtree_2d,
    bench_insert_octree_3d,
    bench_insert_kdtree_2d,
    bench_insert_kdtree_3d,
    bench_insert_rtree_2d,
    bench_insert_rtree_3d
);
criterion_main!(benches);
