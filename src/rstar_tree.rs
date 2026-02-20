//! ## R*‑tree Implementation
//!
//! This module implements an R*‑tree for indexing 2D and 3D points.
//! The implementation supports insertion, deletion, range search, and k‑nearest
//! neighbor (kNN) search. Points stored in the R*‑tree must implement the `RStarTreeObject` trait,
//! which requires an implementation of a method to get a minimum bounding rectangle (for 2D)
//! or cube (for 3D) around the point.
//!
//! # Examples
//!
//! ```
//! use spart::geometry::{Point2D, Rectangle, Point3D, Cube};
//! use spart::rstar_tree::{RStarTree, RStarTreeObject};
//!
//! // Create an R*‑tree for 2D points.
//! let mut tree2d: RStarTree<Point2D<()>> = RStarTree::new(4).unwrap();
//! let pt2d: Point2D<()> = Point2D::new(10.0, 20.0, None);
//! tree2d.insert(pt2d);
//! let query_rect = Rectangle { x: 5.0, y: 15.0, width: 10.0, height: 10.0 };
//! let results = tree2d.range_search_bbox(&query_rect);
//! assert!(!results.is_empty());
//!
//! // Create an R*‑tree for 3D points.
//! let mut tree3d: RStarTree<Point3D<()>> = RStarTree::new(4).unwrap();
//! let pt3d: Point3D<()> = Point3D::new(10.0, 20.0, 30.0, None);
//! tree3d.insert(pt3d);
//! let query_cube = Cube { x: 5.0, y: 15.0, z: 25.0, width: 10.0, height: 10.0, depth: 10.0 };
//! let results3d = tree3d.range_search_bbox(&query_cube);
//! assert!(!results3d.is_empty());
//! ```

use crate::errors::SpartError;
use crate::geometry::{
    BSPBounds, BoundingVolume, BoundingVolumeFromPoint, Cube, DistanceMetric, HasMinDistance,
    Point2D, Point3D, Rectangle,
};
use crate::rtree_common::{
    KnnCandidate, compute_group_mbr as common_compute_group_mbr,
    delete_entry as common_delete_entry, search_node as common_search_node,
};
use ordered_float::OrderedFloat;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use tracing::info;

// Epsilon value for zero-sizes bounding boxes/cubes.
const EPSILON: f64 = 1e-10;

/// Trait for points stored in an R*‑tree.
///
/// Each object must provide its minimum bounding rectangle (or cube) via the `mbr()` method.
#[cfg(feature = "serde")]
pub trait RStarTreeObject: std::fmt::Debug + Clone {
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
pub trait RStarTreeObject: std::fmt::Debug + Clone {
    /// The type of the bounding volume (e.g. `Rectangle` for 2D objects or `Cube` for 3D objects).
    type B: BoundingVolume + std::fmt::Debug + Clone;
    /// Returns the minimum bounding volume of the object.
    fn mbr(&self) -> Self::B;
}

/// An entry in the R*‑tree, which can be either a leaf or a node.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum RStarTreeEntry<T: RStarTreeObject> {
    Leaf {
        mbr: T::B,
        object: T,
    },
    Node {
        mbr: T::B,
        child: Box<RStarTreeNode<T>>,
    },
}

impl<T: RStarTreeObject> RStarTreeEntry<T> {
    /// Returns a reference to the minimum bounding volume for this entry.
    pub fn mbr(&self) -> &T::B {
        match self {
            RStarTreeEntry::Leaf { mbr, .. } => mbr,
            RStarTreeEntry::Node { mbr, .. } => mbr,
        }
    }
}

/// A node in the R*‑tree.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RStarTreeNode<T: RStarTreeObject> {
    /// The entries stored in this node.
    pub entries: Vec<RStarTreeEntry<T>>,
    /// Indicates whether this node is a leaf.
    pub is_leaf: bool,
}

/// R*‑tree data structure for indexing 2D or 3D points.
///
/// The tree is initialized with a maximum number of entries per node. If a node exceeds this
/// number, it will split. The tree supports insertion, deletion, and range searches.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RStarTree<T: RStarTreeObject> {
    root: RStarTreeNode<T>,
    max_entries: usize,
    min_entries: usize,
}

// Common trait implementations for R*-tree to reuse shared algorithms.
impl<T: RStarTreeObject> crate::rtree_common::EntryAccess for RStarTreeEntry<T> {
    type BV = T::B;
    type Node = RStarTreeNode<T>;
    type Obj = T;

