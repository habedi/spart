use std::cmp::Ordering;
use std::collections::BinaryHeap;

use crate::geometry::{BSPBounds, Cube, Point2D, Point3D, Rectangle};
use tracing::{debug, info};

pub trait BoundingVolume: Clone {
    fn area(&self) -> f64;
    fn union(&self, other: &Self) -> Self;
    fn enlargement(&self, other: &Self) -> f64 {
        self.union(other).area() - self.area()
    }
    fn intersects(&self, other: &Self) -> bool;
}

impl BoundingVolume for Rectangle {
    fn area(&self) -> f64 {
        self.area()
    }
    fn union(&self, other: &Self) -> Self {
        self.union(other)
    }
    fn intersects(&self, other: &Self) -> bool {
        self.intersects(other)
    }
}

impl BoundingVolume for Cube {
    fn area(&self) -> f64 {
        self.area()
    }
    fn union(&self, other: &Self) -> Self {
        self.union(other)
    }
    fn intersects(&self, other: &Self) -> bool {
        self.intersects(other)
    }
}

pub trait BSPTreeObject: std::fmt::Debug + Clone {
    type B: BoundingVolume + BSPBounds + Clone + std::fmt::Debug;
    fn mbr(&self) -> Self::B;
}

#[derive(Debug, Clone)]
enum BSPNode<T: BSPTreeObject> {
    Leaf {
        objects: Vec<T>,
        mbr: T::B,
    },
    Node {
        split_dim: usize,
        split_val: f64,
        left: Box<BSPNode<T>>,
        right: Box<BSPNode<T>>,
        mbr: T::B,
    },
}

impl<T: BSPTreeObject> BSPNode<T> {
    fn get_mbr(&self) -> T::B {
        match self {
            BSPNode::Leaf { mbr, .. } => mbr.clone(),
            BSPNode::Node { mbr, .. } => mbr.clone(),
        }
    }
}

#[derive(Debug)]
pub struct BSPTree<T: BSPTreeObject> {
    root: Option<BSPNode<T>>,
    max_objects: usize,
}

