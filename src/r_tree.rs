//! ## R‑tree Implementation
//!
//! This module implements an R‑tree for indexing 2D and 3D points.
//! The implementation supports insertion, deletion, range search, and k‑nearest
//! neighbor (kNN) search. Points stored in the R‑tree must implement the `RTreeObject` trait,
//! which requires an implementation of a method to get a minimum bounding rectangle (for 2D)
//! or cube (for 3D) around the point.
//!
//! # Examples
//!
//! ```
//! use spart::geometry::{Point2D, Rectangle, Point3D, Cube};
//! use spart::r_tree::{RTree, RTreeObject};
//!
//! // Create an R‑tree for 2D points.
//! let mut tree2d: RTree<Point2D<()>> = RTree::new(4);
//! let pt2d: Point2D<()> = Point2D::new(10.0, 20.0, None);
//! tree2d.insert(pt2d);
//! let query_rect = Rectangle { x: 5.0, y: 15.0, width: 10.0, height: 10.0 };
//! let results = tree2d.range_search_bbox(&query_rect);
//! assert!(!results.is_empty());
//!
//! // Create an R‑tree for 3D points.
//! let mut tree3d: RTree<Point3D<()>> = RTree::new(4);
//! let pt3d: Point3D<()> = Point3D::new(10.0, 20.0, 30.0, None);
//! tree3d.insert(pt3d);
//! let query_cube = Cube { x: 5.0, y: 15.0, z: 25.0, width: 10.0, height: 10.0, depth: 10.0 };
//! let results3d = tree3d.range_search_bbox(&query_cube);
//! assert!(!results3d.is_empty());
//! ```

use crate::exceptions::SpartError;
use crate::geometry::{
    BoundingVolume, BoundingVolumeFromPoint, Cube, DistanceMetric, HasMinDistance, Point2D,
    Point3D, Rectangle,
};
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use tracing::{debug, info};

// Epsilon value for zero-sizes bounding boxes/cubes.
const EPSILON: f64 = 1e-10;

/// Trait for points stored in an R‑tree.
///
/// Each object must provide its minimum bounding rectangle (or cube) via the `mbr()` method.
#[cfg(feature = "serde")]
pub trait RTreeObject: std::fmt::Debug + Clone {
    /// The type of the bounding volume (e.g. `Rectangle` for 2D objects or `Cube` for 3D objects).
    type B: BoundingVolume
        + std::fmt::Debug
        + Clone
        + serde::Serialize
        + for<'de> serde::Deserialize<'de>;
    /// Returns the minimum bounding volume of the object.
    fn mbr(&self) -> Self::B;
}
#[cfg(not(feature = "serde"))]
pub trait RTreeObject: std::fmt::Debug + Clone {
    /// The type of the bounding volume (e.g. `Rectangle` for 2D objects or `Cube` for 3D objects).
    type B: BoundingVolume + std::fmt::Debug + Clone;
    /// Returns the minimum bounding volume of the object.
    fn mbr(&self) -> Self::B;
}

/// An entry in the R‑tree, which can be either a leaf or a node.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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

/// A node in the R‑tree.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RTreeNode<T: RTreeObject> {
    /// The entries stored in this node.
    pub entries: Vec<RTreeEntry<T>>,
    /// Indicates whether this node is a leaf.
    pub is_leaf: bool,
}

/// R‑tree data structure for indexing 2D or 3D points.
///
/// The tree is initialized with a maximum number of entries per node. If a node exceeds this
/// number, it will split. The tree supports insertion, deletion, and range searches.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RTree<T: RTreeObject> {
    root: RTreeNode<T>,
    max_entries: usize,
    min_entries: usize,
}

impl<T: RTreeObject> RTree<T> {
    /// Creates a new R‑tree with the specified maximum number of entries per node.
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
            min_entries: (max_entries as f64 * 0.4).ceil() as usize,
        }
    }

    /// Inserts an object into the R‑tree.
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

    /// Inserts a bulk of objects into the R-tree.
    ///
    /// # Arguments
    ///
    /// * `objects` - The objects to insert.
    pub fn insert_bulk(&mut self, objects: Vec<T>) {
        if objects.is_empty() {
            return;
        }

        let mut entries: Vec<RTreeEntry<T>> = objects
            .into_iter()
            .map(|obj| RTreeEntry::Leaf {
                mbr: obj.mbr(),
                object: obj,
            })
            .collect();

        while entries.len() > self.max_entries {
            let mut new_level_entries = Vec::new();
            let chunks = entries.chunks(self.max_entries);

            for chunk in chunks {
                let child_node = RTreeNode {
                    entries: chunk.to_vec(),
                    is_leaf: self.root.is_leaf,
                };
                let mbr = compute_group_mbr(&child_node.entries);
                new_level_entries.push(RTreeEntry::Node {
                    mbr,
                    child: Box::new(child_node),
                });
            }
            entries = new_level_entries;
            self.root.is_leaf = false;
        }

        self.root.entries.extend(entries);
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
    /// Deletes an object from the R‑tree.
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
        let object_mbr = object.mbr();
        let mut reinsert_list = Vec::new();
        let deleted = delete_entry(
            &mut self.root,
            object,
            &object_mbr,
            self.min_entries,
            &mut reinsert_list,
        );

        if deleted {
            for entry in reinsert_list {
                self.insert_entry(entry);
            }

            if !self.root.is_leaf && self.root.entries.len() == 1 {
                if let RTreeEntry::Node { child, .. } = self.root.entries.pop().unwrap() {
                    self.root = *child;
                }
            }
        }
        deleted
    }

    fn insert_entry(&mut self, entry: RTreeEntry<T>) {
        insert_entry_node(&mut self.root, entry);
        if self.root.entries.len() > self.max_entries {
            self.split_root();
        }
    }
}

