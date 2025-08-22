//! ## Kd‑tree Implementation
//!
//! This module provides a Kd‑tree implementation for indexing of points in 2D and 3D spaces.
//! Points must implement the `KdPoint` trait which provides access to coordinates and distance calculations.
//! The tree supports insertion, k‑nearest neighbor search (kNN), range search, and deletion.
//!
//! ### Example
//!
//! ```
//! use spart::geometry::{EuclideanDistance, Point2D, Point3D};
//! use spart::kd_tree::{KdPoint, KdTree};
//!
//! // Create a 2D Kd‑tree and insert some points.
//! let mut tree2d: KdTree<Point2D<()>> = KdTree::new(2);
//! tree2d.insert(Point2D::new(1.0, 2.0, None));
//! tree2d.insert(Point2D::new(3.0, 4.0, None));
//! let neighbors2d = tree2d.knn_search::<EuclideanDistance>(&Point2D::new(2.0, 3.0, None), 1);
//! assert!(!neighbors2d.is_empty());
//!
//! // Create a 3D Kd‑tree and insert some points.
//! let mut tree3d: KdTree<Point3D<()>> = KdTree::new(3);
//! tree3d.insert(Point3D::new(1.0, 2.0, 3.0, None));
//! tree3d.insert(Point3D::new(4.0, 5.0, 6.0, None));
//! let neighbors3d = tree3d.knn_search::<EuclideanDistance>(&Point3D::new(2.0, 3.0, 4.0, None), 1);
//! assert!(!neighbors3d.is_empty());
//! ```

use crate::exceptions::SpartError;
use crate::geometry::DistanceMetric;
use ordered_float::OrderedFloat;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use tracing::info;

/// Trait representing a point that can be stored in the Kd‑tree implementation.
///
/// A type implementing `KdPoint` must provide the number of dimensions,
/// a method to access a coordinate along a given axis, and a method to compute
/// the squared Euclidean distance to another point.
pub trait KdPoint: Clone + PartialEq + std::fmt::Debug {
    /// Returns the number of dimensions of the point.
    fn dims(&self) -> usize;
    /// Returns the coordinate along the specified axis.
    ///
    /// # Panics
    ///
    /// Panics with `SpartError::InvalidDimension` if the axis is invalid.
    fn coord(&self, axis: usize) -> f64;
}

impl<T> KdPoint for crate::geometry::Point2D<T>
where
    T: std::fmt::Debug + Clone + PartialEq,
{
    fn dims(&self) -> usize {
        2
    }
    fn coord(&self, axis: usize) -> f64 {
        match axis {
            0 => self.x,
            1 => self.y,
            _ => panic!(
                "{}",
                SpartError::InvalidDimension {
                    requested: axis,
                    available: 2
                }
            ),
        }
    }
}

impl<T> KdPoint for crate::geometry::Point3D<T>
where
    T: std::fmt::Debug + Clone + PartialEq,
{
    fn dims(&self) -> usize {
        3
    }
    fn coord(&self, axis: usize) -> f64 {
        match axis {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => panic!(
                "{}",
                SpartError::InvalidDimension {
                    requested: axis,
                    available: 3
                }
            ),
        }
    }
}

/// Internal structure used to store items in the k‑nearest neighbor heap.
#[derive(Debug)]
struct HeapItem<P> {
    dist: OrderedFloat<f64>,
    point: P,
}

impl<P> PartialEq for HeapItem<P> {
    fn eq(&self, other: &Self) -> bool {
        self.dist.eq(&other.dist)
    }
}

impl<P> Eq for HeapItem<P> {}

impl<P> PartialOrd for HeapItem<P> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<P> Ord for HeapItem<P> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist.cmp(&other.dist)
    }
}

/// A node in the Kd‑tree containing a point and references to its children.
#[derive(Debug)]
struct KdNode<P: KdPoint> {
    point: P,
    left: Option<Box<KdNode<P>>>,
    right: Option<Box<KdNode<P>>>,
}

impl<P: KdPoint> KdNode<P> {
    /// Creates a new Kd‑tree node with the given point.
    fn new(point: P) -> Self {
        KdNode {
            point,
            left: None,
            right: None,
        }
    }
}

/// Kd‑tree for points implementing `KdPoint`.
///
/// The tree stores points in k‑dimensional space (where `k` is provided during creation)
/// and supports insertion, k‑nearest neighbor search, range search, and deletion.
#[derive(Debug)]
pub struct KdTree<P: KdPoint> {
    root: Option<Box<KdNode<P>>>,
    k: usize,
}

