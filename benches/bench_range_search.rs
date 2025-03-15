#[path = "shared.rs"]
mod shared;
use shared::*;

use criterion::{black_box, criterion_group, Criterion};
use spart::bsp_tree::{Point2DBSP, Point3DBSP};
use spart::geometry::{Cube, Point2D, Point3D, Rectangle};
use spart::{bsp_tree, kd_tree, octree, quadtree, r_tree};
use tracing::info;

// Configure Criterion with our benchmark timeout.
pub fn configure_criterion() -> Criterion {
    Criterion::default().measurement_time(BENCH_TIMEOUT)
}

/// A generic helper function for range search benchmarks.
///
/// The lifetime `'a` ties the lifetime of the tree reference and the return value.
/// The closure `search_fn` must return a value whose lifetime is at least `'a`.
fn bench_range_search<'a, T, Q, R>(
    name: &str,
    tree: &'a T,
    query: &Q,
    search_fn: impl Fn(&'a T, &Q, f64) -> R,
    cc: &mut Criterion,
) where
    R: 'a,
{
    cc.bench_function(name, |b| {
        b.iter(|| {
            info!("Running range search benchmark: {}", name);
            let res = search_fn(tree, query, BENCH_RANGE_RADIUS);
            info!("Completed range search benchmark: {}", name);
            black_box(res)
        })
    });
}

fn benchmark_range_kdtree_2d(_c: &mut Criterion) {
    info!("Setting up benchmark_range_kdtree_2d");
    let points = generate_2d_data();
    let mut tree = kd_tree::KdTree::<Point2D<i32>>::new(2);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let query = Point2D::new(35.0, 45.0, None);
    let mut cc = configure_criterion();
    bench_range_search(
        "range_kdtree_2d",
        &tree,
        &query,
        |t, q, r| t.range_search(q, r),
        &mut cc,
    );
}