fn delete_entry<T: RTreeObject + PartialEq>(
    node: &mut RTreeNode<T>,
    object: &T,
    object_mbr: &T::B,
    min_entries: usize,
    reinsert_list: &mut Vec<RTreeEntry<T>>,
) -> bool {
    let mut deleted = false;
    if node.is_leaf {
        let initial_len = node.entries.len();
        node.entries.retain(|entry| {
            if let RTreeEntry::Leaf { object: ref o, .. } = entry {
                o != object
            } else {
                true
            }
        });
        deleted = node.entries.len() < initial_len;
    } else {
        let mut to_delete_indices = Vec::new();
        for (i, entry) in node.entries.iter_mut().enumerate() {
            if let RTreeEntry::Node { mbr, child } = entry {
                if mbr.intersects(object_mbr)
                    && delete_entry(child, object, object_mbr, min_entries, reinsert_list)
                {
                    deleted = true;
                    if child.entries.len() < min_entries {
                        to_delete_indices.push(i);
                    } else {
                        *mbr = compute_group_mbr(&child.entries);
                    }
                }
            }
        }

        for &index in to_delete_indices.iter().rev() {
            if let RTreeEntry::Node { child, .. } = node.entries.remove(index) {
                reinsert_list.extend(child.entries);
            }
        }
    }
    deleted
}

impl<T: std::fmt::Debug + Clone> RTreeObject for Point2D<T> {
    type B = Rectangle;
    fn mbr(&self) -> Self::B {
        Rectangle {
            x: self.x,
            y: self.y,
            width: EPSILON,
            height: EPSILON,
        }
    }
}

impl<T: std::fmt::Debug + Clone> RTreeObject for Point3D<T> {
    type B = Cube;
    fn mbr(&self) -> Self::B {
        Cube {
            x: self.x,
            y: self.y,
            z: self.z,
            width: EPSILON,
            height: EPSILON,
            depth: EPSILON,
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

// Knn-search candidate.
#[derive(Debug)]
struct KnnCandidate<'a, T: RTreeObject> {
    dist: f64,
    entry: &'a RTreeEntry<T>,
}

impl<T: RTreeObject> PartialEq for KnnCandidate<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.dist.eq(&other.dist)
    }
}

impl<T: RTreeObject> Eq for KnnCandidate<'_, T> {}

impl<T: RTreeObject> Ord for KnnCandidate<'_, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .dist
            .partial_cmp(&self.dist)
            .unwrap_or(Ordering::Equal)
    }
}

impl<T: RTreeObject> PartialOrd for KnnCandidate<'_, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: std::fmt::Debug + Ord + Clone> RTree<Point2D<T>> {
    /// Performs a k‑nearest neighbor search on an R‑tree of 2D points.
    ///
    /// # Arguments
    ///
    /// * `query` - The 2D point to search near.
    /// * `k` - The number of nearest neighbors to return.
    ///
    /// # Returns
    ///
    /// A vector of references to the k nearest 2D points.
    ///
    /// # Note
    ///
    /// The pruning logic for the search is based on Euclidean distance. Custom distance metrics
    /// that are not compatible with Euclidean distance may lead to incorrect results or reduced
    /// performance.
    pub fn knn_search<M: DistanceMetric<Point2D<T>>>(
        &self,
        query: &Point2D<T>,
        k: usize,
    ) -> Vec<&Point2D<T>> {
        if k == 0 {
            return Vec::new();
        }

        let mut heap: BinaryHeap<KnnCandidate<Point2D<T>>> = BinaryHeap::new();
        for entry in &self.root.entries {
            let dist_sq = entry.mbr().min_distance(query).powi(2);
            heap.push(KnnCandidate {
                dist: dist_sq,
                entry,
            });
        }

        #[derive(PartialEq)]
        struct OrdDist(f64);
        impl Eq for OrdDist {}
        impl PartialOrd for OrdDist {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for OrdDist {
            fn cmp(&self, other: &Self) -> Ordering {
                self.0.partial_cmp(&other.0).unwrap()
            }
        }

        let mut results: BinaryHeap<(OrdDist, &Point2D<T>)> = BinaryHeap::new();

        while let Some(KnnCandidate { dist, entry }) = heap.pop() {
            if results.len() >= k {
                if let Some(worst_result) = results.peek() {
                    if dist > worst_result.0 .0 {
                        break;
                    }
                }
            }

            match entry {
                RTreeEntry::Leaf { object, .. } => {
                    let d_sq = M::distance_sq(query, object);
                    if results.len() < k {
                        results.push((OrdDist(d_sq), object));
                    } else if d_sq < results.peek().unwrap().0 .0 {
                        results.pop();
                        results.push((OrdDist(d_sq), object));
                    }
                }
                RTreeEntry::Node { child, .. } => {
                    for child_entry in &child.entries {
                        let d_sq = child_entry.mbr().min_distance(query).powi(2);
                        if results.len() < k || d_sq < results.peek().unwrap().0 .0 {
                            heap.push(KnnCandidate {
                                dist: d_sq,
                                entry: child_entry,
                            });
                        }
                    }
                }
            }
        }

        let mut sorted_results = results.into_vec();
        sorted_results.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        sorted_results.into_iter().map(|r| r.1).collect()
    }
}

