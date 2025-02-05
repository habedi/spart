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

/// Build a 2D Quadtree, insert all points, then delete them.
fn delete_2d_quadtree(points: &[Point2D<i32>]) {
    let boundary = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 1000.0,
        height: 1000.0,
    };
    let mut tree = quadtree::Quadtree::new(&boundary, 5);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    // Delete each point.
    for point in points.iter() {
        // Assumes the tree has a `delete` method that takes an owned value.
        tree.delete(point.clone());
    }
}

/// Build a 3D Octree, insert all points, then delete them.
fn delete_3d_octree(points: &[Point3D<i32>]) {
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
    for point in points.iter() {
        tree.delete(point.clone());
    }
}

/// Build a 2D KdTree, insert all points, then delete them.
fn delete_2d_kdtree(points: &[Point2D<i32>]) {
    let mut tree = kdtree::KdTree::new(2);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    for point in points.iter() {
        tree.delete(point.clone());
    }
}

/// Build a 3D KdTree, insert all points, then delete them.
fn delete_3d_kdtree(points: &[Point3D<i32>]) {
    let mut tree = kdtree::KdTree::new(3);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    for point in points.iter() {
        tree.delete(point.clone());
    }
}

/// Build a 2D RTree, insert all points, then delete them.
fn delete_2d_rtree(points: &[Point2D<i32>]) {
    let mut tree = rtree::RTree::new(5);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    for point in points.iter() {
        tree.delete(point.clone());
    }
}

/// Build a 3D RTree, insert all points, then delete them.
fn delete_3d_rtree(points: &[Point3D<i32>]) {
    let mut tree = rtree::RTree::new(5);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    for point in points.iter() {
        tree.delete(point.clone());
    }
}

/// Benchmark deletion from a 2D Quadtree.
fn benchmark_delete_quadtree_2d(c: &mut Criterion) {
    let points = generate_2d_data();
    c.bench_function("delete_2d_quadtree", |b| {
        b.iter(|| {
            // Each iteration rebuilds the tree, then deletes all points.
            delete_2d_quadtree(black_box(&points))
        })
    });
}

/// Benchmark deletion from a 3D Octree.
fn benchmark_delete_octree_3d(c: &mut Criterion) {
    let points = generate_3d_data();
    c.bench_function("delete_3d_octree", |b| {
        b.iter(|| delete_3d_octree(black_box(&points)))
    });
}

/// Benchmark deletion from a 2D KdTree.
fn benchmark_delete_kdtree_2d(c: &mut Criterion) {
    let points = generate_2d_data();
    c.bench_function("delete_2d_kdtree", |b| {
        b.iter(|| delete_2d_kdtree(black_box(&points)))
    });
}

/// Benchmark deletion from a 3D KdTree.
fn benchmark_delete_kdtree_3d(c: &mut Criterion) {
    let points = generate_3d_data();
    c.bench_function("delete_3d_kdtree", |b| {
        b.iter(|| delete_3d_kdtree(black_box(&points)))
    });
}

/// Benchmark deletion from a 2D RTree.
fn benchmark_delete_rtree_2d(c: &mut Criterion) {
    let points = generate_2d_data();
    c.bench_function("delete_2d_rtree", |b| {
        b.iter(|| delete_2d_rtree(black_box(&points)))
    });
}

/// Benchmark deletion from a 3D RTree.
fn benchmark_delete_rtree_3d(c: &mut Criterion) {
    let points = generate_3d_data();
    c.bench_function("delete_3d_rtree", |b| {
        b.iter(|| delete_3d_rtree(black_box(&points)))
    });
}

criterion_group!(
    benches,
    benchmark_delete_quadtree_2d,
    //benchmark_delete_octree_3d,
    //benchmark_delete_kdtree_2d,
    //benchmark_delete_kdtree_3d,
    //benchmark_delete_rtree_2d,
    //benchmark_delete_rtree_3d
);
criterion_main!(benches);
