#[path = "shared.rs"]
mod shared;
use shared::*;

use criterion::{criterion_group, Criterion};
use spart::geometry::{EuclideanDistance, Point2D, Point3D, Rectangle};
use spart::{kd_tree, octree, quadtree, r_star_tree, r_tree};
use std::hint::black_box;
use tracing::info;

/// Configures Criterion using the shared benchmark timeout.
fn configure_criterion() -> Criterion {
    Criterion::default().measurement_time(BENCH_TIMEOUT)
}

fn benchmark_knn_kdtree_2d(_c: &mut Criterion) {
    info!("Setting up benchmark: knn_kdtree_2d");
    let points = generate_2d_data();
    let mut tree = kd_tree::KdTree::<Point2D<i32>>::new(2);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let target = Point2D::new(35.0, 45.0, None);
    let mut cc = configure_criterion();
    let name = "knn_kdtree_2d";
    cc.bench_function(name, |b| {
        b.iter(|| {
            info!("Running knn search benchmark: {}", name);
            let res = tree.knn_search::<EuclideanDistance>(&target, BENCH_KNN_SIZE);
            info!("Completed knn search benchmark: {}", name);
            black_box(res)
        })
    });
}

fn benchmark_knn_rtree_2d(_c: &mut Criterion) {
    info!("Setting up benchmark: knn_rtree_2d");
    let points = generate_2d_data();
    let mut tree = r_tree::RTree::<Point2D<i32>>::new(BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let target = Point2D::new(35.0, 45.0, None);
    let mut cc = configure_criterion();
    let name = "knn_rtree_2d";
    cc.bench_function(name, |b| {
        b.iter(|| {
            info!("Running knn search benchmark: {}", name);
            let res = tree.knn_search::<EuclideanDistance>(&target, BENCH_KNN_SIZE);
            info!("Completed knn search benchmark: {}", name);
            black_box(res)
        })
    });
}

fn benchmark_knn_quadtree_2d(_c: &mut Criterion) {
    info!("Setting up benchmark: knn_quadtree_2d");
    let points = generate_2d_data();
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
    let target = Point2D::new(35.0, 45.0, None);
    let mut cc = configure_criterion();
    let name = "knn_quadtree_2d";
    cc.bench_function(name, |b| {
        b.iter(|| {
            info!("Running knn search benchmark: {}", name);
            let res = tree.knn_search::<EuclideanDistance>(&target, BENCH_KNN_SIZE);
            info!("Completed knn search benchmark: {}", name);
            black_box(res)
        })
    });
}

fn benchmark_knn_kdtree_3d(_c: &mut Criterion) {
    info!("Setting up benchmark: knn_kdtree_3d");
    let points = generate_3d_data();
    let mut tree = kd_tree::KdTree::<Point3D<i32>>::new(3);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let target = Point3D::new(35.0, 45.0, 35.0, None);
    let mut cc = configure_criterion();
    let name = "knn_kdtree_3d";
    cc.bench_function(name, |b| {
        b.iter(|| {
            info!("Running knn search benchmark: {}", name);
            let res = tree.knn_search::<EuclideanDistance>(&target, BENCH_KNN_SIZE);
            info!("Completed knn search benchmark: {}", name);
            black_box(res)
        })
    });
}

fn benchmark_knn_rtree_3d(_c: &mut Criterion) {
    info!("Setting up benchmark: knn_rtree_3d");
    let points = generate_3d_data();
    let mut tree = r_tree::RTree::<Point3D<i32>>::new(BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let target = Point3D::new(35.0, 45.0, 35.0, None);
    let mut cc = configure_criterion();
    let name = "knn_rtree_3d";
    cc.bench_function(name, |b| {
        b.iter(|| {
            info!("Running knn search benchmark: {}", name);
            let res = tree.knn_search::<EuclideanDistance>(&target, BENCH_KNN_SIZE);
            info!("Completed knn search benchmark: {}", name);
            black_box(res)
        })
    });
}

fn benchmark_knn_octree_3d(_c: &mut Criterion) {
    info!("Setting up benchmark: knn_octree_3d");
    let points = generate_3d_data();
    let boundary = BENCH_BOUNDARY;
    let mut tree = octree::Octree::new(&boundary, BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let target = Point3D::new(35.0, 45.0, 35.0, None);
    let mut cc = configure_criterion();
    let name = "knn_octree_3d";
    cc.bench_function(name, |b| {
        b.iter(|| {
            info!("Running knn search benchmark: {}", name);
            let res = tree.knn_search::<EuclideanDistance>(&target, BENCH_KNN_SIZE);
            info!("Completed knn search benchmark: {}", name);
            black_box(res)
        })
    });
}

fn benchmark_knn_rstartree_2d(_c: &mut Criterion) {
    info!("Setting up benchmark: knn_rstartree_2d");
    let points = generate_2d_data();
    let mut tree = r_star_tree::RStarTree::<Point2D<i32>>::new(BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let target = Point2D::new(35.0, 45.0, None);
    let mut cc = configure_criterion();
    let name = "knn_rstartree_2d";
    cc.bench_function(name, |b| {
        b.iter(|| {
            info!("Running knn search benchmark: {}", name);
            let res = tree.knn_search::<EuclideanDistance>(&target, BENCH_KNN_SIZE);
            info!("Completed knn search benchmark: {}", name);
            black_box(res)
        })
    });
}

fn benchmark_knn_rstartree_3d(_c: &mut Criterion) {
    info!("Setting up benchmark: knn_rstartree_3d");
    let points = generate_3d_data();
    let mut tree = r_star_tree::RStarTree::<Point3D<i32>>::new(BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let target = Point3D::new(35.0, 45.0, 35.0, None);
    let mut cc = configure_criterion();
    let name = "knn_rstartree_3d";
    cc.bench_function(name, |b| {
        b.iter(|| {
            info!("Running knn search benchmark: {}", name);
            let res = tree.knn_search::<EuclideanDistance>(&target, BENCH_KNN_SIZE);
            info!("Completed knn search benchmark: {}", name);
            black_box(res)
        })
    });
}

criterion_group!(
    benches,
    benchmark_knn_kdtree_2d,
    benchmark_knn_rtree_2d,
    benchmark_knn_quadtree_2d,
    benchmark_knn_kdtree_3d,
    benchmark_knn_rtree_3d,
    benchmark_knn_octree_3d,
    benchmark_knn_rstartree_2d,
    benchmark_knn_rstartree_3d
);
