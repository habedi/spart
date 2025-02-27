//! R‑tree implementation.
//!
//! This module implements an R‑Tree, a spatial index structure that efficiently organizes
//! multi-dimensional geometric objects. It supports insertion, deletion, range search, and k‑nearest
//! neighbor (k‑NN) search. Objects stored in the R‑Tree must implement the `RTreeObject` trait,
//! which requires an implementation of a method to obtain a minimum bounding rectangle (for 2D)
//! or cube (for 3D).
//!
//! # Examples
//!
//! ```
//! use spart::geometry::{Point2D, Rectangle, Point3D, Cube};
//! use spart::r_tree::{RTree, RTreeObject};
//!
//! // Create an R‑Tree for 2D points.
//! let mut tree2d: RTree<Point2D<()>> = RTree::new(4);
//! let pt2d: Point2D<()> = Point2D::new(10.0, 20.0, None);
//! tree2d.insert(pt2d);
//! let query_rect = Rectangle { x: 5.0, y: 15.0, width: 10.0, height: 10.0 };
//! let results = tree2d.range_search_bbox(&query_rect);
//! assert!(!results.is_empty());
//!
//! // Create an R‑Tree for 3D points.
//! let mut tree3d: RTree<Point3D<()>> = RTree::new(4);
//! let pt3d: Point3D<()> = Point3D::new(10.0, 20.0, 30.0, None);
//! tree3d.insert(pt3d);
//! let query_cube = Cube { x: 5.0, y: 15.0, z: 25.0, width: 10.0, height: 10.0, depth: 10.0 };
//! let results3d = tree3d.range_search_bbox(&query_cube);
//! assert!(!results3d.is_empty());
//! ```

use crate::exceptions::SpartError;
use crate::geometry::{
    BoundingVolume, BoundingVolumeFromPoint, Cube, HasMinDistance, Point2D, Point3D, Rectangle,
};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use tracing::{debug, info};

/// Trait for objects that can be stored in an R‑Tree.
///
/// Each object must provide its minimum bounding rectangle (or cube) via the `mbr()` method.
pub trait RTreeObject: std::fmt::Debug {
    /// The type of the bounding volume (e.g. `Rectangle` for 2D objects or `Cube` for 3D objects).
    type B: BoundingVolume + std::fmt::Debug;
    /// Returns the minimum bounding volume of the object.
    fn mbr(&self) -> Self::B;
}

/// An entry in the R‑Tree, which can be either a leaf containing an object or a node pointing
/// to a child.
#[derive(Debug, Clone)]
pub enum RTreeEntry<T: RTreeObject> {
    Leaf { mbr: T::B, object: T },
    Node { mbr: T::B, child: Box<RTreeNode<T>> },
}

impl<T: RTreeObject> RTreeEntry<T> {
    /// Returns a reference to the minimum bounding volume for this entry.
    pub fn mbr(&self) -> &T::B {
        match self {
            RTreeEntry::Leaf { mbr, .. } => mbr,
            RTreeEntry::Node { mbr, .. } => mbr,
        }
    }
}

/// A node in the R‑Tree.
#[derive(Debug, Clone)]
pub struct RTreeNode<T: RTreeObject> {
    /// The entries stored in this node.
    pub entries: Vec<RTreeEntry<T>>,
    /// Indicates whether this node is a leaf.
    pub is_leaf: bool,
}

/// R‑Tree data structure for spatial indexing.
///
/// The tree is initialized with a maximum number of entries per node. If a node exceeds this
/// number, it will split. The tree supports insertion, deletion, and range searches.
#[derive(Debug)]
pub struct RTree<T: RTreeObject> {
    root: RTreeNode<T>,
    max_entries: usize,
}

impl<T: RTreeObject> RTree<T> {
    /// Creates a new R‑Tree with the specified maximum number of entries per node.
    ///
    /// # Arguments
    ///
    /// * `max_entries` - The maximum number of entries allowed in a node.
    ///
    /// # Panics
    ///
    /// Panics with `SpartError::InvalidCapacity` if `max_entries` is zero.
    pub fn new(max_entries: usize) -> Self {
        if max_entries == 0 {
            panic!("{}", SpartError::InvalidCapacity { capacity: 0 });
        }
        info!("Creating new RTree with max_entries: {}", max_entries);
        RTree {
            root: RTreeNode {
                entries: Vec::new(),
                is_leaf: true,
            },
            max_entries,
        }
    }

    /// Inserts an object into the R‑Tree.
    ///
    /// # Arguments
    ///
    /// * `object` - The object to insert.
    pub fn insert(&mut self, object: T) {
        info!("Inserting object into RTree: {:?}", object);
        let entry = RTreeEntry::Leaf {
            mbr: object.mbr(),
            object,
        };
        insert_entry_node(&mut self.root, entry);
        if self.root.entries.len() > self.max_entries {
            info!("Root has exceeded max_entries; splitting root");
            self.split_root();
        }
    }

