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
//! use spart::kdtree::{KdPoint, KdTree};
//!
//! // Create a 2D Kd‑tree and insert some points.
//! let mut tree2d: KdTree<Point2D<()>> = KdTree::new();
//! tree2d.insert(Point2D::new(1.0, 2.0, None)).unwrap();
//! tree2d.insert(Point2D::new(3.0, 4.0, None)).unwrap();
//! let neighbors2d = tree2d.knn_search::<EuclideanDistance>(&Point2D::new(2.0, 3.0, None), 1);
//! assert!(!neighbors2d.is_empty());
//!
//! // Create a 3D Kd‑tree and insert some points.
//! let mut tree3d: KdTree<Point3D<()>> = KdTree::new();
//! tree3d.insert(Point3D::new(1.0, 2.0, 3.0, None)).unwrap();
//! tree3d.insert(Point3D::new(4.0, 5.0, 6.0, None)).unwrap();
//! let neighbors3d = tree3d.knn_search::<EuclideanDistance>(&Point3D::new(2.0, 3.0, 4.0, None), 1);
//! assert!(!neighbors3d.is_empty());
//! ```

use std::{cmp::Ordering, collections::BinaryHeap};

use ordered_float::OrderedFloat;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use tracing::info;

use crate::{errors::SpartError, geometry::DistanceMetric};

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
    /// # Errors
    ///
    /// Returns `SpartError::InvalidDimension` if the axis is invalid.
    fn coord(&self, axis: usize) -> Result<f64, SpartError>;
}

