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
    delete_entry as common_delete_entry, node_height as common_node_height,
    search_node as common_search_node,
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
        self.insert_entry_at(entry, 0);
    }

    /// Inserts `entry` into a node at height `target_height`, applying R*-tree overflow treatment
    /// on the way back up.
    ///
    /// Entries pulled out by forced reinsertion are queued instead of being pushed back down
    /// immediately, and each carries **the height it came from**. Re-attaching an entry at any other
    /// height either buries a subtree inside a leaf or an object inside an interior node, which
    /// destroys the uniform depth of the tree. Heights are counted from the leaves, so a queued
    /// height stays valid even when an earlier reinsertion grows a new root.
    fn insert_entry_at(&mut self, entry: RStarTreeEntry<T>, target_height: usize)
    where
        T: Clone,
        T::B: BSPBounds,
    {
        let mut state = InsertState {
            max_entries: self.max_entries,
            min_entries: self.min_entries,
            reinserted_heights: Vec::new(),
            pending: vec![(entry, target_height)],
        };

        while let Some((entry, entry_height)) = state.pending.pop() {
            let height = common_node_height(&self.root);
            let target_height = entry_height.min(height);
            let overflow = insert_recursive(
                &mut self.root,
                entry,
                target_height,
                height,
                true,
                &mut state,
            );
            if let Some(sibling) = overflow {
                self.grow_root(sibling);
            }
        }
    }

    /// Adds one level on top of the tree after the root has been split in two.
    fn grow_root(&mut self, sibling: RStarTreeEntry<T>) {
        info!("Root overflowed; growing the tree by one level");
        let old_root = std::mem::replace(
            &mut self.root,
            RStarTreeNode {
                entries: Vec::with_capacity(2),
                is_leaf: false,
            },
        );
        match common_compute_group_mbr(&old_root.entries) {
            Some(mbr) => {
                self.root.entries.push(RStarTreeEntry::Node {
                    mbr,
                    child: Box::new(old_root),
                });
                self.root.entries.push(sibling);
            }
            None => {
                // The old root was left empty by the split, so the new sibling is the whole tree.
                self.root = match sibling {
                    RStarTreeEntry::Node { child, .. } => *child,
                    leaf => RStarTreeNode {
                        entries: vec![leaf],
                        is_leaf: true,
                    },
                };
            }
        }
    }

    /// Drops levels off the top of the tree while the root has a single child, and turns an emptied
    /// internal root back into an empty leaf so later inserts have somewhere to go.
    fn condense_root(&mut self) {
        while !self.root.is_leaf && self.root.entries.len() == 1 {
            match self.root.entries.pop() {
                Some(RStarTreeEntry::Node { child, .. }) => self.root = *child,
                Some(other) => {
                    self.root.entries.push(other);
                    break;
                }
                None => break,
            }
        }
        if !self.root.is_leaf && self.root.entries.is_empty() {
            self.root.is_leaf = true;
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
    /// When the tree is still empty the objects are sorted along the first axis and packed
    /// bottom-up into a uniformly deep tree, which is far cheaper than the incremental R*-tree
    /// insert. Into a tree that already holds objects they are inserted individually, because
    /// appending pre-built nodes to a populated tree cannot preserve a uniform depth.
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

        if self.root.entries.is_empty() {
            info!(
                "Bulk loading {} objects into an empty RStarTree",
                objects.len()
            );
            let mut entries: Vec<RStarTreeEntry<T>> = objects
                .into_iter()
                .map(|object| RStarTreeEntry::Leaf {
                    mbr: object.mbr(),
                    object,
                })
                .collect();
            // Packing follows the entry order, so a spatial sort keeps the packed MBRs tight.
            sort_by_center(&mut entries, 0);
            self.root = pack_entries(entries, self.max_entries);
        } else {
            for object in objects {
                self.insert(object);
            }
        }
    }

    /// Number of levels in the tree; a tree whose root is a leaf has height 1.
    #[doc(hidden)]
    pub fn height(&self) -> usize {
        common_node_height(&self.root) + 1
    }
}

/// Mutable state threaded through a single R*-tree insertion.
struct InsertState<T: RStarTreeObject> {
    max_entries: usize,
    min_entries: usize,
    /// Heights at which forced reinsertion has already been used during this insertion. The
    /// R*-tree paper allows it at most once per height per inserted object; every later overflow at
    /// that height splits instead, which is what makes the insertion terminate.
    reinserted_heights: Vec<usize>,
    /// Entries removed by forced reinsertion, each with the height it must go back to.
    pending: Vec<(RStarTreeEntry<T>, usize)>,
}

