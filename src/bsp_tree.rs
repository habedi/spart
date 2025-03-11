//! ## BSP‑tree implementation
//!
//! This module implements a binary space partitioning (BSP) tree for indexing 2D and 3D points.
//! Points stored in the tree must implement the `BSPTreeObject` trait, which requires an
//! associated bounding volume type (e.g. `Rectangle` for 2D objects or `Cube` for 3D objects).
//! The tree supports insertion, range search, deletion, and k‑nearest neighbor (kNN) search.
//!
//! The splitting of leaf nodes is based on the dimension with the largest extent (as determined
//! by the bounding volume’s `extent` method) and uses the median of points centers along that dimension.
//!
//! ### Example
//!
//! ```
//! use spart::geometry::{Point2D, Rectangle};
//! use spart::bsp_tree::{BSPTree, BSPTreeObject, Point2DBSP};
//!
//! // Create a BSPTree for 2D points (wrapped in a BSP object).
//! let mut tree: BSPTree<Point2DBSP<()>> = BSPTree::new(4);
//! let pt = Point2D::new(10.0, 20.0, None);
//! tree.insert(Point2DBSP { point: pt });
//!
//! // Perform a range search using a query point and radius.
//! let results = tree.range_search(&Point2DBSP { point: Point2D::new(10.0, 20.0, None) }, 5.0);
//! assert!(!results.is_empty());
//! ```

use crate::exceptions::SpartError;
use crate::geometry::{
    BSPBounds, BoundingVolume, BoundingVolumeFromPoint, Cube, HasMinDistance, Point2D, Point3D,
    Rectangle,
};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use tracing::{debug, info};

/// Trait for points that can be stored in a BSP tree and indexed by a bounding volume.
///
/// Each object must be debuggable and clonable, and must provide a minimum bounding volume.
pub trait BSPTreeObject: std::fmt::Debug + Clone {
    /// The type of bounding volume (e.g. `Rectangle` for 2D, `Cube` for 3D).
    type B: BoundingVolume + BSPBounds + Clone + std::fmt::Debug;
    /// Returns the minimum bounding volume (MBR) of the object.
    fn mbr(&self) -> Self::B;
}

/// Internal BSP tree node representation.
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
    /// Returns the node’s minimum bounding volume.
    fn get_mbr(&self) -> T::B {
        match self {
            BSPNode::Leaf { mbr, .. } => mbr.clone(),
            BSPNode::Node { mbr, .. } => mbr.clone(),
        }
    }
}

/// BSP tree implementation.
#[derive(Debug)]
pub struct BSPTree<T: BSPTreeObject> {
    root: Option<BSPNode<T>>,
    max_objects: usize,
}