    fn mbr(&self) -> &Self::BV {
        RStarTreeEntry::mbr(self)
    }
    fn as_leaf_obj(&self) -> Option<&Self::Obj> {
        match self {
            RStarTreeEntry::Leaf { object, .. } => Some(object),
            _ => None,
        }
    }
    fn child(&self) -> Option<&<Self as crate::rtree_common::EntryAccess>::Node> {
        match self {
            RStarTreeEntry::Node { child, .. } => Some(child),
            _ => None,
        }
    }
    fn child_mut(&mut self) -> Option<&mut <Self as crate::rtree_common::EntryAccess>::Node> {
        match self {
            RStarTreeEntry::Node { child, .. } => Some(child),
            _ => None,
        }
    }
    fn set_mbr(&mut self, new_mbr: Self::BV) {
        if let RStarTreeEntry::Node { mbr, .. } = self {
            *mbr = new_mbr;
        }
    }
    fn into_child(self) -> Option<Box<<Self as crate::rtree_common::EntryAccess>::Node>>
    where
        Self: Sized,
    {
        match self {
            RStarTreeEntry::Node { child, .. } => Some(child),
            _ => None,
        }
    }
}

impl<T: RStarTreeObject> crate::rtree_common::NodeAccess for RStarTreeNode<T> {
    type Entry = RStarTreeEntry<T>;
    fn is_leaf(&self) -> bool {
        self.is_leaf
    }
    fn entries(&self) -> &Vec<Self::Entry> {
        &self.entries
    }
    fn entries_mut(&mut self) -> &mut Vec<Self::Entry> {
        &mut self.entries
    }
}

impl<T: RStarTreeObject> RStarTree<T> {
    /// Creates a new R*‑tree with the specified maximum number of entries per node.
    ///
    /// # Arguments
    ///
    /// * `max_entries` - The maximum number of entries allowed in a node.
    ///
    /// # Errors
    ///
    /// Returns `SpartError::InvalidCapacity` if `max_entries` is less than 2.
    pub fn new(max_entries: usize) -> Result<Self, SpartError> {
        if max_entries < 2 {
            return Err(SpartError::InvalidCapacity {
                capacity: max_entries,
            });
        }
        info!("Creating new RStarTree with max_entries: {}", max_entries);
        Ok(RStarTree {
            root: RStarTreeNode {
                entries: Vec::new(),
                is_leaf: true,
            },
            max_entries,
            min_entries: (max_entries as f64 * 0.4).ceil() as usize,
        })
    }

    /// Inserts an object into the R*‑tree.
    ///
    /// # Arguments
    ///
    /// * `object` - The object to insert.
    pub fn insert(&mut self, object: T)
    where
        T: Clone,
        T::B: BSPBounds,
    {
        info!("Inserting object into RStarTree: {:?}", object);
        let entry = RStarTreeEntry::Leaf {
            mbr: object.mbr(),
            object,
        };
        self.insert_entry(entry, None);
    }

    fn insert_entry(&mut self, entry: RStarTreeEntry<T>, reinsert_from_level: Option<usize>)
    where
        T: Clone,
        T::B: BSPBounds,
    {
        let mut to_insert = vec![(entry, 0)];
        let mut reinsert_level = reinsert_from_level;

        while let Some((item, level)) = to_insert.pop() {
            let overflow = insert_recursive(
                &mut self.root,
                item,
                self.max_entries,
                level,
                &mut reinsert_level,
                &mut to_insert,
            );

            if let Some((overflowed_node, overflow_level)) = overflow {
                if reinsert_level == Some(overflow_level) {
                    let old_entries = overflowed_node;
                    let (group1, group2) = split_entries(old_entries, self.max_entries);
                    let child1 = RStarTreeNode {
                        entries: group1,
                        is_leaf: self.root.is_leaf,
                    };
                    let child2 = RStarTreeNode {
                        entries: group2,
                        is_leaf: self.root.is_leaf,
                    };
                    let mbr1 = common_compute_group_mbr(&child1.entries)
                        .unwrap_or_else(|| unreachable!("non-empty group must have MBR"));
                    let mbr2 = common_compute_group_mbr(&child2.entries)
                        .unwrap_or_else(|| unreachable!("non-empty group must have MBR"));
                    self.root.is_leaf = false;
                    self.root.entries.clear();
                    self.root.entries.push(RStarTreeEntry::Node {
                        mbr: mbr1,
                        child: Box::new(child1),
                    });
                    self.root.entries.push(RStarTreeEntry::Node {
                        mbr: mbr2,
                        child: Box::new(child2),
                    });
                } else {
                    if reinsert_level.is_none() {
                        reinsert_level = Some(overflow_level);
                    }
                    let mut node = RStarTreeNode {
                        entries: overflowed_node,
                        is_leaf: self.root.is_leaf,
                    };
                    let reinserted_entries = forced_reinsert(&mut node, self.max_entries);
                    self.root.entries = node.entries;
                    for entry in reinserted_entries {
                        to_insert.push((entry, 0));
                    }
                }
            }
        }
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
        common_search_node(&self.root, query, &mut result);
        result
    }