impl<T> KdPoint for crate::geometry::Point2D<T>
where
    T: std::fmt::Debug + Clone + PartialEq,
{
    fn dims(&self) -> usize {
        2
    }
    fn coord(&self, axis: usize) -> Result<f64, SpartError> {
        match axis {
            0 => Ok(self.x),
            1 => Ok(self.y),
            _ => Err(SpartError::InvalidDimension {
                requested: axis,
                available: 2,
            }),
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
    fn coord(&self, axis: usize) -> Result<f64, SpartError> {
        match axis {
            0 => Ok(self.x),
            1 => Ok(self.y),
            2 => Ok(self.z),
            _ => Err(SpartError::InvalidDimension {
                requested: axis,
                available: 3,
            }),
        }
    }
}

/// Internal structure used to store items in the k‑nearest neighbor heap.
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct KdTree<P: KdPoint> {
    root: Option<Box<KdNode<P>>>,
    k: Option<usize>,
}

impl<P: KdPoint> Default for KdTree<P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<P: KdPoint> KdTree<P> {
    /// Creates a new, empty Kd-tree.
    pub fn new() -> Self {
        KdTree {
            root: None,
            k: None,
        }
    }

    /// Creates a new, empty Kd-tree with the specified dimension.
    pub fn with_dimension(k: usize) -> Self {
        KdTree {
            root: None,
            k: Some(k),
        }
    }

    /// Returns true if the exact point exists in the tree.
    pub fn contains(&self, point: &P) -> bool {
        let k = match self.k {
            Some(k) => k,
            None => return false,
        };
        Self::contains_rec(&self.root, point, 0, k)
    }

    fn contains_rec(node: &Option<Box<KdNode<P>>>, point: &P, depth: usize, k: usize) -> bool {
        match node {
            None => false,
            Some(n) => {
                if n.point == *point {
                    return true;
                }
                let axis = depth % k;
                let p_coord = point
                    .coord(axis)
                    .unwrap_or_else(|_| unreachable!("axis computed from dims, must be valid"));
                let c_coord = n
                    .point
                    .coord(axis)
                    .unwrap_or_else(|_| unreachable!("axis computed from dims, must be valid"));
                if p_coord < c_coord {
                    Self::contains_rec(&n.left, point, depth + 1, k)
                } else if p_coord > c_coord {
                    Self::contains_rec(&n.right, point, depth + 1, k)
                } else {
                    // Equal on this axis, could be in either subtree.
                    Self::contains_rec(&n.right, point, depth + 1, k)
                        || Self::contains_rec(&n.left, point, depth + 1, k)
                }
            }
        }
    }

    /// Inserts a point into the Kd‑tree.
    ///
    /// If the tree is empty, the dimension of the tree is set to the dimension of the point.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to insert.
    ///
    /// # Errors
    ///
    /// Returns `SpartError::DimensionMismatch` if the point's dimension does not match
    /// the dimension of the tree.
    pub fn insert(&mut self, point: P) -> Result<(), SpartError> {
        let k = match self.k {
            Some(k) => {
                if point.dims() != k {
                    return Err(SpartError::DimensionMismatch {
                        expected: k,
                        actual: point.dims(),
                    });
                }
                k
            }
            None => {
                let k = point.dims();
                self.k = Some(k);
                k
            }
        };
        info!("Inserting point: {:?}", point);
        self.root = Some(Self::insert_rec(self.root.take(), point, 0, k));
        Ok(())
    }

    /// Inserts a bulk of points into the Kd-tree.
    ///
    /// # Arguments
    ///
    /// * `points` - The points to insert. This method takes ownership of the vector
    ///   to avoid mutating the caller's data (e.g., reordering during bulk build).
    ///
    /// # Errors
    ///
    /// Returns `SpartError::DimensionMismatch` if the points have inconsistent dimensions
    /// or conflict with the tree's dimension.
    pub fn insert_bulk(&mut self, mut points: Vec<P>) -> Result<(), SpartError> {
        if points.is_empty() {
            return Ok(());
        }
        let k = match self.k {
            Some(k) => k,
            None => {
                let k = points[0].dims();
                self.k = Some(k);
                k
            }
        };
        for p in &points {
            if p.dims() != k {
                return Err(SpartError::DimensionMismatch {
                    expected: k,
                    actual: p.dims(),
                });
            }
        }

        if self.root.is_some() {
            let mut existing = Vec::new();
            Self::collect_points(&self.root, &mut existing);
            points.extend(existing);
        }

        // Pass k explicitly to avoid unwraps inside recursion
        self.root = Self::insert_bulk_rec(&mut points[..], 0, k);
        Ok(())
    }

    fn collect_points(node: &Option<Box<KdNode<P>>>, result: &mut Vec<P>) {
        if let Some(n) = node {
            result.push(n.point.clone());
            Self::collect_points(&n.left, result);
            Self::collect_points(&n.right, result);
        }
    }

    fn insert_bulk_rec(points: &mut [P], depth: usize, k: usize) -> Option<Box<KdNode<P>>> {
        if points.is_empty() {
            return None;
        }

        let axis = depth % k;
        points.sort_by(|a, b| {
            let ac = a
                .coord(axis)
                .unwrap_or_else(|_| unreachable!("axis computed from dims, must be valid"));
            let bc = b
                .coord(axis)
                .unwrap_or_else(|_| unreachable!("axis computed from dims, must be valid"));
            ac.partial_cmp(&bc).unwrap_or(Ordering::Equal)
        });
        let median_idx = points.len() / 2;

        let mut node = KdNode::new(points[median_idx].clone());
        let (left_slice, right_slice) = points.split_at_mut(median_idx);
        let right_slice = &mut right_slice[1..];

        node.left = Self::insert_bulk_rec(left_slice, depth + 1, k);
        node.right = Self::insert_bulk_rec(right_slice, depth + 1, k);

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
            let p_coord = point
                .coord(axis)
                .unwrap_or_else(|_| unreachable!("axis computed from dims, must be valid"));
            let c_coord = current
                .point
                .coord(axis)
                .unwrap_or_else(|_| unreachable!("axis computed from dims, must be valid"));
            if p_coord < c_coord {
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
        if k_neighbors == 0 {
            return Vec::new();
        }
        let k = match self.k {
            Some(k) => k,
            None => return Vec::new(),
        };
        if target.dims() != k {
            return Vec::new();
        }
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
        result.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(Ordering::Equal));
        result.into_iter().map(|(_d, p)| p).collect()
    }

    fn knn_search_rec<M: DistanceMetric<P>>(
        node: &Option<Box<KdNode<P>>>,
        target: &P,
        k_neighbors: usize,
        depth: usize,
        heap: &mut BinaryHeap<HeapItem<P>>,
    ) {
        if let Some(n) = node {
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
            let target_coord = target
                .coord(axis)
                .unwrap_or_else(|_| unreachable!("axis computed from dims, must be valid"));
            let node_coord = n
                .point
                .coord(axis)
                .unwrap_or_else(|_| unreachable!("axis computed from dims, must be valid"));
            let (first, second) = if target_coord < node_coord {
                (&n.left, &n.right)
            } else {
                (&n.right, &n.left)
            };
            Self::knn_search_rec::<M>(first, target, k_neighbors, depth + 1, heap);
            let diff = (target_coord - node_coord).abs();
            let diff_sq = diff * diff;
            if heap.len() < k_neighbors
                || heap
                    .peek()
                    .map(|h| diff_sq < h.dist.into_inner())
                    .unwrap_or(true)
            {
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
        let k = match self.k {
            Some(k) => k,
            None => return Vec::new(),
        };
        if center.dims() != k {
            return Vec::new();
        }
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
        if let Some(n) = node {
            let dist_sq = M::distance_sq(center, &n.point);
            if dist_sq <= radius_sq {
                found.push(n.point.clone());
            }
            let axis = depth % center.dims();
            let center_coord = center
                .coord(axis)
                .unwrap_or_else(|_| unreachable!("axis computed from dims, must be valid"));
            let node_coord = n
                .point
                .coord(axis)
                .unwrap_or_else(|_| unreachable!("axis computed from dims, must be valid"));
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
        if self.root.is_none() {
            return false;
        }
        info!("Attempting to delete point: {:?}", point);
        let k = match self.k {
            Some(k) => k,
            None => return false,
        };
        let (new_root, deleted) = Self::delete_rec(self.root.take(), point, 0, k);
        self.root = new_root;
        if self.root.is_none() {
            self.k = None;
        }
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
                    // Delete a single instance: replace with successor from right subtree if available,
                    // otherwise promote left subtree, or remove leaf.
                    if let Some(right_subtree) = current.right.take() {
                        let successor = Self::find_min(&right_subtree, axis, depth + 1, k).clone();
                        let (new_right, _) =
                            Self::delete_rec(Some(right_subtree), &successor, depth + 1, k);
                        current.point = successor;
                        current.right = new_right;
                        (Some(current), true)
                    } else if let Some(left_subtree) = current.left.take() {
                        // Replace with min from left subtree on current axis, then delete that min
                        let successor = Self::find_min(&left_subtree, axis, depth + 1, k).clone();
                        let (mut new_left, _) =
                            Self::delete_rec(Some(left_subtree), &successor, depth + 1, k);
                        current.point = successor;
                        // As per standard kd-tree deletion, attach the adjusted left subtree as right child
                        current.right = new_left.take();
                        current.left = None;
                        (Some(current), true)
                    } else {
                        (None, true)
                    }
                } else {
                    let p_coord = point
                        .coord(axis)
                        .unwrap_or_else(|_| unreachable!("axis computed from dims, must be valid"));
                    let c_coord = current
                        .point
                        .coord(axis)
                        .unwrap_or_else(|_| unreachable!("axis computed from dims, must be valid"));

                    if p_coord < c_coord {
                        let (new_left, deleted) =
                            Self::delete_rec(current.left.take(), point, depth + 1, k);
                        current.left = new_left;
                        (Some(current), deleted)
                    } else if p_coord > c_coord {
                        let (new_right, deleted) =
                            Self::delete_rec(current.right.take(), point, depth + 1, k);
                        current.right = new_right;
                        (Some(current), deleted)
                    } else {
                        // Equal on this axis but not equal overall: the point could be in either subtree.
                        // Search right first, then left if not found.
                        let (new_right, deleted_right) =
                            Self::delete_rec(current.right.take(), point, depth + 1, k);
                        current.right = new_right;
                        if deleted_right {
                            (Some(current), true)
                        } else {
                            let (new_left, deleted_left) =
                                Self::delete_rec(current.left.take(), point, depth + 1, k);
                            current.left = new_left;
                            (Some(current), deleted_left)
                        }
                    }
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
                let left_c = left_min
                    .coord(d)
                    .unwrap_or_else(|_| unreachable!("axis computed from dims, must be valid"));
                let min_c = min
                    .coord(d)
                    .unwrap_or_else(|_| unreachable!("axis computed from dims, must be valid"));
                if left_c < min_c {
                    min = left_min;
                }
            }
        } else {
            if let Some(ref left) = node.left {
                let left_min = Self::find_min(left, d, depth + 1, k);
                let left_c = left_min
                    .coord(d)
                    .unwrap_or_else(|_| unreachable!("axis computed from dims, must be valid"));
                let min_c = min
                    .coord(d)
                    .unwrap_or_else(|_| unreachable!("axis computed from dims, must be valid"));
                if left_c < min_c {
                    min = left_min;
                }
            }
            if let Some(ref right) = node.right {
                let right_min = Self::find_min(right, d, depth + 1, k);
                let right_c = right_min
                    .coord(d)
                    .unwrap_or_else(|_| unreachable!("axis computed from dims, must be valid"));
                let min_c = min
                    .coord(d)
                    .unwrap_or_else(|_| unreachable!("axis computed from dims, must be valid"));
                if right_c < min_c {
                    min = right_min;
                }
            }
        }
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::{EuclideanDistance, Point2D, Point3D};

    #[test]
    fn test_insert_bulk_consecutive_preserves_points() {
        let mut tree: KdTree<Point2D<&str>> = KdTree::new();
        let first = vec![
            Point2D::new(1.0, 1.0, Some("A")),
            Point2D::new(2.0, 2.0, Some("B")),
        ];
        let second = vec![
            Point2D::new(3.0, 3.0, Some("C")),
            Point2D::new(4.0, 4.0, Some("D")),
        ];

        tree.insert_bulk(first.clone()).unwrap();
        tree.insert_bulk(second.clone()).unwrap();

        for p in first.into_iter().chain(second) {
            assert!(tree.contains(&p));
        }

        let target = Point2D::new(2.5, 2.5, None::<&str>);
        let knn = tree.knn_search::<EuclideanDistance>(&target, 4);
        assert_eq!(knn.len(), 4);
    }

    #[test]
    fn test_insert_bulk_dimension_mismatch() {
        let mut tree: KdTree<Point2D<()>> = KdTree::with_dimension(3);
        let points = vec![Point2D::new(1.0, 2.0, None)];
        let result = tree.insert_bulk(points);
        assert!(matches!(
            result,
            Err(SpartError::DimensionMismatch {
                expected: 3,
                actual: 2
            })
        ));
    }

    #[test]
    fn test_dimension_inference() {
        let mut tree: KdTree<Point2D<()>> = KdTree::new();
        let p = Point2D::new(1.0, 2.0, None);
        tree.insert(p).unwrap();
        let p2 = Point2D::new(3.0, 4.0, None);
        assert!(tree.insert(p2).is_ok());
    }

    #[test]
    fn test_empty_tree_queries() {
        let mut tree: KdTree<Point2D<&str>> = KdTree::new();
        let target = Point2D::new(1.0, 2.0, None::<&str>);

        let knn_results = tree.knn_search::<EuclideanDistance>(&target, 5);
        assert!(knn_results.is_empty());

        let range_results = tree.range_search::<EuclideanDistance>(&target, 10.0);
        assert!(range_results.is_empty());

        assert!(!tree.delete(&target));
    }

    #[test]
    fn test_knn_edge_cases() {
        let mut tree: KdTree<Point2D<&str>> = KdTree::new();
        let points = vec![
            Point2D::new(0.0, 0.0, Some("A")),
            Point2D::new(1.0, 1.0, Some("B")),
            Point2D::new(2.0, 2.0, Some("C")),
        ];
        let num_points = points.len();
        tree.insert_bulk(points).unwrap();

        let target = Point2D::new(0.5, 0.5, None::<&str>);
        let knn_results = tree.knn_search::<EuclideanDistance>(&target, 0);
        assert!(knn_results.is_empty());

        let knn_results = tree.knn_search::<EuclideanDistance>(&target, num_points + 5);
        assert_eq!(knn_results.len(), num_points);
    }

    #[test]
    fn test_range_zero_radius_exact_match() {
        let mut tree: KdTree<Point2D<&str>> = KdTree::new();
        let target = Point2D::new(10.0, 10.0, Some("A"));
        tree.insert(target.clone()).unwrap();
        tree.insert(Point2D::new(11.0, 11.0, Some("B"))).unwrap();

        let results = tree.range_search::<EuclideanDistance>(&target, 0.0);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], target);
    }

    #[test]
    fn test_duplicates_delete_one() {
        let mut tree: KdTree<Point2D<&str>> = KdTree::new();
        let p1 = Point2D::new(10.0, 10.0, Some("A"));
        let p2 = Point2D::new(10.0, 10.0, Some("A"));
        tree.insert(p1.clone()).unwrap();
        tree.insert(p2.clone()).unwrap();

        let target = Point2D::new(10.0, 10.0, None::<&str>);
        let results = tree.knn_search::<EuclideanDistance>(&target, 2);
        assert_eq!(results.len(), 2);

        assert!(tree.delete(&p1));

        let results_after_delete = tree.knn_search::<EuclideanDistance>(&target, 2);
        assert_eq!(results_after_delete.len(), 1);
    }

    #[test]
    fn test_delete_many() {
        let mut tree: KdTree<Point2D<&str>> = KdTree::new();
        let points = [
            Point2D::new(1.0, 2.0, Some("A")),
            Point2D::new(3.0, 4.0, Some("B")),
            Point2D::new(-1.0, -2.0, Some("C")),
            Point2D::new(1.5, 3.2, Some("D")),
            Point2D::new(0.5, 2.0, Some("E")),
            Point2D::new(0.25, 2.0, Some("F")),
            Point2D::new(0.5, 1.0, Some("G")),
        ];

        for p in points.clone() {
            tree.insert(p).unwrap();
        }

        for p in &points {
            assert!(tree.delete(p));
            let knn_after = tree.knn_search::<EuclideanDistance>(p, 2);
            for pt in &knn_after {
                assert_ne!(pt.data, p.data);
            }
        }
    }

    #[test]
    fn test_delete_same_coords_different_data() {
        let mut tree: KdTree<Point2D<&str>> = KdTree::new();
        let p1 = Point2D::new(10.0, 10.0, Some("A"));
        let p2 = Point2D::new(10.0, 10.0, Some("B"));
        let p3 = Point2D::new(10.0, 10.0, Some("C"));
        tree.insert(p1.clone()).unwrap();
        tree.insert(p2.clone()).unwrap();
        tree.insert(p3.clone()).unwrap();

        assert!(tree.delete(&p2));
        assert!(tree.contains(&p1));
        assert!(tree.contains(&p3));
        assert!(!tree.contains(&p2));

        let tgt = Point2D::new(10.0, 10.0, None::<&str>);
        let res = tree.knn_search::<EuclideanDistance>(&tgt, 3);
        assert_eq!(res.len(), 2);
        for r in res {
            assert_ne!(r.data, Some("B"));
        }
    }

    #[test]
    fn test_delete_nonexistent_with_equal_axis() {
        let mut tree: KdTree<Point2D<&str>> = KdTree::new();
        let a = Point2D::new(1.0, 0.0, Some("A"));
        let b = Point2D::new(1.0, 1.0, Some("B"));
        let c = Point2D::new(1.0, -1.0, Some("C"));
        tree.insert(a.clone()).unwrap();
        tree.insert(b.clone()).unwrap();
        tree.insert(c.clone()).unwrap();

        let not_present = Point2D::new(1.0, 2.0, Some("X"));
        assert!(!tree.delete(&not_present));
        assert!(tree.contains(&a));
        assert!(tree.contains(&b));
        assert!(tree.contains(&c));
    }

    #[test]
    fn test_delete_root_with_only_left() {
        let mut tree: KdTree<Point2D<&str>> = KdTree::new();
        let root = Point2D::new(5.0, 5.0, Some("R"));
        let l1 = Point2D::new(2.0, 2.0, Some("L1"));
        let l2 = Point2D::new(1.0, 1.0, Some("L2"));
        tree.insert(root.clone()).unwrap();
        tree.insert(l1.clone()).unwrap();
        tree.insert(l2.clone()).unwrap();

        assert!(tree.delete(&root));
        assert!(!tree.contains(&root));
        assert!(tree.contains(&l1));
        assert!(tree.contains(&l2));

        assert!(tree.delete(&l1));
        assert!(tree.contains(&l2));
    }

    #[test]
    fn test_delete_all_and_reinsert() {
        let mut tree: KdTree<Point2D<&str>> = KdTree::new();
        let pts = [
            Point2D::new(0.0, 0.0, Some("A")),
            Point2D::new(1.0, 1.0, Some("B")),
            Point2D::new(-1.0, -1.0, Some("C")),
        ];
        for p in pts.iter().cloned() {
            tree.insert(p).unwrap();
        }

        for p in &pts {
            assert!(tree.delete(p));
        }
        for p in &pts {
            assert!(!tree.delete(p));
        }

        let new_pts = [
            Point2D::new(2.0, 2.0, Some("D")),
            Point2D::new(3.0, 3.0, Some("E")),
        ];
        for p in new_pts.iter().cloned() {
            tree.insert(p).unwrap();
        }

        let tgt = Point2D::new(2.1, 2.1, None::<&str>);
        let res = tree.knn_search::<EuclideanDistance>(&tgt, 2);
        assert_eq!(res.len(), 2);
    }

    #[test]
    fn test_delete_many_equal_on_axis() {
        let mut tree: KdTree<Point2D<&str>> = KdTree::new();
        let pts = [
            Point2D::new(0.0, 0.0, Some("A")),
            Point2D::new(0.0, 1.0, Some("B")),
            Point2D::new(0.0, 2.0, Some("C")),
            Point2D::new(0.0, 3.0, Some("D")),
            Point2D::new(0.0, -1.0, Some("E")),
        ];
        for p in pts.iter().cloned() {
            tree.insert(p).unwrap();
        }

        for p in &pts {
            assert!(tree.delete(p));
            assert!(!tree.contains(p));
        }

        let tgt = Point2D::new(0.0, 0.0, None::<&str>);
        let res = tree.knn_search::<EuclideanDistance>(&tgt, 1);
        assert!(res.is_empty());
    }

    #[test]
    fn test_insert_bulk_3d_smoke() {
        let mut tree: KdTree<Point3D<&str>> = KdTree::new();
        let points = vec![
            Point3D::new(1.0, 2.0, 3.0, Some("A")),
            Point3D::new(4.0, 5.0, 6.0, Some("B")),
        ];
        tree.insert_bulk(points).unwrap();
        let target = Point3D::new(2.0, 3.0, 4.0, None::<&str>);
        let results = tree.knn_search::<EuclideanDistance>(&target, 1);
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_insert_bulk_empty_is_ok() {
        let mut tree: KdTree<Point2D<i32>> = KdTree::new();
        let result = tree.insert_bulk(Vec::new());
        assert!(result.is_ok());
    }

    #[test]
    fn test_knn_dimension_mismatch_returns_empty() {
        let tree: KdTree<Point2D<&str>> = KdTree::with_dimension(3);
        let target = Point2D::new(1.0, 2.0, None::<&str>);
        let results = tree.knn_search::<EuclideanDistance>(&target, 1);
        assert!(results.is_empty());
    }

    #[test]
    fn test_range_dimension_mismatch_returns_empty() {
        let tree: KdTree<Point2D<&str>> = KdTree::with_dimension(3);
        let target = Point2D::new(1.0, 2.0, None::<&str>);
        let results = tree.range_search::<EuclideanDistance>(&target, 1.0);
        assert!(results.is_empty());
    }
}