impl<T: std::fmt::Debug + Ord + Clone> RTree<Point3D<T>> {
    /// Performs a k‑nearest neighbor search on an R‑tree of 3D points.
    ///
    /// # Arguments
    ///
    /// * `query` - The 3D point to search near.
    /// * `k` - The number of nearest neighbors to return.
    ///
    /// # Returns
    ///
    /// A vector of references to the k nearest 3D points.
    ///
    /// # Note
    ///
    /// The pruning logic for the search is based on Euclidean distance. Custom distance metrics
    /// that are not compatible with Euclidean distance may lead to incorrect results or reduced
    /// performance.
    pub fn knn_search<M: DistanceMetric<Point3D<T>>>(
        &self,
        query: &Point3D<T>,
        k: usize,
    ) -> Vec<&Point3D<T>> {
        if k == 0 {
            return Vec::new();
        }

        let mut heap: BinaryHeap<KnnCandidate<Point3D<T>>> = BinaryHeap::new();
        for entry in &self.root.entries {
            let dist_sq = entry.mbr().min_distance(query).powi(2);
            heap.push(KnnCandidate {
                dist: dist_sq,
                entry,
            });
        }

        #[derive(PartialEq)]
        struct OrdDist(f64);
        impl Eq for OrdDist {}
        impl PartialOrd for OrdDist {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Ord for OrdDist {
            fn cmp(&self, other: &Self) -> Ordering {
                self.0.partial_cmp(&other.0).unwrap()
            }
        }

        let mut results: BinaryHeap<(OrdDist, &Point3D<T>)> = BinaryHeap::new();

        while let Some(KnnCandidate { dist, entry }) = heap.pop() {
            if results.len() >= k {
                if let Some(worst_result) = results.peek() {
                    if dist > worst_result.0 .0 {
                        break;
                    }
                }
            }

            match entry {
                RTreeEntry::Leaf { object, .. } => {
                    let d_sq = M::distance_sq(query, object);
                    if results.len() < k {
                        results.push((OrdDist(d_sq), object));
                    } else if d_sq < results.peek().unwrap().0 .0 {
                        results.pop();
                        results.push((OrdDist(d_sq), object));
                    }
                }
                RTreeEntry::Node { child, .. } => {
                    for child_entry in &child.entries {
                        let d_sq = child_entry.mbr().min_distance(query).powi(2);
                        if results.len() < k || d_sq < results.peek().unwrap().0 .0 {
                            heap.push(KnnCandidate {
                                dist: d_sq,
                                entry: child_entry,
                            });
                        }
                    }
                }
            }
        }

        let mut sorted_results = results.into_vec();
        sorted_results.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        sorted_results.into_iter().map(|r| r.1).collect()
    }
}

impl<T> RTree<T>
where
    T: RTreeObject + PartialEq + std::fmt::Debug,
    T::B: BoundingVolumeFromPoint<T> + HasMinDistance<T> + Clone,
{
    /// Performs a range search on the R‑tree using a query object and radius.
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
    ///
    /// # Note
    ///
    /// The pruning logic for the search is based on Euclidean distance. Custom distance metrics
    /// that are not compatible with Euclidean distance may lead to incorrect results or reduced
    /// performance.
    pub fn range_search<M: DistanceMetric<T>>(&self, query: &T, radius: f64) -> Vec<&T> {
        let query_volume = T::B::from_point_radius(query, radius);
        let candidates = self.range_search_bbox(&query_volume);
        candidates
            .into_iter()
            .filter(|object| M::distance_sq(query, object) <= radius * radius)
            .collect()
    }
}