    /// Inserts a bulk of objects into the R*-tree.
    ///
    /// # Arguments
    ///
    /// * `objects` - The objects to insert.
    pub fn insert_bulk(&mut self, objects: Vec<T>)
    where
        T: Clone,
        T::B: BSPBounds,
    {
        if objects.is_empty() {
            return;
        }

        let mut entries: Vec<RStarTreeEntry<T>> = objects
            .into_iter()
            .map(|obj| RStarTreeEntry::Leaf {
                mbr: obj.mbr(),
                object: obj,
            })
            .collect();

        while entries.len() > self.max_entries {
            let mut new_level_entries = Vec::new();
            let chunks = entries.chunks(self.max_entries);

            for chunk in chunks {
                let child_node = RStarTreeNode {
                    entries: chunk.to_vec(),
                    is_leaf: self.root.is_leaf,
                };
                if let Some(mbr) = common_compute_group_mbr(&child_node.entries) {
                    new_level_entries.push(RStarTreeEntry::Node {
                        mbr,
                        child: Box::new(child_node),
                    });
                }
            }
            entries = new_level_entries;
            self.root.is_leaf = false;
        }

        self.root.entries.extend(entries);
    }

    #[doc(hidden)]
    pub fn height(&self) -> usize {
        let mut height = 1;
        let mut current_node = &self.root;
        while !current_node.is_leaf {
            height += 1;
            current_node =
                if let Some(RStarTreeEntry::Node { child, .. }) = current_node.entries.first() {
                    child
                } else {
                    break;
                };
        }
        height
    }
}

fn choose_subtree<T: RStarTreeObject>(node: &RStarTreeNode<T>, entry: &RStarTreeEntry<T>) -> usize {
    let children_are_leaves = if let Some(RStarTreeEntry::Node { child, .. }) = node.entries.first()
    {
        child.is_leaf
    } else {
        false
    };

    if children_are_leaves {
        node.entries
            .iter()
            .enumerate()
            .min_by(|&(_, a), &(_, b)| {
                let mbr_a = a.mbr();
                let mbr_b = b.mbr();

                let overlap_a = node
                    .entries
                    .iter()
                    .filter(|e| !std::ptr::eq(*e, a))
                    .map(|e| e.mbr().union(entry.mbr()).overlap(e.mbr()))
                    .sum::<f64>();

                let overlap_b = node
                    .entries
                    .iter()
                    .filter(|e| !std::ptr::eq(*e, b))
                    .map(|e| e.mbr().union(entry.mbr()).overlap(e.mbr()))
                    .sum::<f64>();

                let overlap_cmp = overlap_a.partial_cmp(&overlap_b).unwrap_or(Ordering::Equal);
                if overlap_cmp != Ordering::Equal {
                    return overlap_cmp;
                }

                let enlargement_a = mbr_a.enlargement(entry.mbr());
                let enlargement_b = mbr_b.enlargement(entry.mbr());
                let enlargement_cmp = enlargement_a
                    .partial_cmp(&enlargement_b)
                    .unwrap_or(Ordering::Equal);
                if enlargement_cmp != Ordering::Equal {
                    return enlargement_cmp;
                }

                mbr_a
                    .area()
                    .partial_cmp(&mbr_b.area())
                    .unwrap_or(Ordering::Equal)
            })
            .map(|(i, _)| i)
            .unwrap_or(0)
    } else {
        node.entries
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                let mbr_a = a.mbr();
                let mbr_b = b.mbr();

                let enlargement_a = mbr_a.enlargement(entry.mbr());
                let enlargement_b = mbr_b.enlargement(entry.mbr());

                let enlargement_cmp = enlargement_a
                    .partial_cmp(&enlargement_b)
                    .unwrap_or(Ordering::Equal);
                if enlargement_cmp != Ordering::Equal {
                    return enlargement_cmp;
                }
                mbr_a
                    .area()
                    .partial_cmp(&mbr_b.area())
                    .unwrap_or(Ordering::Equal)
            })
            .map(|(i, _)| i)
            .unwrap_or(0)
    }
}

