use criterion::{criterion_group, criterion_main, Criterion};
use spart::geometry::{Cube, Point2D, Point3D, Rectangle};
use spart::kd_tree::KdTree;
use spart::octree::Octree;
use spart::quadtree::Quadtree;
use spart::r_star_tree::RStarTree;
use spart::r_tree::RTree;

fn bench_quadtree_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("Quadtree Serialization");
    let boundary = Rectangle {
        x: 0.0,
        y: 0.0,
        width: 10000.0,
        height: 10000.0,
    };
    let mut qt = Quadtree::new(&boundary, 4);
    for i in 0..10000 {
        qt.insert(Point2D::new(i as f64, i as f64, Some(i)));
    }

    group.bench_function("serialize", |b| {
        b.iter(|| {
            let _encoded: Vec<u8> = bincode::serialize(&qt).unwrap();
        })
    });

    let encoded: Vec<u8> = bincode::serialize(&qt).unwrap();
    group.bench_function("deserialize", |b| {
        b.iter(|| {
            let _decoded: Quadtree<i32> = bincode::deserialize(&encoded[..]).unwrap();
        })
    });
}

fn bench_octree_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("Octree Serialization");
    let boundary = Cube {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        width: 10000.0,
        height: 10000.0,
        depth: 10000.0,
    };
    let mut octree = Octree::new(&boundary, 4);
    for i in 0..10000 {
        octree.insert(Point3D::new(i as f64, i as f64, i as f64, Some(i)));
    }

    group.bench_function("serialize", |b| {
        b.iter(|| {
            let _encoded: Vec<u8> = bincode::serialize(&octree).unwrap();
        })
    });

    let encoded: Vec<u8> = bincode::serialize(&octree).unwrap();
    group.bench_function("deserialize", |b| {
        b.iter(|| {
            let _decoded: Octree<i32> = bincode::deserialize(&encoded[..]).unwrap();
        })
    });
}

fn bench_kdtree_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("KdTree Serialization");
    let mut tree: KdTree<Point2D<i32>> = KdTree::new(2);
    for i in 0..10000 {
        tree.insert(Point2D::new(i as f64, i as f64, Some(i)));
    }

    group.bench_function("serialize", |b| {
        b.iter(|| {
            let _encoded: Vec<u8> = bincode::serialize(&tree).unwrap();
        })
    });

    let encoded: Vec<u8> = bincode::serialize(&tree).unwrap();
    group.bench_function("deserialize", |b| {
        b.iter(|| {
            let _decoded: KdTree<Point2D<i32>> = bincode::deserialize(&encoded[..]).unwrap();
        })
    });
}

fn bench_rtree_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("RTree Serialization");
    let mut tree: RTree<Point2D<i32>> = RTree::new(16);
    for i in 0..10000 {
        tree.insert(Point2D::new(i as f64, i as f64, Some(i)));
    }

    group.bench_function("serialize", |b| {
        b.iter(|| {
            let _encoded: Vec<u8> = bincode::serialize(&tree).unwrap();
        })
    });

    let encoded: Vec<u8> = bincode::serialize(&tree).unwrap();
    group.bench_function("deserialize", |b| {
        b.iter(|| {
            let _decoded: RTree<Point2D<i32>> = bincode::deserialize(&encoded[..]).unwrap();
        })
    });
}

fn bench_rstartree_serialization(c: &mut Criterion) {
    let mut group = c.benchmark_group("RStarTree Serialization");
    let mut tree: RStarTree<Point2D<i32>> = RStarTree::new(16);
    for i in 0..10000 {
        tree.insert(Point2D::new(i as f64, i as f64, Some(i)));
    }

    group.bench_function("serialize", |b| {
        b.iter(|| {
            let _encoded: Vec<u8> = bincode::serialize(&tree).unwrap();
        })
    });

    let encoded: Vec<u8> = bincode::serialize(&tree).unwrap();
    group.bench_function("deserialize", |b| {
        b.iter(|| {
            let _decoded: RStarTree<Point2D<i32>> = bincode::deserialize(&encoded[..]).unwrap();
        })
    });
}

criterion_group!(
    benches,
    bench_quadtree_serialization,
    bench_octree_serialization,
    bench_kdtree_serialization,
    bench_rtree_serialization,
    bench_rstartree_serialization
);
criterion_main!(benches);
