#[path = "shared.rs"]
mod shared;
use shared::*;

use criterion::{Criterion, criterion_group};
use spart::geometry::Rectangle;
use spart::{kdtree, octree, quadtree, rstar_tree, rtree};
use std::hint::black_box;

fn benchmark_delete_quadtree_2d(_c: &mut Criterion) {
    let points = generate_2d_data();
    let point_to_delete = points[points.len() / 2].clone();
    let boundary = Rectangle {
        x: BENCH_BOUNDARY.x,
        y: BENCH_BOUNDARY.y,
        width: BENCH_BOUNDARY.width,
        height: BENCH_BOUNDARY.height,
    };
    let mut cc = configure_criterion();
    cc.bench_function("delete_2d_quadtree", |b| {
        b.iter_with_setup(
            || {
                let mut tree = quadtree::Quadtree::new(&boundary, BENCH_NODE_CAPACITY).unwrap();
                for p in points.clone() {
                    tree.insert(p);
                }
                tree
            },
            |mut tree| {
                black_box(tree.delete(&point_to_delete));
            },
        )
    });
}

fn benchmark_delete_octree_3d(_c: &mut Criterion) {
    let points = generate_3d_data();
    let point_to_delete = points[points.len() / 2].clone();
    let boundary = BENCH_BOUNDARY;
    let mut cc = configure_criterion();
    cc.bench_function("delete_3d_octree", |b| {
        b.iter_with_setup(
            || {
                let mut tree = octree::Octree::new(&boundary, BENCH_NODE_CAPACITY).unwrap();
                for p in points.clone() {
                    tree.insert(p);
                }
                tree
            },
            |mut tree| {
                black_box(tree.delete(&point_to_delete));
            },
        )
    });
}

fn benchmark_delete_kdtree_2d(_c: &mut Criterion) {
    let points = generate_2d_data();
    let point_to_delete = points[points.len() / 2].clone();
    let mut cc = configure_criterion();
    cc.bench_function("delete_2d_kdtree", |b| {
        b.iter_with_setup(
            || {
                let mut tree = kdtree::KdTree::new();
                for p in points.clone() {
                    _ = tree.insert(p);
                }
                tree
            },
            |mut tree| {
                black_box(tree.delete(&point_to_delete));
            },
        )
    });
}

fn benchmark_delete_kdtree_3d(_c: &mut Criterion) {
    let points = generate_3d_data();
    let point_to_delete = points[points.len() / 2].clone();
    let mut cc = configure_criterion();
    cc.bench_function("delete_3d_kdtree", |b| {
        b.iter_with_setup(
            || {
                let mut tree = kdtree::KdTree::new();
                for p in points.clone() {
                    _ = tree.insert(p);
                }
                tree
            },
            |mut tree| {
                black_box(tree.delete(&point_to_delete));
            },
        )
    });
}

fn benchmark_delete_rtree_2d(_c: &mut Criterion) {
    let points = generate_2d_data();
    let point_to_delete = points[points.len() / 2].clone();
    let mut cc = configure_criterion();
    cc.bench_function("delete_2d_rtree", |b| {
        b.iter_with_setup(
            || {
                let mut tree = rtree::RTree::new(BENCH_NODE_CAPACITY).unwrap();
                for p in points.clone() {
                    tree.insert(p);
                }
                tree
            },
            |mut tree| {
                black_box(tree.delete(&point_to_delete));
            },
        )
    });
}

fn benchmark_delete_rtree_3d(_c: &mut Criterion) {
    let points = generate_3d_data();
    let point_to_delete = points[points.len() / 2].clone();
    let mut cc = configure_criterion();
    cc.bench_function("delete_3d_rtree", |b| {
        b.iter_with_setup(
            || {
                let mut tree = rtree::RTree::new(BENCH_NODE_CAPACITY).unwrap();
                for p in points.clone() {
                    tree.insert(p);
                }
                tree
            },
            |mut tree| {
                black_box(tree.delete(&point_to_delete));
            },
        )
    });
}

fn benchmark_delete_rstartree_2d(_c: &mut Criterion) {
    let points = generate_2d_data();
    let point_to_delete = points[points.len() / 2].clone();
    let mut cc = configure_criterion();
    cc.bench_function("delete_2d_rstartree", |b| {
        b.iter_with_setup(
            || {
                let mut tree = rstar_tree::RStarTree::new(BENCH_NODE_CAPACITY).unwrap();
                for p in points.clone() {
                    tree.insert(p);
                }
                tree
            },
            |mut tree| {
                black_box(tree.delete(&point_to_delete));
            },
        )
    });
}

fn benchmark_delete_rstartree_3d(_c: &mut Criterion) {
    let points = generate_3d_data();
    let point_to_delete = points[points.len() / 2].clone();
    let mut cc = configure_criterion();
    cc.bench_function("delete_3d_rstartree", |b| {
        b.iter_with_setup(
            || {
                let mut tree = rstar_tree::RStarTree::new(BENCH_NODE_CAPACITY).unwrap();
                for p in points.clone() {
                    tree.insert(p);
                }
                tree
            },
            |mut tree| {
                black_box(tree.delete(&point_to_delete));
            },
        )
    });
}

criterion_group!(
    benches,
    benchmark_delete_quadtree_2d,
    benchmark_delete_octree_3d,
    benchmark_delete_kdtree_2d,
    benchmark_delete_kdtree_3d,
    benchmark_delete_rtree_2d,
    benchmark_delete_rtree_3d,
    benchmark_delete_rstartree_2d,
    benchmark_delete_rstartree_3d
);
