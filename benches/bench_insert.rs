#[path = "shared.rs"]
mod shared;
use shared::*;

use criterion::{criterion_group, Criterion};
use spart::geometry::Rectangle;
use spart::{kd_tree, octree, quadtree, r_star_tree, r_tree};
use std::hint::black_box;

fn bench_insert<'a, T, P>(
    c: &mut Criterion,
    name: &str,
    mut setup: impl FnMut() -> (T, P),
    mut insert: impl FnMut(&mut T, P),
) where
    T: 'a,
    P: Clone,
{
    c.bench_function(name, |b| {
        b.iter_with_setup(&mut setup, |(mut tree, point)| {
            insert(&mut tree, point.clone());
            black_box(tree);
        })
    });
}

fn bench_insert_quadtree_2d(c: &mut Criterion) {
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
    bench_insert(
        c,
        "insert_2d_quadtree",
        || {
            let mut tree = quadtree::Quadtree::new(&boundary, BENCH_NODE_CAPACITY).unwrap();
            for p in base_points.clone() {
                tree.insert(p);
            }
            (tree, to_insert.clone())
        },
        |tree, point| {
            tree.insert(point);
        },
    );
}

fn bench_insert_octree_3d(c: &mut Criterion) {
    let points = generate_3d_data();
    let to_insert = points[points.len() - 1].clone();
    let mut base_points = points.clone();
    base_points.pop();
    let boundary = BENCH_BOUNDARY;
    bench_insert(
        c,
        "insert_3d_octree",
        || {
            let mut tree = octree::Octree::new(&boundary, BENCH_NODE_CAPACITY).unwrap();
            for p in base_points.clone() {
                tree.insert(p);
            }
            (tree, to_insert.clone())
        },
        |tree, point| {
            tree.insert(point);
        },
    );
}

fn bench_insert_kdtree_2d(c: &mut Criterion) {
    let points = generate_2d_data();
    let to_insert = points[points.len() - 1].clone();
    let mut base_points = points.clone();
    base_points.pop();
    bench_insert(
        c,
        "insert_2d_kdtree",
        || {
            let mut tree = kd_tree::KdTree::new();
            for p in base_points.clone() {
                let _ = tree.insert(p);
            }
            (tree, to_insert.clone())
        },
        |tree, point| {
            let _ = tree.insert(point);
        },
    );
}

fn bench_insert_kdtree_3d(c: &mut Criterion) {
    let points = generate_3d_data();
    let to_insert = points[points.len() - 1].clone();
    let mut base_points = points.clone();
    base_points.pop();
    bench_insert(
        c,
        "insert_3d_kdtree",
        || {
            let mut tree = kd_tree::KdTree::new();
            for p in base_points.clone() {
                let _ = tree.insert(p);
            }
            (tree, to_insert.clone())
        },
        |tree, point| {
            let _ = tree.insert(point);
        },
    );
}

fn bench_insert_rtree_2d(c: &mut Criterion) {
    let points = generate_2d_data();
    let to_insert = points[points.len() - 1].clone();
    let mut base_points = points.clone();
    base_points.pop();
    bench_insert(
        c,
        "insert_2d_rtree",
        || {
            let mut tree = r_tree::RTree::new(BENCH_NODE_CAPACITY).unwrap();
            for p in base_points.clone() {
                tree.insert(p);
            }
            (tree, to_insert.clone())
        },
        |tree, point| {
            tree.insert(point);
        },
    );
}

fn bench_insert_rtree_3d(c: &mut Criterion) {
    let points = generate_3d_data();
    let to_insert = points[points.len() - 1].clone();
    let mut base_points = points.clone();
    base_points.pop();
    bench_insert(
        c,
        "insert_3d_rtree",
        || {
            let mut tree = r_tree::RTree::new(BENCH_NODE_CAPACITY).unwrap();
            for p in base_points.clone() {
                tree.insert(p);
            }
            (tree, to_insert.clone())
        },
        |tree, point| {
            tree.insert(point);
        },
    );
}

fn bench_insert_rstartree_2d(c: &mut Criterion) {
    let points = generate_2d_data();
    let to_insert = points[points.len() - 1].clone();
    let mut base_points = points.clone();
    base_points.pop();
    bench_insert(
        c,
        "insert_2d_rstartree",
        || {
            let mut tree = r_star_tree::RStarTree::new(BENCH_NODE_CAPACITY).unwrap();
            for p in base_points.clone() {
                tree.insert(p);
            }
            (tree, to_insert.clone())
        },
        |tree, point| {
            tree.insert(point);
        },
    );
}

fn bench_insert_rstartree_3d(c: &mut Criterion) {
    let points = generate_3d_data();
    let to_insert = points[points.len() - 1].clone();
    let mut base_points = points.clone();
    base_points.pop();
    bench_insert(
        c,
        "insert_3d_rstartree",
        || {
            let mut tree = r_star_tree::RStarTree::new(BENCH_NODE_CAPACITY).unwrap();
            for p in base_points.clone() {
                tree.insert(p);
            }
            (tree, to_insert.clone())
        },
        |tree, point| {
            tree.insert(point);
        },
    );
}

criterion_group!(
    name = benches;
    config = configure_criterion();
    targets =
    bench_insert_quadtree_2d,
    bench_insert_octree_3d,
    bench_insert_kdtree_2d,
    bench_insert_kdtree_3d,
    bench_insert_rtree_2d,
    bench_insert_rtree_3d,
    bench_insert_rstartree_2d,
    bench_insert_rstartree_3d
);