    /// Splits the root node into two child nodes when it exceeds the maximum number of entries.
    fn split_root(&mut self) {
        info!("Splitting root node");
        let old_entries = std::mem::take(&mut self.root.entries);
        let (group1, group2) = split_entries(old_entries, self.max_entries);
        let child1 = RTreeNode {
            entries: group1,
            is_leaf: self.root.is_leaf,
        };
        let child2 = RTreeNode {
            entries: group2,
            is_leaf: self.root.is_leaf,
        };
        let mbr1 = compute_group_mbr(&child1.entries);
        let mbr2 = compute_group_mbr(&child2.entries);
        self.root.is_leaf = false;
        self.root.entries.push(RTreeEntry::Node {
            mbr: mbr1,
            child: Box::new(child1),
        });
        self.root.entries.push(RTreeEntry::Node {
            mbr: mbr2,
            child: Box::new(child2),
        });
    }

    /// Performs a range search with a given query bounding volume.
    ///
    /// # Arguments
    ///
    /// * `query` - The bounding volume to search against.
    ///
    /// # Returns
    ///
    /// A vector of references to the objects whose minimum bounding volumes intersect the query.
    pub fn range_search_bbox(&self, query: &T::B) -> Vec<&T> {
        info!("Performing range search with query: {:?}", query);
        let mut result = Vec::new();
        search_node(&self.root, query, &mut result);
        result
    }
}

fn insert_entry_node<T: RTreeObject>(node: &mut RTreeNode<T>, entry: RTreeEntry<T>) {
    if node.is_leaf {
        debug!("Inserting entry into leaf node");
        node.entries.push(entry);
    } else {
        let mut best_index: Option<usize> = None;
        let mut best_enlargement = f64::INFINITY;
        for (i, child_entry) in node.entries.iter().enumerate() {
            if let RTreeEntry::Node { mbr, .. } = child_entry {
                let enlargement = mbr.enlargement(entry.mbr());
                if enlargement < best_enlargement {
                    best_enlargement = enlargement;
                    best_index = Some(i);
                } else if (enlargement - best_enlargement).abs() < f64::EPSILON {
                    if let Some(current_best) = best_index {
                        if mbr.area() < node.entries[current_best].mbr().area() {
                            best_index = Some(i);
                        }
                    }
                }
            }
        }
        if let Some(best_index) = best_index {
            if let RTreeEntry::Node { mbr, child } = &mut node.entries[best_index] {
                *mbr = mbr.union(entry.mbr());
                insert_entry_node(child, entry);
                *mbr = compute_group_mbr(&child.entries);
            }
        } else {
            node.entries.push(entry);
        }
    }
}

fn split_entries<T: RTreeObject>(
    entries: Vec<RTreeEntry<T>>,
    _max_entries: usize,
) -> (Vec<RTreeEntry<T>>, Vec<RTreeEntry<T>>) {
    let mut entries = entries;
    if entries.len() < 2 {
        return (entries, Vec::new());
    }
    let seed1 = entries.remove(0);
    let seed2 = entries.remove(0);
    let mut group1 = vec![seed1];
    let mut group2 = vec![seed2];
    for entry in entries {
        let mbr1 = compute_group_mbr(&group1);
        let mbr2 = compute_group_mbr(&group2);
        let enlargement1 = mbr1.enlargement(entry.mbr());
        let enlargement2 = mbr2.enlargement(entry.mbr());
        if enlargement1 < enlargement2 {
            group1.push(entry);
        } else {
            group2.push(entry);
        }
    }
    (group1, group2)
}

fn compute_group_mbr<T: RTreeObject>(entries: &[RTreeEntry<T>]) -> T::B {
    let mut iter = entries.iter();
    let first = iter
        .next()
        .expect("At least one entry must be present")
        .mbr()
        .clone();
    iter.fold(first, |acc, entry| acc.union(entry.mbr()))
}

fn search_node<'a, T: RTreeObject>(node: &'a RTreeNode<T>, query: &T::B, result: &mut Vec<&'a T>) {
    if node.is_leaf {
        for entry in &node.entries {
            if let RTreeEntry::Leaf { mbr, object } = entry {
                if mbr.intersects(query) {
                    result.push(object);
                }
            }
        }
    } else {
        for entry in &node.entries {
            if let RTreeEntry::Node { mbr, child } = entry {
                if mbr.intersects(query) {
                    search_node(child, query, result);
                }
            }
        }
    }
}