impl<P: KdPoint> KdTree<P> {
    /// Creates a new Kd‑tree for points in `k` dimensions.
    ///
    /// # Arguments
    ///
    /// * `k` - The number of dimensions.
    ///
    /// # Panics
    ///
    /// Panics if `k` is zero.
    pub fn new(k: usize) -> Self {
        if k == 0 {
            panic!(
                "{}",
                SpartError::InvalidDimension {
                    requested: 0,
                    available: 0
                }
            );
        }
        KdTree { root: None, k }
    }

    /// Inserts a point into the Kd‑tree.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to insert.
    ///
    /// # Panics
    ///
    /// Panics if the point's dimension does not match the tree's dimension.
    pub fn insert(&mut self, point: P) {
        if point.dims() != self.k {
            panic!(
                "Point dimension {} does not match KDTree dimension {}",
                point.dims(),
                self.k
            );
        }
        info!("Inserting point: {:?}", point);
        self.root = Some(Self::insert_rec(self.root.take(), point, 0, self.k));
    }

    /// Inserts a bulk of points into the Kd-tree.
    ///
    /// # Arguments
    ///
    /// * `points` - The points to insert.
    pub fn insert_bulk(&mut self, points: &mut [P]) {
        if points.is_empty() {
            return;
        }
        self.root = self.insert_bulk_rec(points, 0);
    }

    fn insert_bulk_rec(&mut self, points: &mut [P], depth: usize) -> Option<Box<KdNode<P>>> {
        if points.is_empty() {
            return None;
        }

        let axis = depth % self.k;
        points.sort_by(|a, b| a.coord(axis).partial_cmp(&b.coord(axis)).unwrap());
        let median_idx = points.len() / 2;

        let mut node = KdNode::new(points[median_idx].clone());
        let (left_slice, right_slice) = points.split_at_mut(median_idx);
        let right_slice = &mut right_slice[1..];

        node.left = self.insert_bulk_rec(left_slice, depth + 1);
        node.right = self.insert_bulk_rec(right_slice, depth + 1);

        Some(Box::new(node))
    }

    fn insert_rec(
        node: Option<Box<KdNode<P>>>,
        point: P,
        depth: usize,
        k: usize,
    ) -> Box<KdNode<P>> {
        if let Some(mut current) = node {
            let axis = depth % k;
            if point.coord(axis) < current.point.coord(axis) {
                current.left = Some(Self::insert_rec(current.left.take(), point, depth + 1, k));
            } else {
                current.right = Some(Self::insert_rec(current.right.take(), point, depth + 1, k));
            }
            current
        } else {
            Box::new(KdNode::new(point))
        }
    }

    /// Performs a k‑nearest neighbor search for the given target point.
    ///
    /// # Arguments
    ///
    /// * `target` - The point to search around.
    /// * `k_neighbors` - The number of nearest neighbors to retrieve.
    ///
    /// # Returns
    ///
    /// A vector of the nearest points, ordered from nearest to farthest.
    pub fn knn_search<M: DistanceMetric<P>>(&self, target: &P, k_neighbors: usize) -> Vec<P> {
        info!(
            "Performing k‑NN search for target {:?} with k={}",
            target, k_neighbors
        );
        let mut heap: BinaryHeap<HeapItem<P>> = BinaryHeap::new();
        Self::knn_search_rec::<M>(&self.root, target, k_neighbors, 0, &mut heap);
        let mut result: Vec<(f64, P)> = heap
            .into_iter()
            .map(|item| (item.dist.into_inner(), item.point))
            .collect();
        result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        result.into_iter().map(|(_d, p)| p).collect()
    }

    fn knn_search_rec<M: DistanceMetric<P>>(
        node: &Option<Box<KdNode<P>>>,
        target: &P,
        k_neighbors: usize,
        depth: usize,
        heap: &mut BinaryHeap<HeapItem<P>>,
    ) {
        if let Some(ref n) = node {
            let dist_sq = M::distance_sq(target, &n.point);
            let dist = OrderedFloat(dist_sq);
            if heap.len() < k_neighbors {
                heap.push(HeapItem {
                    dist,
                    point: n.point.clone(),
                });
            } else if let Some(top) = heap.peek() {
                if dist < top.dist {
                    heap.pop();
                    heap.push(HeapItem {
                        dist,
                        point: n.point.clone(),
                    });
                }
            }
            let axis = depth % target.dims();
            let target_coord = target.coord(axis);
            let node_coord = n.point.coord(axis);
            let (first, second) = if target_coord < node_coord {
                (&n.left, &n.right)
            } else {
                (&n.right, &n.left)
            };
            Self::knn_search_rec::<M>(first, target, k_neighbors, depth + 1, heap);
            let diff = (target_coord - node_coord).abs();
            let diff_sq = diff * diff;
            if heap.len() < k_neighbors || diff_sq < heap.peek().unwrap().dist.into_inner() {
                Self::knn_search_rec::<M>(second, target, k_neighbors, depth + 1, heap);
            }
        }
    }