impl<T: BSPTreeObject> BSPTree<T>
where
    T: PartialEq,
{
    pub fn new(max_objects: usize) -> Self {
        info!("Creating new BSPTree with max_objects: {}", max_objects);
        BSPTree {
            root: None,
            max_objects,
        }
    }

    pub fn insert(&mut self, object: T) {
        let obj_mbr = object.mbr();
        info!("Inserting object with mbr: {:?}", obj_mbr);
        self.root = match self.root.take() {
            None => {
                info!("Tree is empty; creating new leaf.");
                Some(BSPNode::Leaf {
                    objects: vec![object],
                    mbr: obj_mbr,
                })
            }
            Some(node) => {
                let new_node = Self::insert_rec(node, object, obj_mbr, self.max_objects);
                Some(new_node)
            }
        };
    }

    fn insert_rec(node: BSPNode<T>, object: T, obj_mbr: T::B, max_objects: usize) -> BSPNode<T> {
        match node {
            BSPNode::Leaf { mut objects, mbr } => {
                let new_mbr = mbr.union(&obj_mbr);
                debug!(
                    "Inserting into leaf. Old mbr: {:?}, new object mbr: {:?}, new mbr: {:?}",
                    mbr, obj_mbr, new_mbr
                );
                objects.push(object);
                if objects.len() > max_objects {
                    info!(
                        "Leaf exceeded max_objects ({} objects); splitting leaf.",
                        objects.len()
                    );
                    Self::split_leaf(objects, new_mbr)
                } else {
                    BSPNode::Leaf {
                        objects,
                        mbr: new_mbr,
                    }
                }
            }
            BSPNode::Node {
                split_dim,
                split_val,
                left,
                right,
                mbr: _,
            } => {
                let center = obj_mbr.center(split_dim);
                debug!(
                    "At node: split_dim: {}, split_val: {}, object center: {}",
                    split_dim, split_val, center
                );
                if center < split_val {
                    debug!("Inserting object into left child.");
                    let new_left = Self::insert_rec(*left, object, obj_mbr, max_objects);
                    let new_mbr = new_left.get_mbr().union(&right.get_mbr());
                    BSPNode::Node {
                        split_dim,
                        split_val,
                        left: Box::new(new_left),
                        right,
                        mbr: new_mbr,
                    }
                } else {
                    debug!("Inserting object into right child.");
                    let new_right = Self::insert_rec(*right, object, obj_mbr, max_objects);
                    let new_mbr = left.get_mbr().union(&new_right.get_mbr());
                    BSPNode::Node {
                        split_dim,
                        split_val,
                        left,
                        right: Box::new(new_right),
                        mbr: new_mbr,
                    }
                }
            }
        }
    }

    fn split_leaf(objects: Vec<T>, mbr: T::B) -> BSPNode<T> {
        info!("Splitting leaf node.");
        let dims = <T::B as BSPBounds>::DIM;
        let mut best_dim = 0;
        let mut max_extent = 0.0;
        for dim in 0..dims {
            let extent = mbr.extent(dim);
            if extent > max_extent {
                max_extent = extent;
                best_dim = dim;
            }
        }
        info!(
            "Chosen split dimension: {} with extent: {}",
            best_dim, max_extent
        );
        let mut centers: Vec<f64> = objects
            .iter()
            .map(|obj| obj.mbr().center(best_dim))
            .collect();
        centers.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = centers[centers.len() / 2];
        info!("Computed median value: {}", median);
        let (mut left_objs, mut right_objs) = (Vec::new(), Vec::new());
        for obj in objects {
            let c = obj.mbr().center(best_dim);
            if c < median {
                left_objs.push(obj);
            } else {
                right_objs.push(obj);
            }
        }
        if left_objs.is_empty() {
            info!("Left partition empty; moving one object from right to left.");
            left_objs.push(right_objs.remove(0));
        } else if right_objs.is_empty() {
            info!("Right partition empty; moving one object from left to right.");
            right_objs.push(left_objs.pop().unwrap());
        }
        let left_mbr = left_objs
            .iter()
            .skip(1)
            .fold(left_objs[0].mbr(), |acc, obj| acc.union(&obj.mbr()));
        let right_mbr = right_objs
            .iter()
            .skip(1)
            .fold(right_objs[0].mbr(), |acc, obj| acc.union(&obj.mbr()));
        info!(
            "Leaf split complete. Left mbr: {:?}, Right mbr: {:?}",
            left_mbr, right_mbr
        );
        BSPNode::Node {
            split_dim: best_dim,
            split_val: median,
            left: Box::new(BSPNode::Leaf {
                objects: left_objs,
                mbr: left_mbr.clone(),
            }),
            right: Box::new(BSPNode::Leaf {
                objects: right_objs,
                mbr: right_mbr.clone(),
            }),
            mbr: left_mbr.clone().union(&right_mbr.clone()),
        }
    }

    pub fn range_search(&self, query: &T::B) -> Vec<&T> {
        info!("Starting range search with query: {:?}", query);
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            Self::range_search_rec(root, query, &mut result);
        }
        info!("Range search completed; found {} objects.", result.len());
        result
    }

    fn range_search_rec<'a>(node: &'a BSPNode<T>, query: &T::B, result: &mut Vec<&'a T>) {
        match node {
            BSPNode::Leaf { objects, mbr } => {
                if mbr.intersects(query) {
                    for obj in objects {
                        if obj.mbr().intersects(query) {
                            result.push(obj);
                        }
                    }
                }
            }
            BSPNode::Node {
                left, right, mbr, ..
            } => {
                if mbr.intersects(query) {
                    Self::range_search_rec(left, query, result);
                    Self::range_search_rec(right, query, result);
                }
            }
        }
    }

    pub fn knn_search(&self, query: &T::B, k: usize) -> Vec<&T> {
        info!("Starting kNN search with query: {:?}, k: {}", query, k);
        let mut heap = BinaryHeap::new();
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            heap.push(BSPCandidate::Node(root, root.get_mbr().union(query).area()));
        }
        while let Some(candidate) = heap.pop() {
            match candidate {
                BSPCandidate::Leaf(obj, _) => {
                    result.push(obj);
                    if result.len() >= k {
                        break;
                    }
                }
                BSPCandidate::Node(node, _) => match node {
                    BSPNode::Leaf { objects, .. } => {
                        for obj in objects {
                            let d = obj.mbr().union(query).area();
                            heap.push(BSPCandidate::Leaf(obj, d));
                        }
                    }
                    BSPNode::Node { left, right, .. } => {
                        heap.push(BSPCandidate::Node(left, left.get_mbr().union(query).area()));
                        heap.push(BSPCandidate::Node(
                            right,
                            right.get_mbr().union(query).area(),
                        ));
                    }
                },
            }
        }
        info!("kNN search completed; found {} objects.", result.len());
        result
    }

    pub fn delete(&mut self, object: &T) -> bool {
        info!("Attempting to delete object: {:?}", object);
        if let Some(root) = self.root.take() {
            let (new_root, found) = Self::delete_rec(root, object);
            self.root = Some(new_root);
            if found {
                info!("Object deleted successfully.");
            } else {
                info!("Object not found for deletion.");
            }
            found
        } else {
            info!("Delete called on an empty tree.");
            false
        }
    }

    fn delete_rec(node: BSPNode<T>, object: &T) -> (BSPNode<T>, bool) {
        match node {
            BSPNode::Leaf { mut objects, mbr } => {
                let initial = objects.len();
                objects.retain(|obj| obj != object);
                let found = objects.len() != initial;
                let new_mbr = if objects.is_empty() {
                    mbr
                } else {
                    objects
                        .iter()
                        .skip(1)
                        .fold(objects[0].mbr(), |acc, obj| acc.union(&obj.mbr()))
                };
                (
                    BSPNode::Leaf {
                        objects,
                        mbr: new_mbr,
                    },
                    found,
                )
            }
            BSPNode::Node {
                split_dim,
                split_val,
                left,
                right,
                mbr: _,
            } => {
                let (new_left, found_left) = Self::delete_rec(*left, object);
                let (new_right, found_right) = Self::delete_rec(*right, object);
                let found = found_left || found_right;
                let new_mbr = new_left.get_mbr().union(&new_right.get_mbr());
                (
                    BSPNode::Node {
                        split_dim,
                        split_val,
                        left: Box::new(new_left),
                        right: Box::new(new_right),
                        mbr: new_mbr,
                    },
                    found,
                )
            }
        }
    }
}

