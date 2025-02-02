use criterion::{black_box, criterion_group, criterion_main, Criterion};
use spart::geometry::{Cube, Point2D, Point3D, Rectangle};
use spart::{kdtree, octree, quadtree, rtree};

// Data generation functions

// Generate a lot of random 2D points
fn generate_2d_data() -> Vec<Point2D<i32>> {
    (0..1000)
        .map(|i| Point2D::new(i as f64, i as f64, Some(i)))
        .collect()
}

// Generate a lot of random 3D points
fn generate_3d_data() -> Vec<Point3D<i32>> {
    (0..1000)
        .map(|i| Point3D::new(i as f64, i as f64, i as f64, Some(i)))
        .collect()
}

// Insert a lot of points into a Quadtree (2D), Octree (3d), KdTree (2D and 3D), and RTree (2D and 3D)

fn insert_2d_quadtree(points: Vec<Point2D<i32>>) {
    let boundary = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 100.0,
        height: 100.0,
    };
    let mut tree = quadtree::Quadtree::new(&boundary, 5);
    for point in points {
        tree.insert(point);
    }
}

fn insert_3d_octree(points: Vec<Point3D<i32>>) {
    let boundary = Cube {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        width: 100.0,
        height: 100.0,
        depth: 100.0,
    };
    let mut tree = octree::Octree::new(&boundary, 5);
    for point in points {
        tree.insert(point);
    }
}

fn insert_2d_kdtree(points: Vec<Point2D<i32>>) {
    let mut tree = kdtree::KdTree::new(2);
    for point in points {
        tree.insert(point);
    }
}

fn insert_3d_kdtree(points: Vec<Point3D<i32>>) {
    let mut tree = kdtree::KdTree::new(3);
    for point in points {
        tree.insert(point);
    }
}

fn insert_2d_rtree(points: Vec<Point2D<i32>>) {
    let mut tree = rtree::RTree::new(5);
    for point in points {
        tree.insert(point);
    }
}

fn insert_3d_rtree(points: Vec<Point3D<i32>>) {
    let mut tree = rtree::RTree::new(5);
    for point in points {
        tree.insert(point);
    }
}

// Benchmark function for Criterion
fn criterion_benchmark(c: &mut Criterion) {
    //let 2d_points = generate_2d_data();
    //let 3d_points = generate_3d_data();
    c.bench_function("my_function", |b| {
        b.iter(|| {
            // Use black_box to prevent the compiler from optimizing the function away
            black_box(generate_2d_data());
            black_box(generate_3d_data());
            black_box(insert_2d_quadtree(generate_2d_data()));
            black_box(insert_3d_octree(generate_3d_data()));
            black_box(insert_2d_kdtree(generate_2d_data()));
            black_box(insert_3d_kdtree(generate_3d_data()));
            black_box(insert_2d_rtree(generate_2d_data()));
            black_box(insert_3d_rtree(generate_3d_data()));
        })
    });
}

// Criterion requires these macros to define benchmark groups and main entry point
criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