fn insert_recursive<T: RStarTreeObject + Clone>(
    node: &mut RStarTreeNode<T>,
    entry: RStarTreeEntry<T>,
    max_entries: usize,
    level: usize,
    reinsert_level: &mut Option<usize>,
    to_insert_queue: &mut Vec<(RStarTreeEntry<T>, usize)>,
) -> Option<(Vec<RStarTreeEntry<T>>, usize)>
where
    T::B: BSPBounds,
{
    if node.is_leaf {
        node.entries.push(entry);
    } else {
        let best_index = choose_subtree(node, &entry);
        let child = if let RStarTreeEntry::Node { child, .. } = &mut node.entries[best_index] {
            child
        } else {
            unreachable!()
        };

        if let Some((overflow, overflow_level)) = insert_recursive(
            child,
            entry,
            max_entries,
            level + 1,
            reinsert_level,
            to_insert_queue,
        ) {
            if reinsert_level.is_some() && *reinsert_level == Some(overflow_level) {
                let (g1, g2) = split_entries(overflow, max_entries);
                let child1 = RStarTreeNode {
                    entries: g1,
                    is_leaf: child.is_leaf,
                };
                let child2 = RStarTreeNode {
                    entries: g2,
                    is_leaf: child.is_leaf,
                };
                let mbr1 = common_compute_group_mbr(&child1.entries)
                    .unwrap_or_else(|| unreachable!("non-empty group must have MBR"));
                let mbr2 = common_compute_group_mbr(&child2.entries)
                    .unwrap_or_else(|| unreachable!("non-empty group must have MBR"));
                node.entries[best_index] = RStarTreeEntry::Node {
                    mbr: mbr1,
                    child: Box::new(child1),
                };
                node.entries.push(RStarTreeEntry::Node {
                    mbr: mbr2,
                    child: Box::new(child2),
                });
            } else {
                if reinsert_level.is_none() {
                    *reinsert_level = Some(overflow_level);
                }
                let mut overflowed_node = RStarTreeNode {
                    entries: overflow,
                    is_leaf: child.is_leaf,
                };
                let reinserted = forced_reinsert(&mut overflowed_node, max_entries);
                for item in reinserted {
                    to_insert_queue.push((item, 0));
                }
                if let RStarTreeEntry::Node { child, .. } = &mut node.entries[best_index] {
                    child.entries = overflowed_node.entries;
                }
            }
        }
        if let Some(new_mbr) = common_compute_group_mbr(
            if let RStarTreeEntry::Node { child, .. } = &node.entries[best_index] {
                &child.entries
            } else {
                unreachable!()
            },
        ) {
            if let RStarTreeEntry::Node { mbr, .. } = &mut node.entries[best_index] {
                *mbr = new_mbr;
            }
        }
    }

    if node.entries.len() > max_entries {
        return Some((std::mem::take(&mut node.entries), level));
    }
    None
}

fn forced_reinsert<T: RStarTreeObject + Clone>(
    node: &mut RStarTreeNode<T>,
    max_entries: usize,
) -> Vec<RStarTreeEntry<T>>
where
    T::B: BSPBounds,
{
    let node_mbr = if let Some(mbr) = common_compute_group_mbr(&node.entries) {
        mbr
    } else {
        return Vec::new();
    };
    let reinsert_count = (max_entries as f64 * 0.3).ceil() as usize;

    node.entries.sort_by(|a, b| {
        let center_a: Vec<f64> = (0..T::B::DIM)
            .map(|d| {
                a.mbr()
                    .center(d)
                    .unwrap_or_else(|_| unreachable!("dim valid"))
            })
            .collect();
        let center_b: Vec<f64> = (0..T::B::DIM)
            .map(|d| {
                b.mbr()
                    .center(d)
                    .unwrap_or_else(|_| unreachable!("dim valid"))
            })
            .collect();
        let node_center: Vec<f64> = (0..T::B::DIM)
            .map(|d| {
                node_mbr
                    .center(d)
                    .unwrap_or_else(|_| unreachable!("dim valid"))
            })
            .collect();

        let dist_a = center_a
            .iter()
            .zip(node_center.iter())
            .map(|(ca, cb)| (ca - cb).powi(2))
            .sum::<f64>();
        let dist_b = center_b
            .iter()
            .zip(node_center.iter())
            .map(|(ca, cb)| (ca - cb).powi(2))
            .sum::<f64>();

        dist_b.partial_cmp(&dist_a).unwrap_or(Ordering::Equal)
    });

    node.entries.drain(0..reinsert_count).collect()
}