impl<T: RTreeObject> RTree<T>
where
    T: PartialEq,
{
    /// Deletes an object from the R‑Tree.
    ///
    /// # Arguments
    ///
    /// * `object` - The object to delete.
    ///
    /// # Returns
    ///
    /// `true` if at least one matching object was found and removed.
    pub fn delete(&mut self, object: &T) -> bool {
        info!("Attempting to delete object: {:?}", object);
        let count = delete_entry(&mut self.root, object);
        if !self.root.is_leaf && self.root.entries.len() == 1 {
            if let RTreeEntry::Node { child, .. } = self.root.entries.pop().unwrap() {
                self.root = *child;
            }
        }
        count > 0
    }
}

fn delete_entry<T: RTreeObject + PartialEq>(node: &mut RTreeNode<T>, object: &T) -> usize {
    if node.is_leaf {
        let initial_len = node.entries.len();
        node.entries.retain(|entry| {
            if let RTreeEntry::Leaf {
                object: ref obj, ..
            } = entry
            {
                obj != object
            } else {
                true
            }
        });
        initial_len - node.entries.len()
    } else {
        let mut count = 0;
        for entry in &mut node.entries {
            if let RTreeEntry::Node { child, .. } = entry {
                count += delete_entry(child, object);
                if !child.entries.is_empty() {
                    let new_mbr = compute_group_mbr(&child.entries);
                    if let RTreeEntry::Node { ref mut mbr, .. } = entry {
                        *mbr = new_mbr;
                    }
                }
            }
        }
        node.entries.retain(|entry| match entry {
            RTreeEntry::Node { child, .. } => !child.entries.is_empty(),
            _ => true,
        });
        count
    }
}

impl<T: std::fmt::Debug> RTreeObject for Point2D<T> {
    type B = Rectangle;
    fn mbr(&self) -> Self::B {
        Rectangle {
            x: self.x,
            y: self.y,
            width: 0.0,
            height: 0.0,
        }
    }
}

impl<T: std::fmt::Debug> RTreeObject for Point3D<T> {
    type B = Cube;
    fn mbr(&self) -> Self::B {
        Cube {
            x: self.x,
            y: self.y,
            z: self.z,
            width: 0.0,
            height: 0.0,
            depth: 0.0,
        }
    }
}

impl Rectangle {
    /// Computes the minimum distance from this rectangle to a given 2D point.
    pub fn min_distance<T>(&self, point: &Point2D<T>) -> f64 {
        let dx = if point.x < self.x {
            self.x - point.x
        } else if point.x > self.x + self.width {
            point.x - (self.x + self.width)
        } else {
            0.0
        };
        let dy = if point.y < self.y {
            self.y - point.y
        } else if point.y > self.y + self.height {
            point.y - (self.y + self.height)
        } else {
            0.0
        };
        (dx * dx + dy * dy).sqrt()
    }
}

impl Cube {
    /// Computes the minimum distance from this cube to a given 3D point.
    pub fn min_distance<T>(&self, point: &Point3D<T>) -> f64 {
        let dx = if point.x < self.x {
            self.x - point.x
        } else if point.x > self.x + self.width {
            point.x - (self.x + self.width)
        } else {
            0.0
        };
        let dy = if point.y < self.y {
            self.y - point.y
        } else if point.y > self.y + self.height {
            point.y - (self.y + self.height)
        } else {
            0.0
        };
        let dz = if point.z < self.z {
            self.z - point.z
        } else if point.z > self.z + self.depth {
            point.z - (self.z + self.depth)
        } else {
            0.0
        };
        (dx * dx + dy * dy + dz * dz).sqrt()
    }
}

#[derive(Debug)]
struct Candidate2D<'a, T: std::fmt::Debug> {
    dist: f64,
    entry: CandidateEntry2D<'a, T>,
}