impl<T: BSPTreeObject> BSPTree<T>
where
    T: PartialEq,
{
    /// Creates a new BSP tree with the specified maximum number of objects per leaf.
    ///
    /// # Arguments
    ///
    /// * `max_objects` - The maximum number of objects allowed in a leaf node.
    ///
    /// # Panics
    ///
    /// Panics with `SpartError::InvalidCapacity` if `max_objects` is zero.
    pub fn new(max_objects: usize) -> Self {
        if max_objects == 0 {
            panic!("{}", SpartError::InvalidCapacity { capacity: 0 });
        }
        info!("Creating new BSPTree with max_objects: {}", max_objects);
        BSPTree {
            root: None,
            max_objects,
        }
    }

    /// Returns true if the given bounding volume is degenerate (all extents are zero).
    fn is_degenerate(b: &T::B) -> bool {
        let dims = <T::B as BSPBounds>::DIM;
        (0..dims).all(|dim| b.extent(dim) == 0.0)
    }

    /// Inserts an object into the BSP tree.
    ///
    /// # Arguments
    ///
    /// * `object` - The object to insert.
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

    /// Recursively inserts an object into the BSP tree.
    fn insert_rec(node: BSPNode<T>, object: T, obj_mbr: T::B, max_objects: usize) -> BSPNode<T> {
        match node {
            BSPNode::Leaf { mut objects, mbr } => {
                // Update the leaf's bounding volume to include the new object.
                let new_mbr = mbr.union(&obj_mbr);
                debug!(
                    "Inserting into leaf. Old mbr: {:?}, new object mbr: {:?}, new mbr: {:?}",
                    mbr, obj_mbr, new_mbr
                );
                objects.push(object);
                if objects.len() > max_objects {
                    // Check for degenerate bounding volume to avoid infinite splitting.
                    if Self::is_degenerate(&new_mbr) {
                        info!(
                            "Degenerate bounding volume detected in leaf; not splitting further."
                        );
                        return BSPNode::Leaf {
                            objects,
                            mbr: new_mbr,
                        };
                    }
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

    /// Splits a leaf node that has exceeded the maximum number of objects.
    ///
    /// The splitting dimension is chosen as the one with the largest extent. Objects are partitioned
    /// by the median of their centers along that dimension.
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

        // If the bounding volume is degenerate (all objects share the same coordinate along every dimension),
        // avoid splitting further to prevent infinite recursion.
        if max_extent == 0.0 {
            info!("Degenerate bounding volume detected; not splitting further.");
            return BSPNode::Leaf { objects, mbr };
        }

        // Compute the median along the best dimension.
        let mut centers: Vec<f64> = objects
            .iter()
            .map(|obj| obj.mbr().center(best_dim))
            .collect();
        centers.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let median = centers[centers.len() / 2];
        info!(
            "Chosen split dimension: {} with extent: {}",
            best_dim, max_extent
        );
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
            mbr: left_mbr.union(&right_mbr),
        }
    }

    /// Performs a range search using a bounding volume query.
    ///
    /// # Arguments
    ///
    /// * `query` - The bounding volume used for the search.
    ///
    /// # Returns
    ///
    /// A vector of references to objects whose bounding volumes intersect the query.
    pub fn range_search_bbox(&self, query: &T::B) -> Vec<&T> {
        info!("Starting range search with query: {:?}", query);
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            Self::range_search_rec(root, query, &mut result);
        }
        info!("Range search completed; found {} objects.", result.len());
        result
    }

    /// Recursive helper for range search.
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

    /// Deletes an object from the BSP tree.
    ///
    /// # Arguments
    ///
    /// * `object` - The object to delete.
    ///
    /// # Returns
    ///
    /// `true` if the object was found and deleted, otherwise `false`.
    pub fn delete(&mut self, object: &T) -> bool {
        info!("Attempting to delete object: {:?}", object);
        if let Some(root) = self.root.take() {
            let (new_root, found) = Self::delete_rec(root, object, self.max_objects);
            self.root = new_root;
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

    /// Recursively deletes an object from the BSP tree.
    fn delete_rec(node: BSPNode<T>, object: &T, max_objects: usize) -> (Option<BSPNode<T>>, bool) {
        match node {
            BSPNode::Leaf {
                mut objects,
                mbr: _,
            } => {
                let initial = objects.len();
                objects.retain(|obj| obj != object);
                let found = objects.len() != initial;
                if objects.is_empty() {
                    (None, found)
                } else {
                    let new_mbr = objects
                        .iter()
                        .skip(1)
                        .fold(objects[0].mbr(), |acc, obj| acc.union(&obj.mbr()));
                    (
                        Some(BSPNode::Leaf {
                            objects,
                            mbr: new_mbr,
                        }),
                        found,
                    )
                }
            }
            BSPNode::Node {
                split_dim,
                split_val,
                left,
                right,
                mbr: _,
            } => {
                let (new_left, found_left) = Self::delete_rec(*left, object, max_objects);
                let (new_right, found_right) = Self::delete_rec(*right, object, max_objects);
                let found = found_left || found_right;
                match (new_left, new_right) {
                    (None, None) => (None, found),
                    (Some(child), None) | (None, Some(child)) => (Some(child), found),
                    (Some(l), Some(r)) => {
                        let merged_node = match (l.clone(), r.clone()) {
                            (
                                BSPNode::Leaf {
                                    objects: mut objs_l,
                                    mbr: mbr_l,
                                },
                                BSPNode::Leaf {
                                    objects: objs_r,
                                    mbr: mbr_r,
                                },
                            ) => {
                                if objs_l.len() + objs_r.len() <= max_objects {
                                    objs_l.extend(objs_r);
                                    let new_mbr = objs_l
                                        .iter()
                                        .skip(1)
                                        .fold(objs_l[0].mbr(), |acc, obj| acc.union(&obj.mbr()));
                                    BSPNode::Leaf {
                                        objects: objs_l,
                                        mbr: new_mbr,
                                    }
                                } else {
                                    let new_mbr = mbr_l.union(&mbr_r);
                                    BSPNode::Node {
                                        split_dim,
                                        split_val,
                                        left: Box::new(l),
                                        right: Box::new(r),
                                        mbr: new_mbr,
                                    }
                                }
                            }
                            (l_node, r_node) => {
                                let new_mbr = l_node.get_mbr().union(&r_node.get_mbr());
                                BSPNode::Node {
                                    split_dim,
                                    split_val,
                                    left: Box::new(l_node),
                                    right: Box::new(r_node),
                                    mbr: new_mbr,
                                }
                            }
                        };
                        (Some(merged_node), found)
                    }
                }
            }
        }
    }
}

/// Candidate wrapper for kNN search in the BSP tree.
#[derive(Debug)]
enum BSPCandidate<'a, T: BSPTreeObject> {
    Node(&'a BSPNode<T>, f64),
    Leaf(&'a T, f64),
}

impl<T: BSPTreeObject> PartialEq for BSPCandidate<'_, T> {
    fn eq(&self, other: &Self) -> bool {
        self.distance().eq(&other.distance())
    }
}
impl<T: BSPTreeObject> Eq for BSPCandidate<'_, T> {}
impl<T: BSPTreeObject> PartialOrd for BSPCandidate<'_, T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T: BSPTreeObject> Ord for BSPCandidate<'_, T> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance().partial_cmp(&self.distance()).unwrap()
    }
}

impl<T: BSPTreeObject> BSPCandidate<'_, T> {
    fn distance(&self) -> f64 {
        match self {
            BSPCandidate::Node(_, d) => *d,
            BSPCandidate::Leaf(_, d) => *d,
        }
    }
}