/// Packs entries bottom-up into a uniformly deep tree.
///
/// Each level records whether its nodes are leaves, so leaf entries only ever land in leaf nodes.
fn pack_entries<T: RStarTreeObject>(
    entries: Vec<RStarTreeEntry<T>>,
    max_entries: usize,
) -> RStarTreeNode<T> {
    let mut level = entries;
    let mut level_is_leaf = true;

    while level.len() > max_entries {
        let mut parents = Vec::with_capacity(level.len().div_ceil(max_entries));
        let mut remaining = level;
        while !remaining.is_empty() {
            let take = remaining.len().min(max_entries);
            let child = RStarTreeNode {
                entries: remaining.drain(..take).collect(),
                is_leaf: level_is_leaf,
            };
            if let Some(mbr) = common_compute_group_mbr(&child.entries) {
                parents.push(RStarTreeEntry::Node {
                    mbr,
                    child: Box::new(child),
                });
            }
        }
        level = parents;
        level_is_leaf = false;
    }

    RStarTreeNode {
        entries: level,
        is_leaf: level_is_leaf,
    }
}

/// Recomputes an entry's MBR from the child it points at.
fn refresh_mbr<T: RStarTreeObject>(entry: &mut RStarTreeEntry<T>) {
    if let RStarTreeEntry::Node { mbr, child } = entry {
        if let Some(new_mbr) = common_compute_group_mbr(&child.entries) {
            *mbr = new_mbr;
        }
    }
}

/// Sorts entries by the centre of their MBR along `dim`.
fn sort_by_center<T: RStarTreeObject>(entries: &mut [RStarTreeEntry<T>], dim: usize)
where
    T::B: BSPBounds,
{
    entries.sort_by(|a, b| {
        let ca = a.mbr().center(dim).unwrap_or(0.0);
        let cb = b.mbr().center(dim).unwrap_or(0.0);
        ca.partial_cmp(&cb).unwrap_or(Ordering::Equal)
    });
}

/// R*-tree ChooseSubtree.
///
/// Just above leaf level the child whose overlap with its siblings grows least wins; higher up the
/// child needing the least enlargement wins. Ties are broken towards the smaller MBR.
///
/// Keys are computed once per candidate instead of inside a comparator, both because the overlap
/// term is quadratic in the entry count and because a comparator that recomputes them is not a
/// consistent ordering.
fn choose_subtree<T: RStarTreeObject>(node: &RStarTreeNode<T>, mbr: &T::B) -> usize {
    let children_are_leaves = matches!(
        node.entries.first(),
        Some(RStarTreeEntry::Node { child, .. }) if child.is_leaf
    );

    let mut best_index = 0;
    let mut best_key = (
        OrderedFloat(f64::INFINITY),
        OrderedFloat(f64::INFINITY),
        OrderedFloat(f64::INFINITY),
    );

    for (i, entry) in node.entries.iter().enumerate() {
        let candidate = entry.mbr();
        let overlap = if children_are_leaves {
            let enlarged = candidate.union(mbr);
            node.entries
                .iter()
                .enumerate()
                .filter(|(j, _)| *j != i)
                .map(|(_, sibling)| enlarged.overlap(sibling.mbr()))
                .sum::<f64>()
        } else {
            0.0
        };
        let key = (
            OrderedFloat(overlap),
            OrderedFloat(candidate.enlargement(mbr)),
            OrderedFloat(candidate.area()),
        );
        if key < best_key {
            best_key = key;
            best_index = i;
        }
    }

    best_index
}

/// Inserts `entry` into the node at height `target_height` in the subtree rooted at `node`, whose
/// own height is `node_height`.
///
/// Returns an entry pointing at a freshly created sibling when `node` had to split; interior callers
/// push it into their own entry list and the root caller turns it into a new level.
fn insert_recursive<T: RStarTreeObject + Clone>(
    node: &mut RStarTreeNode<T>,
    entry: RStarTreeEntry<T>,
    target_height: usize,
    node_height: usize,
    is_root: bool,
    state: &mut InsertState<T>,
) -> Option<RStarTreeEntry<T>>
where
    T::B: BSPBounds,
{
    if node_height == target_height {
        node.entries.push(entry);
    } else {
        let best_index = choose_subtree(node, entry.mbr());
        // Above leaf level a node only ever holds `Node` entries. If a tree ever turns up that
        // breaks the invariant, keep the entry reachable here rather than dropping it.
        let can_descend = matches!(
            node.entries.get(best_index),
            Some(RStarTreeEntry::Node { .. })
        );
        let overflow = if can_descend {
            let RStarTreeEntry::Node { child, .. } = &mut node.entries[best_index] else {
                unreachable!("checked by `can_descend`")
            };
            insert_recursive(child, entry, target_height, node_height - 1, false, state)
        } else {
            node.entries.push(entry);
            None
        };
        if can_descend {
            refresh_mbr(&mut node.entries[best_index]);
        }
        if let Some(sibling) = overflow {
            node.entries.push(sibling);
        }
    }

    if node.entries.len() <= state.max_entries {
        return None;
    }

    // Overflow treatment: the first overflow at a given height below the root pulls out the entries
    // furthest from the node's centre and queues them for reinsertion; a later overflow at the same
    // height, or any overflow at the root, splits instead.
    if !is_root && !state.reinserted_heights.contains(&node_height) {
        state.reinserted_heights.push(node_height);
        for entry in forced_reinsert(node, state.max_entries) {
            state.pending.push((entry, node_height));
        }
        return None;
    }

    split_node(node, state.min_entries)
}