#[derive(Debug)]
enum CandidateEntry2D<'a, T: std::fmt::Debug> {
    Node(&'a RTreeNode<Point2D<T>>),
    Leaf(&'a Point2D<T>),
}

impl<T: std::fmt::Debug> PartialEq for Candidate2D<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.dist.eq(&other.dist)
    }
}
impl<T: std::fmt::Debug> Eq for Candidate2D<'_, T> {}
impl<T: std::fmt::Debug> PartialOrd for Candidate2D<'_, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.dist.partial_cmp(&self.dist)
    }
}
impl<T: std::fmt::Debug> Ord for Candidate2D<'_, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T: std::fmt::Debug> RTree<Point2D<T>> {
    /// Performs a k‑nearest neighbor search on an R‑Tree of 2D points.
    ///
    /// # Arguments
    ///
    /// * `query` - The 2D point to search near.
    /// * `k` - The number of nearest neighbors to return.
    ///
    /// # Returns
    ///
    /// A vector of references to the k nearest 2D points.
    pub fn knn_search(&self, query: &Point2D<T>, k: usize) -> Vec<&Point2D<T>> {
        let mut heap = BinaryHeap::new();
        heap.push(Candidate2D {
            dist: compute_group_mbr(&self.root.entries).min_distance(query),
            entry: CandidateEntry2D::Node(&self.root),
        });
        let mut result = Vec::new();
        while let Some(Candidate2D { entry, .. }) = heap.pop() {
            match entry {
                CandidateEntry2D::Leaf(obj) => {
                    result.push(obj);
                    if result.len() == k {
                        break;
                    }
                }
                CandidateEntry2D::Node(node) => {
                    for entry in &node.entries {
                        match entry {
                            RTreeEntry::Leaf { mbr, object } => {
                                let d = mbr.min_distance(query);
                                heap.push(Candidate2D {
                                    dist: d,
                                    entry: CandidateEntry2D::Leaf(object),
                                });
                            }
                            RTreeEntry::Node { mbr, child } => {
                                let d = mbr.min_distance(query);
                                heap.push(Candidate2D {
                                    dist: d,
                                    entry: CandidateEntry2D::Node(child),
                                });
                            }
                        }
                    }
                }
            }
        }
        result
    }
}

#[derive(Debug)]
struct Candidate3D<'a, T: std::fmt::Debug> {
    dist: f64,
    entry: CandidateEntry3D<'a, T>,
}

#[derive(Debug)]
enum CandidateEntry3D<'a, T: std::fmt::Debug> {
    Node(&'a RTreeNode<Point3D<T>>),
    Leaf(&'a Point3D<T>),
}

impl<T: std::fmt::Debug> PartialEq for Candidate3D<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.dist.eq(&other.dist)
    }
}
impl<T: std::fmt::Debug> Eq for Candidate3D<'_, T> {}
impl<T: std::fmt::Debug> PartialOrd for Candidate3D<'_, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.dist.partial_cmp(&self.dist)
    }
}
impl<T: std::fmt::Debug> Ord for Candidate3D<'_, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T: std::fmt::Debug> RTree<Point3D<T>> {
    /// Performs a k‑nearest neighbor search on an R‑Tree of 3D points.
    ///
    /// # Arguments
    ///
    /// * `query` - The 3D point to search near.
    /// * `k` - The number of nearest neighbors to return.
    ///
    /// # Returns
    ///
    /// A vector of references to the k nearest 3D points.
    pub fn knn_search(&self, query: &Point3D<T>, k: usize) -> Vec<&Point3D<T>> {
        let mut heap = BinaryHeap::new();
        let root_mbr = compute_group_mbr(&self.root.entries);
        heap.push(Candidate3D {
            dist: root_mbr.min_distance(query),
            entry: CandidateEntry3D::Node(&self.root),
        });
        let mut result = Vec::new();
        while let Some(Candidate3D { entry, .. }) = heap.pop() {
            match entry {
                CandidateEntry3D::Leaf(obj) => {
                    result.push(obj);
                    if result.len() == k {
                        break;
                    }
                }
                CandidateEntry3D::Node(node) => {
                    for entry in &node.entries {
                        match entry {
                            RTreeEntry::Leaf { mbr, object } => {
                                let d = mbr.min_distance(query);
                                heap.push(Candidate3D {
                                    dist: d,
                                    entry: CandidateEntry3D::Leaf(object),
                                });
                            }
                            RTreeEntry::Node { mbr, child } => {
                                let d = mbr.min_distance(query);
                                heap.push(Candidate3D {
                                    dist: d,
                                    entry: CandidateEntry3D::Node(child),
                                });
                            }
                        }
                    }
                }
            }
        }
        result
    }
}

impl<T> RTree<T>
where
    T: RTreeObject + PartialEq + std::fmt::Debug,
    T::B: BoundingVolumeFromPoint<T> + HasMinDistance<T> + Clone,
{
    /// Performs a range search on the R‑Tree using a query object and radius.
    ///
    /// The query object is wrapped into a bounding volume using `from_point_radius`.
    ///
    /// # Arguments
    ///
    /// * `query` - The query object.
    /// * `radius` - The search radius.
    ///
    /// # Returns
    ///
    /// A vector of references to the objects within the given radius.
    pub fn range_search(&self, query: &T, radius: f64) -> Vec<&T> {
        let query_volume = T::B::from_point_radius(query, radius);
        let candidates = self.range_search_bbox(&query_volume);
        candidates
            .into_iter()
            .filter(|object| object.mbr().min_distance(query) <= radius)
            .collect()
    }
}