    /// Performs a range search, returning all points within the specified radius of the center.
    ///
    /// # Arguments
    ///
    /// * `center` - The center of the search.
    /// * `radius` - The search radius.
    ///
    /// # Returns
    ///
    /// A vector of points within the specified radius.
    pub fn range_search<M: DistanceMetric<P>>(&self, center: &P, radius: f64) -> Vec<P> {
        info!("Finding points within radius {} of {:?}", radius, center);
        let mut found = Vec::new();
        let radius_sq = radius * radius;
        Self::range_search_rec::<M>(&self.root, center, radius_sq, 0, radius, &mut found);
        found
    }

    fn range_search_rec<M: DistanceMetric<P>>(
        node: &Option<Box<KdNode<P>>>,
        center: &P,
        radius_sq: f64,
        depth: usize,
        radius: f64,
        found: &mut Vec<P>,
    ) {
        if let Some(ref n) = node {
            let dist_sq = M::distance_sq(center, &n.point);
            if dist_sq <= radius_sq {
                found.push(n.point.clone());
            }
            let axis = depth % center.dims();
            let center_coord = center.coord(axis);
            let node_coord = n.point.coord(axis);
            if center_coord - radius <= node_coord {
                Self::range_search_rec::<M>(&n.left, center, radius_sq, depth + 1, radius, found);
            }
            if center_coord + radius >= node_coord {
                Self::range_search_rec::<M>(&n.right, center, radius_sq, depth + 1, radius, found);
            }
        }
    }

    /// Deletes a point from the Kd‑tree.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to delete.
    ///
    /// # Returns
    ///
    /// `true` if the point was found and deleted, otherwise `false`.
    pub fn delete(&mut self, point: &P) -> bool {
        info!("Attempting to delete point: {:?}", point);
        let (new_root, deleted) = Self::delete_rec(self.root.take(), point, 0, self.k);
        self.root = new_root;
        deleted
    }

    fn delete_rec(
        node: Option<Box<KdNode<P>>>,
        point: &P,
        depth: usize,
        k: usize,
    ) -> (Option<Box<KdNode<P>>>, bool) {
        match node {
            None => (None, false),
            Some(mut current) => {
                let axis = depth % k;
                if current.point == *point {
                    if let Some(right_subtree) = current.right.take() {
                        let successor = Self::find_min(&right_subtree, axis, depth + 1, k).clone();
                        let (new_right, _) =
                            Self::delete_rec(Some(right_subtree), &successor, depth + 1, k);
                        current.point = successor;
                        current.right = new_right;
                        (Some(current), true)
                    } else if let Some(left_subtree) = current.left.take() {
                        (Some(left_subtree), true)
                    } else {
                        (None, true)
                    }
                } else if point.coord(axis) < current.point.coord(axis) {
                    let (new_left, deleted) =
                        Self::delete_rec(current.left.take(), point, depth + 1, k);
                    current.left = new_left;
                    (Some(current), deleted)
                } else {
                    let (new_right, deleted) =
                        Self::delete_rec(current.right.take(), point, depth + 1, k);
                    current.right = new_right;
                    (Some(current), deleted)
                }
            }
        }
    }

    fn find_min(node: &KdNode<P>, d: usize, depth: usize, k: usize) -> &P {
        let axis = depth % k;
        let mut min = &node.point;

        if axis == d {
            if let Some(ref left) = node.left {
                let left_min = Self::find_min(left, d, depth + 1, k);
                if left_min.coord(d) < min.coord(d) {
                    min = left_min;
                }
            }
        } else {
            if let Some(ref left) = node.left {
                let left_min = Self::find_min(left, d, depth + 1, k);
                if left_min.coord(d) < min.coord(d) {
                    min = left_min;
                }
            }
            if let Some(ref right) = node.right {
                let right_min = Self::find_min(right, d, depth + 1, k);
                if right_min.coord(d) < min.coord(d) {
                    min = right_min;
                }
            }
        }
        min
    }
}
