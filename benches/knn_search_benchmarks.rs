#[path = "shared.rs"]
mod shared;
use shared::*;

use criterion::{black_box, criterion_group, Criterion};
use spart::bsp_tree::{Point2DBSP, Point3DBSP};
use spart::geometry::{Point2D, Point3D, Rectangle};
use spart::{bsp_tree, kd_tree, octree, quadtree, r_tree};
use tracing::info;

/// Configures Criterion using the shared benchmark timeout.
fn configure_criterion() -> Criterion {
    Criterion::default().measurement_time(BENCH_TIMEOUT)
}

/// A generic helper function for knn search benchmarks.
/// The lifetime `'a` ties the returned value's lifetime to that of the tree reference.
fn bench_knn_search<'a, T, Q, R>(
    name: &str,
    tree: &'a T,
    query: &Q,
    search_fn: impl Fn(&'a T, &Q, usize) -> R,
    cc: &mut Criterion,
) where
    R: 'a,
{
    cc.bench_function(name, |b| {
        b.iter(|| {
            info!("Running knn search benchmark: {}", name);
            let res = search_fn(tree, query, BENCH_KNN_SIZE);
            info!("Completed knn search benchmark: {}", name);
            black_box(res)
        })
    });
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
    bench_knn_search(
        "knn_kdtree_2d",
        &tree,
        &target,
        |t, q, k| t.knn_search(q, k),
        &mut cc,
    );
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
    bench_knn_search(
        "knn_rtree_2d",
        &tree,
        &target,
        |t, q, k| t.knn_search(q, k),
        &mut cc,
    );
}

fn benchmark_knn_bsptree_2d(_c: &mut Criterion) {
    info!("Setting up benchmark: knn_bsptree_2d");
    let points = generate_2d_data_wrapped();
    let mut tree = bsp_tree::BSPTree::<Point2DBSP<i32>>::new(BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let target: Point2DBSP<i32> = Point2DBSP {
        point: Point2D::new(35.0, 45.0, None),
    };
    let mut cc = configure_criterion();
    bench_knn_search(
        "knn_bsptree_2d",
        &tree,
        &target,
        |t, q, k| t.knn_search(q, k),
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
    let mut tree = quadtree::Quadtree::new(&boundary, BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let target = Point2D::new(35.0, 45.0, None);
    let mut cc = configure_criterion();
    bench_knn_search(
        "knn_quadtree_2d",
        &tree,
        &target,
        |t, q, k| t.knn_search(q, k),
        &mut cc,
    );
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
    bench_knn_search(
        "knn_kdtree_3d",
        &tree,
        &target,
        |t, q, k| t.knn_search(q, k),
        &mut cc,
    );
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
    bench_knn_search(
        "knn_rtree_3d",
        &tree,
        &target,
        |t, q, k| t.knn_search(q, k),
        &mut cc,
    );
}

fn benchmark_knn_bsptree_3d(_c: &mut Criterion) {
    info!("Setting up benchmark: knn_bsptree_3d");
    let points = generate_3d_data_wrapped();
    let mut tree = bsp_tree::BSPTree::<Point3DBSP<i32>>::new(BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let target: Point3DBSP<i32> = Point3DBSP {
        point: Point3D::new(35.0, 45.0, 35.0, None),
    };
    let mut cc = configure_criterion();
    bench_knn_search(
        "knn_bsptree_3d",
        &tree,
        &target,
        |t, q, k| t.knn_search(q, k),
        &mut cc,
    );
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
    bench_knn_search(
        "knn_octree_3d",
        &tree,
        &target,
        |t, q, k| t.knn_search(q, k),
        &mut cc,
    );
}

criterion_group!(
    benches,
    benchmark_knn_kdtree_2d,
    benchmark_knn_rtree_2d,
    benchmark_knn_bsptree_2d,
    benchmark_knn_quadtree_2d,
    benchmark_knn_kdtree_3d,
    benchmark_knn_rtree_3d,
    benchmark_knn_bsptree_3d,
    benchmark_knn_octree_3d,
);