/// Splits an overfull node in place, returning an entry that points at the new sibling.
fn split_node<T: RStarTreeObject + Clone>(
    node: &mut RStarTreeNode<T>,
    min_entries: usize,
) -> Option<RStarTreeEntry<T>>
where
    T::B: BSPBounds,
{
    let entries = std::mem::take(&mut node.entries);
    let (group1, group2) = split_entries(entries, min_entries);
    node.entries = group1;
    let sibling = RStarTreeNode {
        entries: group2,
        is_leaf: node.is_leaf,
    };
    // `None` only when the second group came out empty, in which case there is nothing to move and
    // `node` already holds every entry.
    let mbr = common_compute_group_mbr(&sibling.entries)?;
    Some(RStarTreeEntry::Node {
        mbr,
        child: Box::new(sibling),
    })
}

/// R*-tree ReInsert: removes the entries furthest from the node's centre, furthest first, so the
/// caller can put them back somewhere better.
///
/// Never removes so many that the node is left empty, and always removes at least one so that the
/// node it was called on is no longer overfull.
fn forced_reinsert<T: RStarTreeObject + Clone>(
    node: &mut RStarTreeNode<T>,
    max_entries: usize,
) -> Vec<RStarTreeEntry<T>>
where
    T::B: BSPBounds,
{
    let Some(node_mbr) = common_compute_group_mbr(&node.entries) else {
        return Vec::new();
    };
    let node_center: Vec<f64> = (0..T::B::DIM)
        .map(|d| node_mbr.center(d).unwrap_or(0.0))
        .collect();

    let mut ranked: Vec<(OrderedFloat<f64>, RStarTreeEntry<T>)> = node
        .entries
        .drain(..)
        .map(|entry| {
            let distance = (0..T::B::DIM)
                .map(|d| {
                    let center = entry.mbr().center(d).unwrap_or(0.0);
                    (center - node_center[d]).powi(2)
                })
                .sum::<f64>();
            (OrderedFloat(distance), entry)
        })
        .collect();
    // Furthest from the centre first.
    ranked.sort_by(|a, b| b.0.cmp(&a.0));

    let count = ((max_entries as f64 * 0.3).ceil() as usize)
        .clamp(1, ranked.len().saturating_sub(1).max(1));
    let mut removed = Vec::with_capacity(count);
    for (i, (_, entry)) in ranked.into_iter().enumerate() {
        if i < count {
            removed.push(entry);
        } else {
            node.entries.push(entry);
        }
    }
    removed
}

