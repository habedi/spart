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
//! use spart::rtree::{RTree, RTreeObject};
//!
//! // Create an R‑tree for 2D points.
//! let mut tree2d: RTree<Point2D<()>> = RTree::new(4).unwrap();
//! let pt2d: Point2D<()> = Point2D::new(10.0, 20.0, None);
//! tree2d.insert(pt2d);
//! let query_rect = Rectangle { x: 5.0, y: 15.0, width: 10.0, height: 10.0 };
//! let results = tree2d.range_search_bbox(&query_rect);
//! assert!(!results.is_empty());
//!
//! // Create an R‑tree for 3D points.
//! let mut tree3d: RTree<Point3D<()>> = RTree::new(4).unwrap();
//! let pt3d: Point3D<()> = Point3D::new(10.0, 20.0, 30.0, None);
//! tree3d.insert(pt3d);
//! let query_cube = Cube { x: 5.0, y: 15.0, z: 25.0, width: 10.0, height: 10.0, depth: 10.0 };
//! let results3d = tree3d.range_search_bbox(&query_cube);
//! assert!(!results3d.is_empty());
//! ```

use crate::errors::SpartError;
use crate::geometry::{
    BoundingVolume, BoundingVolumeFromPoint, Cube, DistanceMetric, HasMinDistance, Point2D,
    Point3D, Rectangle,
};
use crate::knn::KnnHeap;
use crate::rtree_common::{
    KnnCandidate, compute_group_mbr as common_compute_group_mbr,
    delete_entry as common_delete_entry, node_height as common_node_height,
    search_node as common_search_node,
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
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct RTree<T: RTreeObject> {
    root: RTreeNode<T>,
    max_entries: usize,
    min_entries: usize,
}

// Common trait implementations to unify algorithms across R-tree family.
impl<T: RTreeObject> crate::rtree_common::EntryAccess for RTreeEntry<T> {
    type BV = T::B;
    type Node = RTreeNode<T>;
    type Obj = T;

    fn mbr(&self) -> &Self::BV {
        RTreeEntry::mbr(self)
    }

    fn as_leaf_obj(&self) -> Option<&Self::Obj> {
        match self {
            RTreeEntry::Leaf { object, .. } => Some(object),
            _ => None,
        }
    }

    fn child(&self) -> Option<&<Self as crate::rtree_common::EntryAccess>::Node> {
        match self {
            RTreeEntry::Node { child, .. } => Some(child),
            _ => None,
        }
    }

    fn child_mut(&mut self) -> Option<&mut <Self as crate::rtree_common::EntryAccess>::Node> {
        match self {
            RTreeEntry::Node { child, .. } => Some(child),
            _ => None,
        }
    }

    fn set_mbr(&mut self, new_mbr: Self::BV) {
        if let RTreeEntry::Node { mbr, .. } = self {
            *mbr = new_mbr;
        }
    }

    fn into_child(self) -> Option<Box<<Self as crate::rtree_common::EntryAccess>::Node>>
    where
        Self: Sized,
    {
        match self {
            RTreeEntry::Node { child, .. } => Some(child),
            _ => None,
        }
    }
}

impl<T: RTreeObject> crate::rtree_common::NodeAccess for RTreeNode<T> {
    type Entry = RTreeEntry<T>;

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

impl<T: RTreeObject> RTree<T> {
    /// Creates a new R‑tree with the specified maximum number of entries per node.
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
        info!("Creating new RTree with max_entries: {}", max_entries);
        Ok(RTree {
            root: RTreeNode {
                entries: Vec::new(),
                is_leaf: true,
            },
            max_entries,
            min_entries: (max_entries as f64 * 0.4).ceil() as usize,
        })
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
        self.insert_entry_at(entry, 0);
    }

    /// Inserts `entry` into a node at height `target_height`, growing a new root if the old one
    /// splits. Object entries belong at height 0; entries recovered from a condensed subtree
    /// belong at the height they came from.
    fn insert_entry_at(&mut self, entry: RTreeEntry<T>, target_height: usize) {
        let height = common_node_height(&self.root);
        let target_height = target_height.min(height);
        let overflow = insert_entry_at_height(
            &mut self.root,
            entry,
            target_height,
            height,
            self.max_entries,
            self.min_entries,
        );
        if let Some(sibling) = overflow {
            self.grow_root(sibling);
        }
    }

    /// Adds one level on top of the tree after the root has been split in two.
    fn grow_root(&mut self, sibling: RTreeEntry<T>) {
        info!("Root overflowed; growing the tree by one level");
        let old_root = std::mem::replace(
            &mut self.root,
            RTreeNode {
                entries: Vec::with_capacity(2),
                is_leaf: false,
            },
        );
        match common_compute_group_mbr(&old_root.entries) {
            Some(mbr) => {
                self.root.entries.push(RTreeEntry::Node {
                    mbr,
                    child: Box::new(old_root),
                });
                self.root.entries.push(sibling);
            }
            None => {
                // The old root was left empty by the split, so the new sibling is the whole tree.
                self.root = match sibling {
                    RTreeEntry::Node { child, .. } => *child,
                    leaf => RTreeNode {
                        entries: vec![leaf],
                        is_leaf: true,
                    },
                };
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

    /// Inserts a bulk of objects into the R-tree.
    ///
    /// When the tree is still empty the objects are packed bottom-up into a uniformly deep tree,
    /// which is considerably cheaper than inserting them one by one. Objects are packed in the order
    /// given, so pre-sorting them by locality gives a tighter index. Into a tree that already holds
    /// objects they are inserted individually, because appending pre-built nodes to a populated
    /// tree cannot preserve a uniform depth.
    ///
    /// # Arguments
    ///
    /// * `objects` - The objects to insert.
    pub fn insert_bulk(&mut self, objects: Vec<T>) {
        if objects.is_empty() {
            return;
        }

        if self.root.entries.is_empty() {
            info!("Bulk loading {} objects into an empty RTree", objects.len());
            let entries = objects
                .into_iter()
                .map(|object| RTreeEntry::Leaf {
                    mbr: object.mbr(),
                    object,
                })
                .collect();
            self.root = pack_entries(entries, self.max_entries);
        } else {
            for object in objects {
                self.insert(object);
            }
        }
    }
}

/// Packs entries bottom-up into a uniformly deep tree.
///
/// Every node on a level is filled to `max_entries` before the next is started (the last node of a
/// level takes the remainder), and each level records whether its nodes are leaves, so leaf entries
/// only ever land in leaf nodes.
fn pack_entries<T: RTreeObject>(entries: Vec<RTreeEntry<T>>, max_entries: usize) -> RTreeNode<T> {
    let mut level = entries;
    let mut level_is_leaf = true;

    while level.len() > max_entries {
        let mut parents = Vec::with_capacity(level.len().div_ceil(max_entries));
        let mut remaining = level;
        while !remaining.is_empty() {
            let take = remaining.len().min(max_entries);
            let child = RTreeNode {
                entries: remaining.drain(..take).collect(),
                is_leaf: level_is_leaf,
            };
            if let Some(mbr) = common_compute_group_mbr(&child.entries) {
                parents.push(RTreeEntry::Node {
                    mbr,
                    child: Box::new(child),
                });
            }
        }
        level = parents;
        level_is_leaf = false;
    }

    RTreeNode {
        entries: level,
        is_leaf: level_is_leaf,
    }
}

/// Picks the child of `node` whose MBR needs the least enlargement to cover `mbr`, breaking ties
/// towards the smaller MBR.
fn choose_subtree<T: RTreeObject>(node: &RTreeNode<T>, mbr: &T::B) -> usize {
    let mut best_index = 0;
    let mut best_enlargement = f64::INFINITY;
    let mut best_area = f64::INFINITY;
    for (i, entry) in node.entries.iter().enumerate() {
        let candidate = entry.mbr();
        let enlargement = candidate.enlargement(mbr);
        let area = candidate.area();
        if enlargement < best_enlargement || (enlargement == best_enlargement && area < best_area) {
            best_index = i;
            best_enlargement = enlargement;
            best_area = area;
        }
    }
    best_index
}

/// Recomputes an entry's MBR from the child it points at.
fn refresh_mbr<T: RTreeObject>(entry: &mut RTreeEntry<T>) {
    if let RTreeEntry::Node { mbr, child } = entry {
        if let Some(new_mbr) = common_compute_group_mbr(&child.entries) {
            *mbr = new_mbr;
        }
    }
}

/// Inserts `entry` into the node at height `target_height` in the subtree rooted at `node`, whose
/// own height is `node_height`.
///
/// Returns an entry pointing at a freshly created sibling when `node` had to split. The caller owns
/// that sibling: an interior caller pushes it into its own entry list, and `RTree::insert_entry_at`
/// turns it into a new root. Splitting on the way back up is what keeps every leaf at the same
/// depth. Without it only the root would ever split, and the tree would stay two levels deep no
/// matter how many objects it held.
fn insert_entry_at_height<T: RTreeObject>(
    node: &mut RTreeNode<T>,
    entry: RTreeEntry<T>,
    target_height: usize,
    node_height: usize,
    max_entries: usize,
    min_entries: usize,
) -> Option<RTreeEntry<T>> {
    if node_height == target_height {
        debug!("Inserting entry into node at height {}", node_height);
        node.entries.push(entry);
    } else {
        let best_index = choose_subtree(node, entry.mbr());
        // Above leaf level a node only ever holds `Node` entries. If a tree ever turns up that
        // breaks the invariant, keep the entry reachable here rather than dropping it.
        let can_descend = matches!(node.entries.get(best_index), Some(RTreeEntry::Node { .. }));
        let overflow = if can_descend {
            let RTreeEntry::Node { child, .. } = &mut node.entries[best_index] else {
                unreachable!("checked by `can_descend`")
            };
            insert_entry_at_height(
                child,
                entry,
                target_height,
                node_height - 1,
                max_entries,
                min_entries,
            )
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

    if node.entries.len() <= max_entries {
        return None;
    }
    split_node(node, min_entries)
}

/// Splits an overfull node in place, returning an entry that points at the new sibling.
fn split_node<T: RTreeObject>(
    node: &mut RTreeNode<T>,
    min_entries: usize,
) -> Option<RTreeEntry<T>> {
    let entries = std::mem::take(&mut node.entries);
    let (group1, group2) = split_entries(entries, min_entries);
    node.entries = group1;
    let sibling = RTreeNode {
        entries: group2,
        is_leaf: node.is_leaf,
    };
    // `None` only when the second group came out empty, in which case there is nothing to move and
    // `node` already holds every entry.
    let mbr = common_compute_group_mbr(&sibling.entries)?;
    Some(RTreeEntry::Node {
        mbr,
        child: Box::new(sibling),
    })
}

/// Splits `entries` into two groups, each holding at least `min_entries` of them.
///
/// This is Guttman's quadratic split: the pair of entries wasting the most space when covered
/// together seeds the two groups, then the rest are assigned in order of how strongly they prefer
/// one group to the other. Because the `min_entries` floor is enforced for both groups, the usual
/// call with `max_entries + 1` entries cannot leave either group larger than `max_entries`.
fn split_entries<T: RTreeObject>(
    mut entries: Vec<RTreeEntry<T>>,
    min_entries: usize,
) -> (Vec<RTreeEntry<T>>, Vec<RTreeEntry<T>>) {
    if entries.len() < 2 {
        return (entries, Vec::new());
    }
    // Asking for more than half of the entries could never be satisfied by both groups.
    let min_entries = min_entries.clamp(1, entries.len() / 2);

    let (first_seed, second_seed) = pick_seeds(&entries);
    // Remove the later index first so the earlier one stays valid.
    let second = entries.remove(second_seed);
    let first = entries.remove(first_seed);
    let mut mbr1 = first.mbr().clone();
    let mut mbr2 = second.mbr().clone();
    let mut group1 = vec![first];
    let mut group2 = vec![second];

    while !entries.is_empty() {
        // Once a group can only just still reach `min_entries`, everything left belongs to it.
        if group1.len() + entries.len() == min_entries {
            group1.append(&mut entries);
            break;
        }
        if group2.len() + entries.len() == min_entries {
            group2.append(&mut entries);
            break;
        }

        let entry = entries.remove(pick_next(&entries, &mbr1, &mbr2));
        let enlargement1 = mbr1.enlargement(entry.mbr());
        let enlargement2 = mbr2.enlargement(entry.mbr());
        let to_first = match enlargement1.partial_cmp(&enlargement2) {
            Some(Ordering::Less) => true,
            Some(Ordering::Greater) => false,
            // Equal enlargement: prefer the smaller, then the emptier group.
            _ => match mbr1.area().partial_cmp(&mbr2.area()) {
                Some(Ordering::Less) => true,
                Some(Ordering::Greater) => false,
                _ => group1.len() <= group2.len(),
            },
        };
        if to_first {
            mbr1 = mbr1.union(entry.mbr());
            group1.push(entry);
        } else {
            mbr2 = mbr2.union(entry.mbr());
            group2.push(entry);
        }
    }

    (group1, group2)
}

/// Returns the indices of the two entries that waste the most space when covered by one volume.
fn pick_seeds<T: RTreeObject>(entries: &[RTreeEntry<T>]) -> (usize, usize) {
    let mut seeds = (0, 1);
    let mut worst_waste = f64::NEG_INFINITY;
    for i in 0..entries.len() {
        for j in (i + 1)..entries.len() {
            let a = entries[i].mbr();
            let b = entries[j].mbr();
            let waste = a.union(b).area() - a.area() - b.area();
            if waste > worst_waste {
                worst_waste = waste;
                seeds = (i, j);
            }
        }
    }
    seeds
}

/// Returns the index of the entry with the strongest preference between the two groups.
fn pick_next<T: RTreeObject>(entries: &[RTreeEntry<T>], mbr1: &T::B, mbr2: &T::B) -> usize {
    let mut best_index = 0;
    let mut best_preference = f64::NEG_INFINITY;
    for (i, entry) in entries.iter().enumerate() {
        let preference = (mbr1.enlargement(entry.mbr()) - mbr2.enlargement(entry.mbr())).abs();
        if preference > best_preference {
            best_preference = preference;
            best_index = i;
        }
    }
    best_index
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

    /// Drops levels off the top of the tree while the root has a single child, and turns an emptied
    /// internal root back into an empty leaf so later inserts have somewhere to go.
    fn condense_root(&mut self) {
        while !self.root.is_leaf && self.root.entries.len() == 1 {
            match self.root.entries.pop() {
                Some(RTreeEntry::Node { child, .. }) => self.root = *child,
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
    ///
    /// Kept as an inherent method for convenience; the logic lives in
    /// [`HasMinDistance`].
    pub fn min_distance<T>(&self, point: &Point2D<T>) -> f64 {
        HasMinDistance::min_distance(self, point)
    }

    /// Computes the squared minimum distance from this rectangle to a given 2D point.
    pub fn min_distance_sq<T>(&self, point: &Point2D<T>) -> f64 {
        HasMinDistance::min_distance_sq(self, point)
    }
}

impl Cube {
    /// Computes the minimum distance from this cube to a given 3D point.
    ///
    /// Kept as an inherent method for convenience; the logic lives in
    /// [`HasMinDistance`].
    pub fn min_distance<T>(&self, point: &Point3D<T>) -> f64 {
        HasMinDistance::min_distance(self, point)
    }

    /// Computes the squared minimum distance from this cube to a given 3D point.
    pub fn min_distance_sq<T>(&self, point: &Point3D<T>) -> f64 {
        HasMinDistance::min_distance_sq(self, point)
    }
}

impl<T: std::fmt::Debug + Clone> RTree<Point2D<T>> {
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
        let mut results = KnnHeap::new(k);
        // Best-first descent: always expand whichever pending entry is nearest to the query, so the
        // search can stop as soon as the nearest pending entry is farther than the worst result kept.
        let mut pending: BinaryHeap<KnnCandidate<RTreeEntry<Point2D<T>>>> = BinaryHeap::new();
        for entry in &self.root.entries {
            pending.push(KnnCandidate {
                dist: entry.mbr().min_distance_sq(query),
                entry,
            });
        }

        while let Some(KnnCandidate { dist, entry }) = pending.pop() {
            if dist > results.worst() {
                break;
            }
            match entry {
                RTreeEntry::Leaf { object, .. } => {
                    results.offer(M::distance_sq(query, object), object);
                }
                RTreeEntry::Node { child, .. } => {
                    for child_entry in &child.entries {
                        let child_dist = child_entry.mbr().min_distance_sq(query);
                        if child_dist <= results.worst() {
                            pending.push(KnnCandidate {
                                dist: child_dist,
                                entry: child_entry,
                            });
                        }
                    }
                }
            }
        }

        results.into_sorted_vec()
    }
}

impl<T: std::fmt::Debug + Clone> RTree<Point3D<T>> {
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
        let mut results = KnnHeap::new(k);
        // Best-first descent: always expand whichever pending entry is nearest to the query, so the
        // search can stop as soon as the nearest pending entry is farther than the worst result kept.
        let mut pending: BinaryHeap<KnnCandidate<RTreeEntry<Point3D<T>>>> = BinaryHeap::new();
        for entry in &self.root.entries {
            pending.push(KnnCandidate {
                dist: entry.mbr().min_distance_sq(query),
                entry,
            });
        }

        while let Some(KnnCandidate { dist, entry }) = pending.pop() {
            if dist > results.worst() {
                break;
            }
            match entry {
                RTreeEntry::Leaf { object, .. } => {
                    results.offer(M::distance_sq(query, object), object);
                }
                RTreeEntry::Node { child, .. } => {
                    for child_entry in &child.entries {
                        let child_dist = child_entry.mbr().min_distance_sq(query);
                        if child_dist <= results.worst() {
                            pending.push(KnnCandidate {
                                dist: child_dist,
                                entry: child_entry,
                            });
                        }
                    }
                }
            }
        }

        results.into_sorted_vec()
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
    use crate::geometry::EuclideanDistance;
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

    fn check(tree: &RTree<Point2D<u32>>, context: &str) -> usize {
        assert_structure(
            &tree.root,
            tree.max_entries,
            Some(tree.min_entries),
            context,
        )
    }

    /// The structural invariants must survive every single insert and delete. Before non-root nodes
    /// learned to split, this tree stayed two levels deep forever and both of its leaves grew
    /// without bound, turning every query into a scan of half the data.
    #[test]
    fn test_structure_survives_inserts_and_deletes() {
        let mut tree: RTree<Point2D<u32>> = RTree::new(MAX_ENTRIES).unwrap();
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

        let height = crate::rtree_common::node_height(&tree.root);
        assert!(
            height >= 3,
            "400 objects with max_entries={MAX_ENTRIES} cannot fit in a tree of height {height}"
        );

        for (i, point) in points.iter().enumerate() {
            assert!(tree.delete(point), "delete of point {i} failed");
            let reachable = check(&tree, &format!("after {} deletes", i + 1));
            assert_eq!(reachable, points.len() - i - 1);
        }
        assert!(tree.root.entries.is_empty());
        assert!(tree.root.is_leaf, "an emptied tree should be a leaf again");
    }

    /// Bulk loading must produce a uniformly deep tree, and a later bulk load into a populated tree
    /// must not staple mismatched entries onto the root.
    #[test]
    fn test_structure_survives_bulk_load() {
        let points = spiral(300);

        let mut packed: RTree<Point2D<u32>> = RTree::new(MAX_ENTRIES).unwrap();
        packed.insert_bulk(points.clone());
        // Packing fills nodes greedily, so the last node of a level may sit below min_entries.
        let reachable = assert_structure(&packed.root, MAX_ENTRIES, None, "after bulk load");
        assert_eq!(reachable, points.len());
        assert!(crate::rtree_common::node_height(&packed.root) >= 3);

        // Bulk into a tree that already holds objects, and single inserts after a bulk load.
        let mut mixed: RTree<Point2D<u32>> = RTree::new(MAX_ENTRIES).unwrap();
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
        let mut tree: RTree<Point2D<&str>> = RTree::new(4).unwrap();
        let target = Point2D::new(5.0, 5.0, Some("T"));
        tree.insert(target.clone());
        tree.insert(Point2D::new(5.0, 6.0, Some("N")));

        let results = tree.range_search::<EuclideanDistance>(&target, 0.0);
        assert_eq!(results.len(), 1);
        assert_eq!(*results[0], target);
    }

    #[test]
    fn test_range_search_bbox_filters_results() {
        let mut tree: RTree<Point2D<&str>> = RTree::new(4).unwrap();
        let inside = Point2D::new(1.0, 1.0, Some("I"));
        let outside = Point2D::new(20.0, 20.0, Some("O"));
        tree.insert(inside.clone());
        tree.insert(outside);

        let query = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 5.0,
            height: 5.0,
        };
        let results = tree.range_search_bbox(&query);
        assert_eq!(results.len(), 1);
        assert_eq!(*results[0], inside);
    }

    #[test]
    fn test_delete_removes_point_3d() {
        let mut tree: RTree<Point3D<&str>> = RTree::new(4).unwrap();
        let a = Point3D::new(1.0, 1.0, 1.0, Some("A"));
        let b = Point3D::new(2.0, 2.0, 2.0, Some("B"));
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
    fn test_delete_underflow() {
        let mut tree: RTree<Point2D<i32>> = RTree::new(4).unwrap();
        let points: Vec<_> = (0..10)
            .map(|i| Point2D::new(i as f64, i as f64, Some(i)))
            .collect();

        for p in &points {
            tree.insert(p.clone());
        }

        assert!(tree.delete(&points[0]));
        assert!(tree.delete(&points[1]));
        assert!(tree.delete(&points[2]));

        let all_points = tree.range_search_bbox(&crate::geometry::Rectangle {
            x: -1.0,
            y: -1.0,
            width: 12.0,
            height: 12.0,
        });
        assert_eq!(all_points.len(), 7);

        for point in points.iter().take(10).skip(3) {
            assert!(tree.delete(point));
        }

        let all_points_after_all_deleted = tree.range_search_bbox(&crate::geometry::Rectangle {
            x: -1.0,
            y: -1.0,
            width: 12.0,
            height: 12.0,
        });
        assert!(all_points_after_all_deleted.is_empty());
    }

    #[test]
    fn test_empty_tree_queries() {
        let mut tree: RTree<Point2D<&str>> = RTree::new(4).unwrap();
        let target = Point2D::new(5.0, 5.0, None::<&str>);

        let knn_results = tree.knn_search::<EuclideanDistance>(&target, 5);
        assert!(knn_results.is_empty());

        let range_results = tree.range_search::<EuclideanDistance>(&target, 10.0);
        assert!(range_results.is_empty());

        assert!(!tree.delete(&target));
    }

    #[test]
    fn test_knn_edge_cases() {
        let mut tree: RTree<Point2D<&str>> = RTree::new(4).unwrap();
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
        let mut tree: RTree<Point2D<&str>> = RTree::new(4).unwrap();
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
        let mut tree: RTree<Point2D<&str>> = RTree::new(4).unwrap();
        let target = Point2D::new(5.0, 5.0, Some("T"));
        tree.insert(target.clone());

        let results = tree.range_search::<EuclideanDistance>(&target, -1.0);
        assert!(results.is_empty());
    }
}