fn split_entries<T: RStarTreeObject + Clone>(
    mut entries: Vec<RStarTreeEntry<T>>,
    max_entries: usize,
) -> (Vec<RStarTreeEntry<T>>, Vec<RStarTreeEntry<T>>)
where
    T::B: BSPBounds,
{
    let min_entries = (max_entries as f64 * 0.4).ceil() as usize;
    let mut best_axis = 0;
    let mut best_split_index = 0;
    let mut min_margin = f64::INFINITY;

    for dim in 0..T::B::DIM {
        entries.sort_by(|a, b| {
            let ca = a
                .mbr()
                .center(dim)
                .unwrap_or_else(|_| unreachable!("dim valid"));
            let cb = b
                .mbr()
                .center(dim)
                .unwrap_or_else(|_| unreachable!("dim valid"));
            ca.partial_cmp(&cb).unwrap_or(Ordering::Equal)
        });

        for k in min_entries..=entries.len() - min_entries {
            let group1 = &entries[..k];
            let group2 = &entries[k..];
            let mbr1 = common_compute_group_mbr(group1)
                .unwrap_or_else(|| unreachable!("non-empty group must have MBR"));
            let mbr2 = common_compute_group_mbr(group2)
                .unwrap_or_else(|| unreachable!("non-empty group must have MBR"));
            let margin = mbr1.margin() + mbr2.margin();
            if margin < min_margin {
                min_margin = margin;
                best_axis = dim;
                best_split_index = k;
            }
        }
    }

    entries.sort_by(|a, b| {
        let ca = a
            .mbr()
            .center(best_axis)
            .unwrap_or_else(|_| unreachable!("dim valid"));
        let cb = b
            .mbr()
            .center(best_axis)
            .unwrap_or_else(|_| unreachable!("dim valid"));
        ca.partial_cmp(&cb).unwrap_or(Ordering::Equal)
    });

    let mut best_overlap = f64::INFINITY;
    let mut best_area = f64::INFINITY;

    for k in min_entries..=entries.len() - min_entries {
        let group1 = &entries[..k];
        let group2 = &entries[k..];
        let mbr1 = common_compute_group_mbr(group1)
            .unwrap_or_else(|| unreachable!("non-empty group must have MBR"));
        let mbr2 = common_compute_group_mbr(group2)
            .unwrap_or_else(|| unreachable!("non-empty group must have MBR"));
        let overlap = mbr1.overlap(&mbr2);
        let area = mbr1.area() + mbr2.area();

        if overlap < best_overlap {
            best_overlap = overlap;
            best_area = area;
            best_split_index = k;
        } else if (overlap - best_overlap).abs() < EPSILON && area < best_area {
            best_area = area;
            best_split_index = k;
        }
    }

    let (group1, group2) = entries.split_at(best_split_index);
    (group1.to_vec(), group2.to_vec())
}

impl<T: RStarTreeObject> RStarTree<T>
where
    T: PartialEq + Clone,
    T::B: BSPBounds,
{
    /// Deletes an object from the R*‑tree.
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
        let deleted = common_delete_entry(
            &mut self.root,
            object,
            &object_mbr,
            self.min_entries,
            &mut reinsert_list,
        );

        if deleted {
            for entry in reinsert_list {
                self.insert_entry(entry, None);
            }

            if !self.root.is_leaf && self.root.entries.len() == 1 {
                if let Some(RStarTreeEntry::Node { child, .. }) = self.root.entries.pop() {
                    self.root = *child;
                }
            }
        }
        deleted
    }
}