/// R*-tree split: pick the axis whose sorted distributions have the smallest total margin, then
/// along that axis the distribution with the least overlap (least total area on a tie).
///
/// Both groups are guaranteed at least `min_entries` entries, so the usual call with
/// `max_entries + 1` entries cannot leave either group larger than `max_entries`.
fn split_entries<T: RStarTreeObject + Clone>(
    mut entries: Vec<RStarTreeEntry<T>>,
    min_entries: usize,
) -> (Vec<RStarTreeEntry<T>>, Vec<RStarTreeEntry<T>>)
where
    T::B: BSPBounds,
{
    if entries.len() < 2 {
        return (entries, Vec::new());
    }
    // Both groups have to be able to reach the floor, so it can never exceed half the entries.
    let min_entries = min_entries.clamp(1, entries.len() / 2);
    let last_split = entries.len() - min_entries;

    let mut best_axis = 0;
    let mut smallest_margin = f64::INFINITY;
    for dim in 0..T::B::DIM {
        sort_by_center(&mut entries, dim);
        let mut margin = 0.0;
        for k in min_entries..=last_split {
            if let (Some(mbr1), Some(mbr2)) = (
                common_compute_group_mbr(&entries[..k]),
                common_compute_group_mbr(&entries[k..]),
            ) {
                margin += mbr1.margin() + mbr2.margin();
            }
        }
        if margin < smallest_margin {
            smallest_margin = margin;
            best_axis = dim;
        }
    }

    sort_by_center(&mut entries, best_axis);
    let mut best_split = min_entries;
    let mut best_key = (OrderedFloat(f64::INFINITY), OrderedFloat(f64::INFINITY));
    for k in min_entries..=last_split {
        let (Some(mbr1), Some(mbr2)) = (
            common_compute_group_mbr(&entries[..k]),
            common_compute_group_mbr(&entries[k..]),
        ) else {
            continue;
        };
        let key = (
            OrderedFloat(mbr1.overlap(&mbr2)),
            OrderedFloat(mbr1.area() + mbr2.area()),
        );
        if key < best_key {
            best_key = key;
            best_split = k;
        }
    }

    let group2 = entries.split_off(best_split);
    (entries, group2)
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
        let height = common_node_height(&self.root);
        let deleted = common_delete_entry(
            &mut self.root,
            object,
            &object_mbr,
            self.min_entries,
            height,
            &mut reinsert_list,
        );

        if deleted {
            // Each entry goes back in at the height it was detached from. Heights are counted from
            // the leaves, so they stay correct even if an earlier reinsertion grew a new root.
            for (entry, entry_height) in reinsert_list {
                self.insert_entry_at(entry, entry_height);
            }
            self.condense_root();
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
            let dist_sq = entry.mbr().min_distance_sq(query);
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
                        let d_sq = child_entry.mbr().min_distance_sq(query);
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
            let dist_sq = entry.mbr().min_distance_sq(query);
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
                        let d_sq = child_entry.mbr().min_distance_sq(query);
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
        if radius < 0.0 {
            return Vec::new();
        }
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
    use crate::geometry::{EuclideanDistance, Rectangle};
    use crate::rtree_common::assert_structure;

    const MAX_ENTRIES: usize = 4;

    fn spiral(n: u32) -> Vec<Point2D<u32>> {
        (0..n)
            .map(|i| {
                let a = i as f64 * 0.7;
                Point2D::new(a.sin() * 100.0, a.cos() * 100.0, Some(i))
            })
            .collect()
    }

    fn check(tree: &RStarTree<Point2D<u32>>, context: &str) -> usize {
        assert_structure(
            &tree.root,
            tree.max_entries,
            Some(tree.min_entries),
            context,
        )
    }

    /// The structural invariants must survive every single insert and delete.
    ///
    /// Forced reinsertion used to put the entries it pulled out back at level 0, which buried whole
    /// subtrees inside leaf nodes: by the eleventh insert a range search over this tree could only
    /// see 6 of the 11 objects, and by a thousand objects it could see one.
    #[test]
    fn test_structure_survives_inserts_and_deletes() {
        let mut tree: RStarTree<Point2D<u32>> = RStarTree::new(MAX_ENTRIES).unwrap();
        let points = spiral(400);

        for (i, point) in points.iter().enumerate() {
            tree.insert(point.clone());
            let reachable = check(&tree, &format!("after {} inserts", i + 1));
            assert_eq!(
                reachable,
                i + 1,
                "objects went missing after {} inserts",
                i + 1
            );
        }

        assert!(
            tree.height() >= 4,
            "400 objects with max_entries={MAX_ENTRIES} cannot fit in {} levels",
            tree.height()
        );

        for (i, point) in points.iter().enumerate() {
            assert!(tree.delete(point), "delete of point {i} failed");
            let reachable = check(&tree, &format!("after {} deletes", i + 1));
            assert_eq!(reachable, points.len() - i - 1);
        }
        assert!(tree.root.entries.is_empty());
        assert!(tree.root.is_leaf, "an emptied tree should be a leaf again");
    }

    #[test]
    fn test_structure_survives_bulk_load() {
        let points = spiral(300);

        let mut packed: RStarTree<Point2D<u32>> = RStarTree::new(MAX_ENTRIES).unwrap();
        packed.insert_bulk(points.clone());
        // Packing fills nodes greedily, so the last node of a level may sit below min_entries.
        let reachable = assert_structure(&packed.root, MAX_ENTRIES, None, "after bulk load");
        assert_eq!(reachable, points.len());
        assert!(packed.height() >= 4);

        let mut mixed: RStarTree<Point2D<u32>> = RStarTree::new(MAX_ENTRIES).unwrap();
        for point in points.iter().take(7) {
            mixed.insert(point.clone());
        }
        mixed.insert_bulk(points[7..40].to_vec());
        assert_eq!(
            assert_structure(&mixed.root, MAX_ENTRIES, None, "bulk onto populated tree"),
            40
        );
        for point in points.iter().skip(40).take(20) {
            mixed.insert(point.clone());
        }
        assert_eq!(
            assert_structure(&mixed.root, MAX_ENTRIES, None, "inserts after bulk"),
            60
        );
    }

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

    #[test]
    fn test_forced_reinsertion_height_and_contents() {
        let mut tree: RStarTree<Point2D<i32>> = RStarTree::new(4).unwrap();
        let points: Vec<_> = (0..5)
            .map(|i| Point2D::new(i as f64, i as f64, Some(i)))
            .collect();

        for p in &points {
            tree.insert(p.clone());
        }

        assert_eq!(tree.height(), 2);

        for i in 5..10 {
            tree.insert(Point2D::new(i as f64, i as f64, Some(i)));
        }

        assert_eq!(tree.height(), 2);

        let all_points = tree.range_search_bbox(&Rectangle {
            x: -1.0,
            y: -1.0,
            width: 11.0,
            height: 11.0,
        });
        assert_eq!(all_points.len(), 10);
    }

    #[test]
    fn test_delete_underflow() {
        let mut tree: RStarTree<Point2D<i32>> = RStarTree::new(4).unwrap();
        let points: Vec<_> = (0..10)
            .map(|i| Point2D::new(i as f64, i as f64, Some(i)))
            .collect();

        for p in &points {
            tree.insert(p.clone());
        }

        assert!(tree.delete(&points[0]));
        assert!(tree.delete(&points[1]));
        assert!(tree.delete(&points[2]));

        let all_points = tree.range_search_bbox(&Rectangle {
            x: -1.0,
            y: -1.0,
            width: 12.0,
            height: 12.0,
        });
        assert_eq!(all_points.len(), 7);

        for point in points.iter().take(10).skip(3) {
            assert!(tree.delete(point));
        }

        let all_points_after_all_deleted = tree.range_search_bbox(&Rectangle {
            x: -1.0,
            y: -1.0,
            width: 12.0,
            height: 12.0,
        });
        assert!(all_points_after_all_deleted.is_empty());
    }

    #[test]
    fn test_empty_tree_queries() {
        let mut tree: RStarTree<Point2D<&str>> = RStarTree::new(4).unwrap();
        let target = Point2D::new(5.0, 5.0, None::<&str>);

        let knn_results = tree.knn_search::<EuclideanDistance>(&target, 5);
        assert!(knn_results.is_empty());

        let range_results = tree.range_search::<EuclideanDistance>(&target, 10.0);
        assert!(range_results.is_empty());

        assert!(!tree.delete(&target));
    }

    #[test]
    fn test_knn_edge_cases() {
        let mut tree: RStarTree<Point2D<&str>> = RStarTree::new(4).unwrap();
        let points = vec![
            Point2D::new(1.0, 1.0, Some("A")),
            Point2D::new(2.0, 2.0, Some("B")),
            Point2D::new(3.0, 3.0, Some("C")),
        ];
        let num_points = points.len();
        tree.insert_bulk(points.clone());

        let target = Point2D::new(1.5, 1.5, None::<&str>);
        let knn_results = tree.knn_search::<EuclideanDistance>(&target, 0);
        assert!(knn_results.is_empty());

        let knn_results = tree.knn_search::<EuclideanDistance>(&target, num_points + 5);
        assert_eq!(knn_results.len(), num_points);
    }

    #[test]
    fn test_duplicates_delete_one() {
        let mut tree: RStarTree<Point2D<&str>> = RStarTree::new(4).unwrap();
        let p1 = Point2D::new(10.0, 10.0, Some("A"));
        let p2 = p1.clone();
        tree.insert(p1.clone());
        tree.insert(p2.clone());

        let results = tree.knn_search::<EuclideanDistance>(&p1, 2);
        assert_eq!(results.len(), 2);

        assert!(tree.delete(&p1));

        let results_after_delete = tree.knn_search::<EuclideanDistance>(&p1, 2);
        assert_eq!(results_after_delete.len(), 1);
    }

    #[test]
    fn test_range_search_negative_radius_empty() {
        let mut tree: RStarTree<Point2D<&str>> = RStarTree::new(4).unwrap();
        let target = Point2D::new(5.0, 5.0, Some("T"));
        tree.insert(target.clone());

        let results = tree.range_search::<EuclideanDistance>(&target, -1.0);
        assert!(results.is_empty());
    }
}
