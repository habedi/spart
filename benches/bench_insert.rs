#![allow(dead_code)]
#[path = "shared.rs"]
mod shared;
use shared::*;

use criterion::{criterion_group, Criterion};
use spart::geometry::{Point2D, Point3D, Rectangle};
use spart::{kd_tree, octree, quadtree, r_star_tree, r_tree};
use std::hint::black_box;
use tracing::info;

/// A generic helper that benchmarks an insertion function.
///
/// # Arguments
///
/// * `bench_name` - The name of the benchmark.
/// * `points` - A vector of points to be inserted.
/// * `insert_fn` - A closure that takes a vector of points and performs insertion.
/// * `cc` - A mutable reference to a configured Criterion instance.
fn bench_insert<P: Clone>(
    bench_name: &str,
    points: Vec<P>,
    insert_fn: impl Fn(Vec<P>),
    cc: &mut Criterion,
) {
    cc.bench_function(bench_name, |b| {
        b.iter(|| {
            // Use black_box to avoid optimizations.
            insert_fn(black_box(points.clone()))
        })
    });
}

fn insert_2d_quadtree(points: Vec<Point2D<i32>>) {
    info!("Starting insertion for 2D Quadtree");
    let boundary = Rectangle {
        x: BENCH_BOUNDARY.x,
        y: BENCH_BOUNDARY.y,
        width: BENCH_BOUNDARY.width,
        height: BENCH_BOUNDARY.height,
    };
    let mut tree = quadtree::Quadtree::new(&boundary, BENCH_NODE_CAPACITY).unwrap();
    for point in points {
        tree.insert(point);
    }
    info!("Finished insertion for 2D Quadtree");
}

fn insert_3d_octree(points: Vec<Point3D<i32>>) {
    info!("Starting insertion for 3D Octree");
    let boundary = BENCH_BOUNDARY;
    let mut tree = octree::Octree::new(&boundary, BENCH_NODE_CAPACITY).unwrap();
    for point in points {
        tree.insert(point);
    }
    info!("Finished insertion for 3D Octree");
}

fn insert_2d_kdtree(points: Vec<Point2D<i32>>) {
    info!("Starting insertion for 2D KdTree");
    let mut tree = kd_tree::KdTree::new(2).unwrap();
    for point in points {
        tree.insert(point);
    }
    info!("Finished insertion for 2D KdTree");
}

fn insert_3d_kdtree(points: Vec<Point3D<i32>>) {
    info!("Starting insertion for 3D KdTree");
    let mut tree = kd_tree::KdTree::new(3).unwrap();
    for point in points {
        tree.insert(point);
    }
    info!("Finished insertion for 3D KdTree");
}

fn insert_2d_rtree(points: Vec<Point2D<i32>>) {
    info!("Starting insertion for 2D RTree");
    let mut tree = r_tree::RTree::new(BENCH_NODE_CAPACITY).unwrap();
    for point in points {
        tree.insert(point);
    }
    info!("Finished insertion for 2D RTree");
}

fn insert_3d_rtree(points: Vec<Point3D<i32>>) {
    info!("Starting insertion for 3D RTree");
    let mut tree = r_tree::RTree::new(BENCH_NODE_CAPACITY).unwrap();
    for point in points {
        tree.insert(point);
    }
    info!("Finished insertion for 3D RTree");
}

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
                let mut tree = kd_tree::KdTree::new(2).unwrap();
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

fn bench_insert_kdtree_3d(_c: &mut Criterion) {
    let points = generate_3d_data();
    let to_insert = points[points.len() - 1].clone();
    let mut base_points = points.clone();
    base_points.pop();
    let mut cc = configure_criterion();
    cc.bench_function("insert_3d_kdtree", |b| {
        b.iter_with_setup(
            || {
                let mut tree = kd_tree::KdTree::new(3).unwrap();
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

fn insert_2d_rstartree(points: Vec<Point2D<i32>>) {
    info!("Starting insertion for 2D RStarTree");
    let mut tree = r_star_tree::RStarTree::new(BENCH_NODE_CAPACITY).unwrap();
    for point in points {
        tree.insert(point);
    }
    info!("Finished insertion for 2D RStarTree");
}

fn insert_3d_rstartree(points: Vec<Point3D<i32>>) {
    info!("Starting insertion for 3D RStarTree");
    let mut tree = r_star_tree::RStarTree::new(BENCH_NODE_CAPACITY).unwrap();
    for point in points {
        tree.insert(point);
    }
    info!("Finished insertion for 3D RStarTree");
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