fn benchmark_range_bbox_rtree_2d(_c: &mut Criterion) {
    info!("Setting up benchmark_range_bbox_rtree_2d");
    let points = generate_2d_data();
    let mut tree = r_tree::RTree::<Point2D<i32>>::new(BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let query_rect = Rectangle {
        x: 35.0 - BENCH_RANGE_RADIUS,
        y: 45.0 - BENCH_RANGE_RADIUS,
        width: 2.0 * BENCH_RANGE_RADIUS,
        height: 2.0 * BENCH_RANGE_RADIUS,
    };
    let mut cc = configure_criterion();
    bench_range_search(
        "range_rtree_bbox_2d",
        &tree,
        &query_rect,
        |t, q, _| t.range_search_bbox(q),
        &mut cc,
    );
}

fn benchmark_range_rtree_2d(_c: &mut Criterion) {
    info!("Setting up benchmark_range_rtree_2d");
    let points = generate_2d_data();
    let mut tree = r_tree::RTree::<Point2D<i32>>::new(BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let query_point = Point2D {
        x: 35.0,
        y: 45.0,
        data: None,
    };
    let mut cc = configure_criterion();
    bench_range_search(
        "range_rtree_2d",
        &tree,
        &query_point,
        |t, q, r| t.range_search(q, r),
        &mut cc,
    );
}

fn benchmark_range_bbox_bsptree_2d(_c: &mut Criterion) {
    info!("Setting up benchmark_range_bbox_bsptree_2d");
    let points = generate_2d_data_wrapped();
    let mut tree = bsp_tree::BSPTree::<Point2DBSP<i32>>::new(BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let query_rect = Rectangle {
        x: 35.0 - BENCH_RANGE_RADIUS,
        y: 45.0 - BENCH_RANGE_RADIUS,
        width: 2.0 * BENCH_RANGE_RADIUS,
        height: 2.0 * BENCH_RANGE_RADIUS,
    };
    let mut cc = configure_criterion();
    bench_range_search(
        "range_bsptree_bbox_2d",
        &tree,
        &query_rect,
        |t, q, _| t.range_search_bbox(q),
        &mut cc,
    );
}

fn benchmark_range_bsptree_2d(_c: &mut Criterion) {
    info!("Setting up benchmark_range_bsptree_2d");
    let points = generate_2d_data_wrapped();
    let mut tree = bsp_tree::BSPTree::<Point2DBSP<i32>>::new(BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let query_point = Point2DBSP {
        point: Point2D {
            x: 35.0,
            y: 45.0,
            data: None,
        },
    };
    let mut cc = configure_criterion();
    bench_range_search(
        "range_bsptree_2d",
        &tree,
        &query_point,
        |t, q, r| t.range_search(q, r),
        &mut cc,
    );
}

fn benchmark_range_quadtree_2d(_c: &mut Criterion) {
    info!("Setting up benchmark_range_quadtree_2d");
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
    let query = Point2D::new(35.0, 45.0, None);
    let mut cc = configure_criterion();
    bench_range_search(
        "range_quadtree_2d",
        &tree,
        &query,
        |t, q, r| t.range_search(q, r),
        &mut cc,
    );
}

fn benchmark_range_kdtree_3d(_c: &mut Criterion) {
    info!("Setting up benchmark_range_kdtree_3d");
    let points = generate_3d_data();
    let mut tree = kd_tree::KdTree::<Point3D<i32>>::new(3);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let query = Point3D::new(35.0, 45.0, 35.0, None);
    let mut cc = configure_criterion();
    bench_range_search(
        "range_kdtree_3d",
        &tree,
        &query,
        |t, q, r| t.range_search(q, r),
        &mut cc,
    );
}

fn benchmark_range_bbox_rtree_3d(_c: &mut Criterion) {
    info!("Setting up benchmark_range_bbox_rtree_3d");
    let points = generate_3d_data();
    let mut tree = r_tree::RTree::<Point3D<i32>>::new(BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let query_cube = Cube {
        x: 35.0 - BENCH_RANGE_RADIUS,
        y: 45.0 - BENCH_RANGE_RADIUS,
        z: 35.0 - BENCH_RANGE_RADIUS,
        width: 2.0 * BENCH_RANGE_RADIUS,
        height: 2.0 * BENCH_RANGE_RADIUS,
        depth: 2.0 * BENCH_RANGE_RADIUS,
    };
    let mut cc = configure_criterion();
    bench_range_search(
        "range_rtree_bbox_3d",
        &tree,
        &query_cube,
        |t, q, _| t.range_search_bbox(q),
        &mut cc,
    );
}

fn benchmark_range_rtree_3d(_c: &mut Criterion) {
    info!("Setting up benchmark_range_rtree_3d");
    let points = generate_3d_data();
    let mut tree = r_tree::RTree::<Point3D<i32>>::new(BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let query_point = Point3D {
        x: 35.0,
        y: 45.0,
        z: 35.0,
        data: None,
    };
    let mut cc = configure_criterion();
    bench_range_search(
        "range_rtree_3d",
        &tree,
        &query_point,
        |t, q, r| t.range_search(q, r),
        &mut cc,
    );
}

fn benchmark_range_bbox_bsptree_3d(_c: &mut Criterion) {
    info!("Setting up benchmark_range_bbox_bsptree_3d");
    let points = generate_3d_data_wrapped();
    let mut tree = bsp_tree::BSPTree::<Point3DBSP<i32>>::new(BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let query_cube = Cube {
        x: 35.0 - BENCH_RANGE_RADIUS,
        y: 45.0 - BENCH_RANGE_RADIUS,
        z: 35.0 - BENCH_RANGE_RADIUS,
        width: 2.0 * BENCH_RANGE_RADIUS,
        height: 2.0 * BENCH_RANGE_RADIUS,
        depth: 2.0 * BENCH_RANGE_RADIUS,
    };
    let mut cc = configure_criterion();
    bench_range_search(
        "range_bsptree_bbox_3d",
        &tree,
        &query_cube,
        |t, q, _| t.range_search_bbox(q),
        &mut cc,
    );
}

fn benchmark_range_bsptree_3d(_c: &mut Criterion) {
    info!("Setting up benchmark_range_bsptree_3d");
    let points = generate_3d_data_wrapped();
    let mut tree = bsp_tree::BSPTree::<Point3DBSP<i32>>::new(BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let query_point = Point3DBSP {
        point: Point3D {
            x: 35.0,
            y: 45.0,
            z: 35.0,
            data: None,
        },
    };
    let mut cc = configure_criterion();
    bench_range_search(
        "range_bsptree_3d",
        &tree,
        &query_point,
        |t, q, r| t.range_search(q, r),
        &mut cc,
    );
}

fn benchmark_range_octree_3d(_c: &mut Criterion) {
    info!("Setting up benchmark_range_octree_3d");
    let points = generate_3d_data();
    let boundary = BENCH_BOUNDARY;
    let mut tree = octree::Octree::new(&boundary, BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let query = Point3D::new(35.0, 45.0, 35.0, None);
    let mut cc = configure_criterion();
    bench_range_search(
        "range_octree_3d",
        &tree,
        &query,
        |t, q, r| t.range_search(q, r),
        &mut cc,
    );
}

criterion_group!(
    benches,
    benchmark_range_kdtree_2d,
    benchmark_range_rtree_2d,
    benchmark_range_bbox_rtree_2d,
    benchmark_range_bsptree_2d,
    benchmark_range_bbox_bsptree_2d,
    benchmark_range_quadtree_2d,
    benchmark_range_kdtree_3d,
    benchmark_range_rtree_3d,
    benchmark_range_bbox_rtree_3d,
    benchmark_range_bsptree_3d,
    benchmark_range_bbox_bsptree_3d,
    benchmark_range_octree_3d,
);
