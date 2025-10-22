#[path = "shared.rs"]
mod shared;
use shared::*;

use criterion::{criterion_group, Criterion};
use spart::geometry::Rectangle;
use spart::{kdtree, octree, quadtree, rstar_tree, rtree};
use std::hint::black_box;

fn bench_insert_bulk_quadtree_2d(_c: &mut Criterion) {
    let points = generate_2d_data();
    let boundary = Rectangle {
        x: BENCH_BOUNDARY.x,
        y: BENCH_BOUNDARY.y,
        width: BENCH_BOUNDARY.width,
        height: BENCH_BOUNDARY.height,
    };
    let mut cc = configure_criterion();
    cc.bench_function("insert_bulk_2d_quadtree", |b| {
        b.iter_with_setup(
            || {
                let tree = quadtree::Quadtree::new(&boundary, BENCH_NODE_CAPACITY).unwrap();
                (tree, points.clone())
            },
            |(mut tree, points)| {
                tree.insert_bulk(&points);
                black_box(());
            },
        )
    });
}

fn bench_insert_bulk_octree_3d(_c: &mut Criterion) {
    let points = generate_3d_data();
    let boundary = BENCH_BOUNDARY;
    let mut cc = configure_criterion();
    cc.bench_function("insert_bulk_3d_octree", |b| {
        b.iter_with_setup(
            || {
                let tree = octree::Octree::new(&boundary, BENCH_NODE_CAPACITY).unwrap();
                (tree, points.clone())
            },
            |(mut tree, points)| {
                tree.insert_bulk(&points);
                black_box(());
            },
        )
    });
}

fn bench_insert_bulk_kdtree_2d(_c: &mut Criterion) {
    let points = generate_2d_data();
    let mut cc = configure_criterion();
    cc.bench_function("insert_bulk_2d_kdtree", |b| {
        b.iter_with_setup(
            || {
                let tree = kdtree::KdTree::new();
                (tree, points.clone())
            },
            |(mut tree, points)| {
                _ = black_box(tree.insert_bulk(points));
            },
        )
    });
}

fn bench_insert_bulk_kdtree_3d(_c: &mut Criterion) {
    let points = generate_3d_data();
    let mut cc = configure_criterion();
    cc.bench_function("insert_bulk_3d_kdtree", |b| {
        b.iter_with_setup(
            || {
                let tree = kdtree::KdTree::new();
                (tree, points.clone())
            },
            |(mut tree, points)| {
                _ = black_box(tree.insert_bulk(points));
            },
        )
    });
}

fn bench_insert_bulk_rtree_2d(_c: &mut Criterion) {
    let points = generate_2d_data();
    let mut cc = configure_criterion();
    cc.bench_function("insert_bulk_2d_rtree", |b| {
        b.iter_with_setup(
            || {
                let tree = rtree::RTree::new(BENCH_NODE_CAPACITY).unwrap();
                (tree, points.clone())
            },
            |(mut tree, points)| {
                tree.insert_bulk(points);
                black_box(());
            },
        )
    });
}

fn bench_insert_bulk_rtree_3d(_c: &mut Criterion) {
    let points = generate_3d_data();
    let mut cc = configure_criterion();
    cc.bench_function("insert_bulk_3d_rtree", |b| {
        b.iter_with_setup(
            || {
                let tree = rtree::RTree::new(BENCH_NODE_CAPACITY).unwrap();
                (tree, points.clone())
            },
            |(mut tree, points)| {
                tree.insert_bulk(points);
                black_box(());
            },
        )
    });
}

fn bench_insert_bulk_rstartree_2d(_c: &mut Criterion) {
    let points = generate_2d_data();
    let mut cc = configure_criterion();
    cc.bench_function("insert_bulk_2d_rstartree", |b| {
        b.iter_with_setup(
            || {
                let tree = rstar_tree::RStarTree::new(BENCH_NODE_CAPACITY).unwrap();
                (tree, points.clone())
            },
            |(mut tree, points)| {
                tree.insert_bulk(points);
                black_box(());
            },
        )
    });
}

fn bench_insert_bulk_rstartree_3d(_c: &mut Criterion) {
    let points = generate_3d_data();
    let mut cc = configure_criterion();
    cc.bench_function("insert_bulk_3d_rstartree", |b| {
        b.iter_with_setup(
            || {
                let tree = rstar_tree::RStarTree::new(BENCH_NODE_CAPACITY).unwrap();
                (tree, points.clone())
            },
            |(mut tree, points)| {
                tree.insert_bulk(points);
                black_box(());
            },
        )
    });
}

criterion_group!(
    benches,
    bench_insert_bulk_quadtree_2d,
    bench_insert_bulk_octree_3d,
    bench_insert_bulk_kdtree_2d,
    bench_insert_bulk_kdtree_3d,
    bench_insert_bulk_rtree_2d,
    bench_insert_bulk_rtree_3d,
    bench_insert_bulk_rstartree_2d,
    bench_insert_bulk_rstartree_3d
);