/// Wrapper for a 2D point for use in the tree.
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

/// Wrapper for a 3D point for use in the tree.
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

// -----------------------------------------------------------------------
// Convenience: add a `range_search` method that accepts a query object
// and a radius. This converts the query into a bounding volume, calls
// `range_search_bbox`, and filters the results based on the exact distance.
// -----------------------------------------------------------------------

impl<T> BSPTree<T>
where
    T: BSPTreeObject + PartialEq + std::fmt::Debug,
    T::B: BoundingVolumeFromPoint<T> + HasMinDistance<T> + Clone,
{
    /// Performs a range search on the BSP tree using a query object and a radius.
    ///
    /// # Arguments
    ///
    /// * `query` - The query object.
    /// * `radius` - The search radius.
    ///
    /// # Returns
    ///
    /// A vector of references to objects within the specified radius.
    pub fn range_search(&self, query: &T, radius: f64) -> Vec<&T> {
        let query_volume = T::B::from_point_radius(query, radius);
        let candidates = self.range_search_bbox(&query_volume);
        candidates
            .into_iter()
            .filter(|object| object.mbr().min_distance(query) <= radius)
            .collect()
    }
}

// -----------------------------------------------------------------------
// Implementations to support queries using the BSP wrapper types.
// These impls allow a query of type Point2DBSP<T> or Point3DBSP<T> to be used
// with the BSP tree’s k‑NN and range search methods.
// -----------------------------------------------------------------------

impl<T: Clone + std::fmt::Debug + 'static> HasMinDistance<Point2DBSP<T>> for Rectangle {
    fn min_distance(&self, query: &Point2DBSP<T>) -> f64 {
        HasMinDistance::<Point2D<T>>::min_distance(self, &query.point)
    }
}

impl<T: Clone + std::fmt::Debug + 'static> BoundingVolumeFromPoint<Point2DBSP<T>> for Rectangle {
    fn from_point_radius(query: &Point2DBSP<T>, radius: f64) -> Self {
        Rectangle {
            x: query.point.x - radius,
            y: query.point.y - radius,
            width: 2.0 * radius,
            height: 2.0 * radius,
        }
    }
}

impl<T: Clone + std::fmt::Debug + 'static> HasMinDistance<Point3DBSP<T>> for Cube {
    fn min_distance(&self, query: &Point3DBSP<T>) -> f64 {
        HasMinDistance::<Point3D<T>>::min_distance(self, &query.point)
    }
}

impl<T: Clone + std::fmt::Debug + 'static> BoundingVolumeFromPoint<Point3DBSP<T>> for Cube {
    fn from_point_radius(query: &Point3DBSP<T>, radius: f64) -> Self {
        Cube {
            x: query.point.x - radius,
            y: query.point.y - radius,
            z: query.point.z - radius,
            width: 2.0 * radius,
            height: 2.0 * radius,
            depth: 2.0 * radius,
        }
    }
}

impl<T: BSPTreeObject> BSPTree<T>
where
    T: PartialEq,
{
    /// Performs a k‑nearest neighbor search on the BSP tree.
    ///
    /// # Arguments
    ///
    /// * `query` - The query object.
    /// * `k` - The number of nearest neighbors to return.
    ///
    /// # Returns
    ///
    /// A vector of references to the k nearest objects.
    pub fn knn_search<Q>(&self, query: &Q, k: usize) -> Vec<&T>
    where
        T::B: BoundingVolumeFromPoint<Q> + HasMinDistance<Q> + Clone,
        Q: std::fmt::Debug,
    {
        info!("Starting kNN search with query: {:?}, k: {}", query, k);
        let mut heap = BinaryHeap::new();
        let mut result = Vec::new();
        if let Some(ref root) = self.root {
            let d = root.get_mbr().min_distance(query);
            heap.push(BSPCandidate::Node(root, d));
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
                            let d = obj.mbr().min_distance(query);
                            heap.push(BSPCandidate::Leaf(obj, d));
                        }
                    }
                    BSPNode::Node { left, right, .. } => {
                        let d_left = left.get_mbr().min_distance(query);
                        let d_right = right.get_mbr().min_distance(query);
                        heap.push(BSPCandidate::Node(left, d_left));
                        heap.push(BSPCandidate::Node(right, d_right));
                    }
                },
            }
        }
        info!("kNN search completed; found {} objects.", result.len());
        result
    }
}
