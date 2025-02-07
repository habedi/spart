use criterion::{black_box, criterion_group, criterion_main, Criterion};
mod utils;
use spart::bsp_tree::{Point2DBSP, Point3DBSP};
use spart::geometry::{Cube, Point2D, Point3D, Rectangle};
use spart::{bsp_tree, kd_tree, octree, quadtree, r_tree};
use tracing::info;
use utils::*;

pub fn configure_criterion() -> Criterion {
    Criterion::default().measurement_time(BENCH_TIMEOUT)
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
    cc.bench_function("range_kdtree_2d", |b| {
        b.iter(|| {
            info!("Running range search on 2D KdTree");
            let res = tree.range_search(&query, BENCH_RANGE_RADIUS);
            info!("Completed range search on 2D KdTree");
            black_box(res)
        })
    });
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
    cc.bench_function("range_rtree_2d", |b| {
        b.iter(|| {
            info!("Running range search on 2D RTree");
            let res = tree.range_search_bbox(&query_rect);
            info!("Completed range search on 2D RTree");
            black_box(res)
        })
    });
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
    cc.bench_function("range_rtree_2d", |b| {
        b.iter(|| {
            info!("Running range search on 2D RTree");
            let res = tree.range_search(&query_point, BENCH_RANGE_RADIUS);
            info!("Completed range search on 2D RTree");
            black_box(res)
        })
    });
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
    cc.bench_function("range_bsptree_2d", |b| {
        b.iter(|| {
            info!("Running range search on 2D BSPTree");
            let res = tree.range_search_bbox(&query_rect);
            info!("Completed range search on 2D BSPTree");
            black_box(res)
        })
    });
}

fn benchmark_range_bsptree_2d(_c: &mut Criterion) {
    info!("Setting up benchmark_range_bsptree_2d");
    let points = generate_2d_data_wrapped();
    let mut tree = bsp_tree::BSPTree::<Point2DBSP<i32>>::new(BENCH_NODE_CAPACITY);
    for point in points.iter() {
        tree.insert(point.clone());
    }
    let quuery_point = Point2DBSP {
        point: Point2D {
            x: 35.0,
            y: 45.0,
            data: None,
        },
    };
    let mut cc = configure_criterion();
    cc.bench_function("range_bsptree_2d", |b| {
        b.iter(|| {
            info!("Running range search on 2D BSPTree");
            let res = tree.range_search(&quuery_point, BENCH_RANGE_RADIUS);
            info!("Completed range search on 2D BSPTree");
            black_box(res)
        })
    });
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
    cc.bench_function("range_quadtree_2d", |b| {
        b.iter(|| {
            info!("Running range search on 2D Quadtree");
            let res = tree.range_search(&query, BENCH_RANGE_RADIUS);
            info!("Completed range search on 2D Quadtree");
            black_box(res)
        })
    });
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
    cc.bench_function("range_kdtree_3d", |b| {
        b.iter(|| {
            info!("Running range search on 3D KdTree");
            let res = tree.range_search(&query, BENCH_RANGE_RADIUS);
            info!("Completed range search on 3D KdTree");
            black_box(res)
        })
    });
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
    cc.bench_function("range_rtree_3d", |b| {
        b.iter(|| {
            info!("Running range search on 3D RTree");
            let res = tree.range_search_bbox(&query_cube);
            info!("Completed range search on 3D RTree");
            black_box(res)
        })
    });
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
    cc.bench_function("range_rtree_3d", |b| {
        b.iter(|| {
            info!("Running range search on 3D RTree");
            let res = tree.range_search(&query_point, BENCH_RANGE_RADIUS);
            info!("Completed range search on 3D RTree");
            black_box(res)
        })
    });
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
    cc.bench_function("range_bsptree_3d", |b| {
        b.iter(|| {
            info!("Running range search on 3D BSPTree");
            let res = tree.range_search_bbox(&query_cube);
            info!("Completed range search on 3D BSPTree");
            black_box(res)
        })
    });
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
    cc.bench_function("range_bsptree_3d", |b| {
        b.iter(|| {
            info!("Running range search on 3D BSPTree");
            let res = tree.range_search(&query_point, BENCH_RANGE_RADIUS);
            info!("Completed range search on 3D BSPTree");
            black_box(res)
        })
    });
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
    cc.bench_function("range_octree_3d", |b| {
        b.iter(|| {
            info!("Running range search on 3D Octree");
            let res = tree.range_search(&query, BENCH_RANGE_RADIUS);
            info!("Completed range search on 3D Octree");
            black_box(res)
        })
    });
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
criterion_main!(benches);
