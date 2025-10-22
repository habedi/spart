#[path = "shared.rs"]
mod shared;
use shared::*;

use criterion::{criterion_group, Criterion};
use spart::geometry::{EuclideanDistance, Point2D, Point3D, Rectangle};
use spart::{kdtree, octree, quadtree, rstar_tree, rtree};
use std::hint::black_box;
use tracing::info;

fn bench_knn_search<'a, T, P, R>(
    name: &str,
    tree: &'a T,
    query: &P,
    search_fn: impl Fn(&'a T, &P, usize) -> R,
    cc: &mut Criterion,
) where
    R: 'a,
{
    cc.bench_function(name, |b| {
        b.iter(|| {
            let res = search_fn(tree, query, BENCH_KNN_SIZE);
            black_box(res)
        })
    });
}

fn benchmark_knn_kdtree_2d(_c: &mut Criterion) {
    info!("Setting up benchmark: knn_kdtree_2d");
    let points = generate_2d_data();
    let mut tree = kdtree::KdTree::<Point2D<i32>>::new();
    for point in points.iter() {
        _ = tree.insert(point.clone());
    }
    let target = Point2D::new(35.0, 45.0, None);
    let mut cc = configure_criterion();
    bench_knn_search(
        "knn_kdtree_2d",
        &tree,
        &target,
        |t, q, k| t.knn_search::<EuclideanDistance>(q, k),
        &mut cc,
    );
}

fn benchmark_knn_rtree_2d(_c: &mut Criterion) {
    info!("Setting up benchmark: knn_rtree_2d");
    let points = generate_2d_data();
    let mut tree = rtree::RTree::<Point2D<i32>>::new(BENCH_NODE_CAPACITY).unwrap();
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let target = Point2D::new(35.0, 45.0, None);
    let mut cc = configure_criterion();
    bench_knn_search(
        "knn_rtree_2d",
        &tree,
        &target,
        |t, q, k| t.knn_search::<EuclideanDistance>(q, k),
        &mut cc,
    );
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
    let mut tree = quadtree::Quadtree::new(&boundary, BENCH_NODE_CAPACITY).unwrap();
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let target = Point2D::new(35.0, 45.0, None);
    let mut cc = configure_criterion();
    bench_knn_search(
        "knn_quadtree_2d",
        &tree,
        &target,
        |t, q, k| t.knn_search::<EuclideanDistance>(q, k),
        &mut cc,
    );
}

fn benchmark_knn_kdtree_3d(_c: &mut Criterion) {
    info!("Setting up benchmark: knn_kdtree_3d");
    let points = generate_3d_data();
    let mut tree = kdtree::KdTree::<Point3D<i32>>::new();
    for point in points.iter() {
        _ = tree.insert(point.clone());
    }
    let target = Point3D::new(35.0, 45.0, 35.0, None);
    let mut cc = configure_criterion();
    bench_knn_search(
        "knn_kdtree_3d",
        &tree,
        &target,
        |t, q, k| t.knn_search::<EuclideanDistance>(q, k),
        &mut cc,
    );
}

fn benchmark_knn_rtree_3d(_c: &mut Criterion) {
    info!("Setting up benchmark: knn_rtree_3d");
    let points = generate_3d_data();
    let mut tree = rtree::RTree::<Point3D<i32>>::new(BENCH_NODE_CAPACITY).unwrap();
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let target = Point3D::new(35.0, 45.0, 35.0, None);
    let mut cc = configure_criterion();
    bench_knn_search(
        "knn_rtree_3d",
        &tree,
        &target,
        |t, q, k| t.knn_search::<EuclideanDistance>(q, k),
        &mut cc,
    );
}

fn benchmark_knn_octree_3d(_c: &mut Criterion) {
    info!("Setting up benchmark: knn_octree_3d");
    let points = generate_3d_data();
    let boundary = BENCH_BOUNDARY;
    let mut tree = octree::Octree::new(&boundary, BENCH_NODE_CAPACITY).unwrap();
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let target = Point3D::new(35.0, 45.0, 35.0, None);
    let mut cc = configure_criterion();
    bench_knn_search(
        "knn_octree_3d",
        &tree,
        &target,
        |t, q, k| t.knn_search::<EuclideanDistance>(q, k),
        &mut cc,
    );
}

fn benchmark_knn_rstartree_2d(_c: &mut Criterion) {
    info!("Setting up benchmark: knn_rstartree_2d");
    let points = generate_2d_data();
    let mut tree = rstar_tree::RStarTree::<Point2D<i32>>::new(BENCH_NODE_CAPACITY).unwrap();
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let target = Point2D::new(35.0, 45.0, None);
    let mut cc = configure_criterion();
    bench_knn_search(
        "knn_rstartree_2d",
        &tree,
        &target,
        |t, q, k| t.knn_search::<EuclideanDistance>(q, k),
        &mut cc,
    );
}

fn benchmark_knn_rstartree_3d(_c: &mut Criterion) {
    info!("Setting up benchmark: knn_rstartree_3d");
    let points = generate_3d_data();
    let mut tree = rstar_tree::RStarTree::<Point3D<i32>>::new(BENCH_NODE_CAPACITY).unwrap();
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let target = Point3D::new(35.0, 45.0, 35.0, None);
    let mut cc = configure_criterion();
    bench_knn_search(
        "knn_rstartree_3d",
        &tree,
        &target,
        |t, q, k| t.knn_search::<EuclideanDistance>(q, k),
        &mut cc,
    );
}

criterion_group!(
    name = benches;
    config = configure_criterion();
    targets =
    benchmark_knn_kdtree_2d,
    benchmark_knn_rtree_2d,
    benchmark_knn_quadtree_2d,
    benchmark_knn_kdtree_3d,
    benchmark_knn_rtree_3d,
    benchmark_knn_octree_3d,
    benchmark_knn_rstartree_2d,
    benchmark_knn_rstartree_3d
);