impl<T: std::fmt::Debug + Clone> RStarTreeObject for Point2D<T> {
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

impl<T: std::fmt::Debug + Clone> RStarTreeObject for Point3D<T> {
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

impl<T: std::fmt::Debug + Clone> RStarTree<Point2D<T>> {
    /// Performs a k‑nearest neighbor search on an R*‑tree of 2D points.
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

        let mut heap: BinaryHeap<KnnCandidate<RStarTreeEntry<Point2D<T>>>> = BinaryHeap::new();
        for entry in &self.root.entries {
            let dist_sq = entry.mbr().min_distance(query).powi(2);
            heap.push(KnnCandidate {
                dist: dist_sq,
                entry,
            });
        }

        type OrdDist = OrderedFloat<f64>;
        #[inline]
        #[allow(non_snake_case)]
        fn OrdDist(x: f64) -> OrderedFloat<f64> {
            OrderedFloat(x)
        }

        struct HeapItem<'a, P> {
            key: OrdDist,
            idx: usize,
            obj: &'a P,
        }
        impl<P> PartialEq for HeapItem<'_, P> {
            fn eq(&self, other: &Self) -> bool {
                self.key == other.key && self.idx == other.idx
            }
        }
        impl<P> Eq for HeapItem<'_, P> {}
        impl<P> Ord for HeapItem<'_, P> {
            fn cmp(&self, other: &Self) -> Ordering {
                match self.key.cmp(&other.key) {
                    Ordering::Equal => self.idx.cmp(&other.idx),
                    ord => ord,
                }
            }
        }
        impl<P> PartialOrd for HeapItem<'_, P> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut results: BinaryHeap<HeapItem<Point2D<T>>> = BinaryHeap::new();
        let mut counter: usize = 0;

        while let Some(KnnCandidate { dist, entry }) = heap.pop() {
            if results.len() >= k {
                if let Some(worst_result) = results.peek() {
                    if dist > worst_result.key.0 {
                        break;
                    }
                }
            }

            match entry {
                RStarTreeEntry::Leaf { object, .. } => {
                    let d_sq = M::distance_sq(query, object);
                    if results.len() < k {
                        counter += 1;
                        results.push(HeapItem {
                            key: OrdDist(d_sq),
                            idx: counter,
                            obj: object,
                        });
                    } else if let Some(peek) = results.peek() {
                        if d_sq < peek.key.0 {
                            results.pop();
                            counter += 1;
                            results.push(HeapItem {
                                key: OrdDist(d_sq),
                                idx: counter,
                                obj: object,
                            });
                        }
                    }
                }
                RStarTreeEntry::Node { child, .. } => {
                    for child_entry in &child.entries {
                        let d_sq = child_entry.mbr().min_distance(query).powi(2);
                        if results.len() < k {
                            heap.push(KnnCandidate {
                                dist: d_sq,
                                entry: child_entry,
                            });
                        } else if let Some(peek) = results.peek() {
                            if d_sq < peek.key.0 {
                                heap.push(KnnCandidate {
                                    dist: d_sq,
                                    entry: child_entry,
                                });
                            }
                        }
                    }
                }
            }
        }

        let mut sorted_results = results.into_vec();
        sorted_results.sort_by(|a, b| a.key.partial_cmp(&b.key).unwrap_or(Ordering::Equal));
        sorted_results.into_iter().map(|r| r.obj).collect()
    }
}

impl<T: std::fmt::Debug + Clone> RStarTree<Point3D<T>> {
    /// Performs a k‑nearest neighbor search on an R*‑tree of 3D points.
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

        let mut heap: BinaryHeap<KnnCandidate<RStarTreeEntry<Point3D<T>>>> = BinaryHeap::new();
        for entry in &self.root.entries {
            let dist_sq = entry.mbr().min_distance(query).powi(2);
            heap.push(KnnCandidate {
                dist: dist_sq,
                entry,
            });
        }

        type OrdDist = OrderedFloat<f64>;
        #[inline]
        #[allow(non_snake_case)]
        fn OrdDist(x: f64) -> OrderedFloat<f64> {
            OrderedFloat(x)
        }

