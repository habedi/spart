#![allow(dead_code)]
#[path = "shared.rs"]
mod shared;
use shared::*;

use criterion::{criterion_group, Criterion};
use spart::geometry::{Point2D, Point3D, Rectangle};
use spart::{kd_tree, octree, quadtree, r_star_tree, r_tree};
use std::hint::black_box;
use tracing::info;

/// Helper function that benchmarks a deletion function on a given dataset.
///
/// # Arguments
///
/// * `bench_name` - The name of the benchmark.
/// * `points` - A vector of points to be inserted into the tree.
/// * `delete_fn` - A closure that takes the points vector and performs tree insertion and deletion.
fn bench_delete_tree<P: Clone>(bench_name: &str, points: Vec<P>, delete_fn: impl Fn(Vec<P>)) {
    let mut cc = configure_criterion();
    cc.bench_function(bench_name, |b| {
        b.iter(|| {
            info!("Running deletion benchmark: {}", bench_name);
            delete_fn(black_box(points.clone()));
        })
    });
}

/// Deletes all points from a 2D Quadtree.
fn delete_2d_quadtree(points: Vec<Point2D<i32>>) {
    info!("Starting deletion for 2D Quadtree");
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
    for point in points.iter() {
        tree.delete(point);
    }
    info!("Finished deletion for 2D Quadtree");
}

/// Deletes all points from a 3D Octree.
fn delete_3d_octree(points: Vec<Point3D<i32>>) {
    info!("Starting deletion for 3D Octree");
    let boundary = BENCH_BOUNDARY;
    let mut tree = octree::Octree::new(&boundary, BENCH_NODE_CAPACITY).unwrap();
    for point in points.iter() {
        tree.insert(point.clone());
    }
    for point in points.iter() {
        tree.delete(point);
    }
    info!("Finished deletion for 3D Octree");
}

/// Deletes all points from a 2D KdTree.
fn delete_2d_kdtree(points: Vec<Point2D<i32>>) {
    info!("Starting deletion for 2D KdTree");
    let mut tree = kd_tree::KdTree::new(2).unwrap();
    for point in points.iter() {
        tree.insert(point.clone());
    }
    for point in points.iter() {
        tree.delete(point);
    }
    info!("Finished deletion for 2D KdTree");
}

/// Deletes all points from a 3D KdTree.
fn delete_3d_kdtree(points: Vec<Point3D<i32>>) {
    info!("Starting deletion for 3D KdTree");
    let mut tree = kd_tree::KdTree::new(3).unwrap();
    for point in points.iter() {
        tree.insert(point.clone());
    }
    for point in points.iter() {
        tree.delete(point);
    }
    info!("Finished deletion for 3D KdTree");
}

/// Deletes all points from a 2D RTree.
fn delete_2d_rtree(points: Vec<Point2D<i32>>) {
    info!("Starting deletion for 2D RTree");
    let mut tree = r_tree::RTree::new(BENCH_NODE_CAPACITY).unwrap();
    for point in points.iter() {
        tree.insert(point.clone());
    }
    for point in points.iter() {
        tree.delete(point);
    }
    info!("Finished deletion for 2D RTree");
}

/// Deletes all points from a 3D RTree.
fn delete_3d_rtree(points: Vec<Point3D<i32>>) {
    info!("Starting deletion for 3D RTree");
    let mut tree = r_tree::RTree::new(BENCH_NODE_CAPACITY).unwrap();
    for point in points.iter() {
        tree.insert(point.clone());
    }
    for point in points.iter() {
        tree.delete(point);
    }
    info!("Finished deletion for 3D RTree");
}

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
                let mut tree = kd_tree::KdTree::new(2).unwrap();
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

fn benchmark_delete_kdtree_3d(_c: &mut Criterion) {
    let points = generate_3d_data();
    let point_to_delete = points[points.len() / 2].clone();
    let mut cc = configure_criterion();
    cc.bench_function("delete_3d_kdtree", |b| {
        b.iter_with_setup(
            || {
                let mut tree = kd_tree::KdTree::new(3).unwrap();
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

fn benchmark_delete_rtree_2d(_c: &mut Criterion) {
    let points = generate_2d_data();
    let point_to_delete = points[points.len() / 2].clone();
    let mut cc = configure_criterion();
    cc.bench_function("delete_2d_rtree", |b| {
        b.iter_with_setup(
            || {
                let mut tree = r_tree::RTree::new(BENCH_NODE_CAPACITY).unwrap();
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
                let mut tree = r_tree::RTree::new(BENCH_NODE_CAPACITY).unwrap();
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

fn delete_2d_rstartree(points: Vec<Point2D<i32>>) {
    info!("Starting deletion for 2D RStarTree");
    let mut tree = r_star_tree::RStarTree::new(BENCH_NODE_CAPACITY).unwrap();
    for point in points.iter() {
        tree.insert(point.clone());
    }
    for point in points.iter() {
        tree.delete(point);
    }
    info!("Finished deletion for 2D RStarTree");
}

fn delete_3d_rstartree(points: Vec<Point3D<i32>>) {
    info!("Starting deletion for 3D RStarTree");
    let mut tree = r_star_tree::RStarTree::new(BENCH_NODE_CAPACITY).unwrap();
    for point in points.iter() {
        tree.insert(point.clone());
    }
    for point in points.iter() {
        tree.delete(point);
    }
    info!("Finished deletion for 3D RStarTree");
}

fn benchmark_delete_rstartree_2d(_c: &mut Criterion) {
    let points = generate_2d_data();
    let point_to_delete = points[points.len() / 2].clone();
    let mut cc = configure_criterion();
    cc.bench_function("delete_2d_rstartree", |b| {
        b.iter_with_setup(
            || {
                let mut tree = r_star_tree::RStarTree::new(BENCH_NODE_CAPACITY).unwrap();
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
                let mut tree = r_star_tree::RStarTree::new(BENCH_NODE_CAPACITY).unwrap();
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
