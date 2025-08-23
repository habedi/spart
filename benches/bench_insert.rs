#[path = "shared.rs"]
mod shared;
use shared::*;

use criterion::{criterion_group, Criterion};
use spart::geometry::Rectangle;
use spart::{kd_tree, octree, quadtree, r_star_tree, r_tree};
use std::hint::black_box;

fn bench_insert_quadtree_2d(_c: &mut Criterion) {
    let points = generate_2d_data();
    let to_insert = points[points.len() - 1].clone();
    let mut base_points = points.clone();
    base_points.pop();
    let boundary = Rectangle {
        x: BENCH_BOUNDARY.x,
        y: BENCH_BOUNDARY.y,
        width: BENCH_BOUNDARY.width,
        height: BENCH_BOUNDARY.height,
    };
    let mut cc = configure_criterion();
    cc.bench_function("insert_2d_quadtree", |b| {
        b.iter_with_setup(
            || {
                let mut tree = quadtree::Quadtree::new(&boundary, BENCH_NODE_CAPACITY).unwrap();
                for p in base_points.clone() {
                    tree.insert(p);
                }
                tree
            },
            |mut tree| {
                black_box(tree.insert(to_insert.clone()));
            },
        )
    });
}

fn bench_insert_octree_3d(_c: &mut Criterion) {
    let points = generate_3d_data();
    let to_insert = points[points.len() - 1].clone();
    let mut base_points = points.clone();
    base_points.pop();
    let boundary = BENCH_BOUNDARY;
    let mut cc = configure_criterion();
    cc.bench_function("insert_3d_octree", |b| {
        b.iter_with_setup(
            || {
                let mut tree = octree::Octree::new(&boundary, BENCH_NODE_CAPACITY).unwrap();
                for p in base_points.clone() {
                    tree.insert(p);
                }
                tree
            },
            |mut tree| {
                black_box(tree.insert(to_insert.clone()));
            },
        )
    });
}

fn bench_insert_kdtree_2d(_c: &mut Criterion) {
    let points = generate_2d_data();
    let to_insert = points[points.len() - 1].clone();
    let mut base_points = points.clone();
    base_points.pop();
    let mut cc = configure_criterion();
    cc.bench_function("insert_2d_kdtree", |b| {
        b.iter_with_setup(
            || {
                let mut tree = kd_tree::KdTree::new();
                for p in base_points.clone() {
                    let _ = tree.insert(p);
                }
                tree
            },
            |mut tree| {
                _ = black_box(tree.insert(to_insert.clone()));
            },
        )
    });
}

fn bench_insert_kdtree_3d(_c: &mut Criterion) {
    let points = generate_3d_data();
    let to_insert = points[points.len() - 1].clone();
    let mut base_points = points.clone();
    base_points.pop();
    let mut cc = configure_criterion();
    cc.bench_function("insert_3d_kdtree", |b| {
        b.iter_with_setup(
            || {
                let mut tree = kd_tree::KdTree::new();
                for p in base_points.clone() {
                    let _ = tree.insert(p);
                }
                tree
            },
            |mut tree| {
                _ = black_box(tree.insert(to_insert.clone()));
            },
        )
    });
}

fn bench_insert_rtree_2d(_c: &mut Criterion) {
    let points = generate_2d_data();
    let to_insert = points[points.len() - 1].clone();
    let mut base_points = points.clone();
    base_points.pop();
    let mut cc = configure_criterion();
    cc.bench_function("insert_2d_rtree", |b| {
        b.iter_with_setup(
            || {
                let mut tree = r_tree::RTree::new(BENCH_NODE_CAPACITY).unwrap();
                for p in base_points.clone() {
                    tree.insert(p);
                }
                tree
            },
            |mut tree| {
                black_box(tree.insert(to_insert.clone()));
            },
        )
    });
}

fn bench_insert_rtree_3d(_c: &mut Criterion) {
    let points = generate_3d_data();
    let to_insert = points[points.len() - 1].clone();
    let mut base_points = points.clone();
    base_points.pop();
    let mut cc = configure_criterion();
    cc.bench_function("insert_3d_rtree", |b| {
        b.iter_with_setup(
            || {
                let mut tree = r_tree::RTree::new(BENCH_NODE_CAPACITY).unwrap();
                for p in base_points.clone() {
                    tree.insert(p);
                }
                tree
            },
            |mut tree| {
                black_box(tree.insert(to_insert.clone()));
            },
        )
    });
}

fn bench_insert_rstartree_2d(_c: &mut Criterion) {
    let points = generate_2d_data();
    let to_insert = points[points.len() - 1].clone();
    let mut base_points = points.clone();
    base_points.pop();
    let mut cc = configure_criterion();
    cc.bench_function("insert_2d_rstartree", |b| {
        b.iter_with_setup(
            || {
                let mut tree = r_star_tree::RStarTree::new(BENCH_NODE_CAPACITY).unwrap();
                for p in base_points.clone() {
                    tree.insert(p);
                }
                tree
            },
            |mut tree| {
                black_box(tree.insert(to_insert.clone()));
            },
        )
    });
}

fn bench_insert_rstartree_3d(_c: &mut Criterion) {
    let points = generate_3d_data();
    let to_insert = points[points.len() - 1].clone();
    let mut base_points = points.clone();
    base_points.pop();
    let mut cc = configure_criterion();
    cc.bench_function("insert_3d_rstartree", |b| {
        b.iter_with_setup(
            || {
                let mut tree = r_star_tree::RStarTree::new(BENCH_NODE_CAPACITY).unwrap();
                for p in base_points.clone() {
                    tree.insert(p);
                }
                tree
            },
            |mut tree| {
                black_box(tree.insert(to_insert.clone()));
            },
        )
    });
}

criterion_group!(
    benches,
    bench_insert_quadtree_2d,
    bench_insert_octree_3d,
    bench_insert_kdtree_2d,
    bench_insert_kdtree_3d,
    bench_insert_rtree_2d,
    bench_insert_rtree_3d,
    bench_insert_rstartree_2d,
    bench_insert_rstartree_3d
);