        struct HeapItem<'a, P> {
            key: OrdDist,
            idx: usize,
            obj: &'a P,
        }
        impl<P> PartialEq for HeapItem<'_, P> {
            fn eq(&self, other: &Self) -> bool {
                self.key == other.key && self.idx == other.idx
            }
        }
        impl<P> Eq for HeapItem<'_, P> {}
        impl<P> Ord for HeapItem<'_, P> {
            fn cmp(&self, other: &Self) -> Ordering {
                match self.key.cmp(&other.key) {
                    Ordering::Equal => self.idx.cmp(&other.idx),
                    ord => ord,
                }
            }
        }
        impl<P> PartialOrd for HeapItem<'_, P> {
            fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                Some(self.cmp(other))
            }
        }

        let mut results: BinaryHeap<HeapItem<Point3D<T>>> = BinaryHeap::new();
        let mut counter: usize = 0;

        while let Some(KnnCandidate { dist, entry }) = heap.pop() {
            if results.len() >= k {
                if let Some(worst_result) = results.peek() {
                    if dist > worst_result.key.0 {
                        break;
                    }
                }
            }

            match entry {
                RStarTreeEntry::Leaf { object, .. } => {
                    let d_sq = M::distance_sq(query, object);
                    if results.len() < k {
                        counter += 1;
                        results.push(HeapItem {
                            key: OrdDist(d_sq),
                            idx: counter,
                            obj: object,
                        });
                    } else if let Some(peek) = results.peek() {
                        if d_sq < peek.key.0 {
                            results.pop();
                            counter += 1;
                            results.push(HeapItem {
                                key: OrdDist(d_sq),
                                idx: counter,
                                obj: object,
                            });
                        }
                    }
                }
                RStarTreeEntry::Node { child, .. } => {
                    for child_entry in &child.entries {
                        let d_sq = child_entry.mbr().min_distance(query).powi(2);
                        if results.len() < k {
                            heap.push(KnnCandidate {
                                dist: d_sq,
                                entry: child_entry,
                            });
                        } else if let Some(peek) = results.peek() {
                            if d_sq < peek.key.0 {
                                heap.push(KnnCandidate {
                                    dist: d_sq,
                                    entry: child_entry,
                                });
                            }
                        }
                    }
                }
            }
        }

        let mut sorted_results = results.into_vec();
        sorted_results.sort_by(|a, b| a.key.partial_cmp(&b.key).unwrap_or(Ordering::Equal));
        sorted_results.into_iter().map(|r| r.obj).collect()
    }
}

impl<T> RStarTree<T>
where
    T: RStarTreeObject + PartialEq + std::fmt::Debug,
    T::B: BoundingVolumeFromPoint<T> + HasMinDistance<T> + Clone,
{
    /// Performs a range search on the R*‑tree using a query object and radius.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::EuclideanDistance;

    #[test]
    fn test_range_search_radius_zero_2d() {
        let mut tree: RStarTree<Point2D<&str>> = RStarTree::new(4).unwrap();
        let target = Point2D::new(5.0, 5.0, Some("T"));
        tree.insert(target.clone());
        tree.insert(Point2D::new(5.0, 6.0, Some("N")));

        let results = tree.range_search::<EuclideanDistance>(&target, 0.0);
        assert_eq!(results.len(), 1);
        assert_eq!(*results[0], target);
    }

    #[test]
    fn test_range_search_bbox_filters_results_3d() {
        let mut tree: RStarTree<Point3D<&str>> = RStarTree::new(4).unwrap();
        let inside = Point3D::new(1.0, 1.0, 1.0, Some("I"));
        let outside = Point3D::new(20.0, 20.0, 20.0, Some("O"));
        tree.insert(inside.clone());
        tree.insert(outside);

        let query = Cube {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            width: 5.0,
            height: 5.0,
            depth: 5.0,
        };
        let results = tree.range_search_bbox(&query);
        assert_eq!(results.len(), 1);
        assert_eq!(*results[0], inside);
    }

    #[test]
    fn test_delete_removes_point_2d() {
        let mut tree: RStarTree<Point2D<&str>> = RStarTree::new(4).unwrap();
        let a = Point2D::new(1.0, 1.0, Some("A"));
        let b = Point2D::new(2.0, 2.0, Some("B"));
        tree.insert(a.clone());
        tree.insert(b.clone());

        assert!(tree.delete(&a));
        let removed = tree.range_search::<EuclideanDistance>(&a, 0.0);
        let remaining = tree.range_search::<EuclideanDistance>(&b, 0.0);
        assert!(removed.is_empty());
        assert_eq!(remaining.len(), 1);
        assert_eq!(*remaining[0], b);
    }
}