#[derive(Debug)]
enum BSPCandidate<'a, T: BSPTreeObject> {
    Node(&'a BSPNode<T>, f64),
    Leaf(&'a T, f64),
}

impl<'a, T: BSPTreeObject> PartialEq for BSPCandidate<'a, T> {
    fn eq(&self, other: &Self) -> bool {
        self.distance().eq(&other.distance())
    }
}
impl<'a, T: BSPTreeObject> Eq for BSPCandidate<'a, T> {}
impl<'a, T: BSPTreeObject> PartialOrd for BSPCandidate<'a, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.distance().partial_cmp(&self.distance())
    }
}
impl<'a, T: BSPTreeObject> Ord for BSPCandidate<'a, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<'a, T: BSPTreeObject> BSPCandidate<'a, T> {
    fn distance(&self) -> f64 {
        match self {
            BSPCandidate::Node(_, d) => *d,
            BSPCandidate::Leaf(_, d) => *d,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point2DBSP<T> {
    pub point: Point2D<T>,
}

impl<T: std::fmt::Debug + Clone> BSPTreeObject for Point2DBSP<T> {
    type B = Rectangle;
    fn mbr(&self) -> Self::B {
        Rectangle {
            x: self.point.x,
            y: self.point.y,
            width: 0.0,
            height: 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Point3DBSP<T> {
    pub point: Point3D<T>,
}

impl<T: std::fmt::Debug + Clone> BSPTreeObject for Point3DBSP<T> {
    type B = Cube;
    fn mbr(&self) -> Self::B {
        Cube {
            x: self.point.x,
            y: self.point.y,
            z: self.point.z,
            width: 0.0,
            height: 0.0,
            depth: 0.0,
        }
    }
}
