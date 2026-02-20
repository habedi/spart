//! ## Octree Implementation
//!
//! This module implements an Octree for indexing of 3D points. An octree recursively subdivides
//! a cubic region (defined by a `Cube`) into eight smaller subcubes when the number of points exceeds a specified capacity.
//! The octree provides operations for insertion, k-nearest neighbor (kNN) search, range search, and deletion.
//!
//! # Example
//!
//! ```
//! use spart::geometry::{Cube, EuclideanDistance, Point3D};
//! use spart::octree::Octree;
//!
//! // Define a cubic boundary for the octree.
//! let boundary = Cube { x: 0.0, y: 0.0, z: 0.0, width: 100.0, height: 100.0, depth: 100.0 };
//! // Create an octree with a capacity of 4 points per node.
//! let mut octree = Octree::new(&boundary, 4).unwrap();
//!
//! // Insert some points.
//! let pt1: Point3D<()> = Point3D::new(10.0, 20.0, 30.0, None);
//! let pt2: Point3D<()> = Point3D::new(50.0, 50.0, 50.0, None);
//! octree.insert(pt1);
//! octree.insert(pt2);
//!
//! // Perform a k-nearest neighbor search.
//! let neighbors = octree.knn_search::<EuclideanDistance>(&Point3D::new(12.0, 22.0, 32.0, None), 1);
//! assert!(!neighbors.is_empty());
//! ```

use crate::errors::SpartError;
use crate::geometry::{Cube, DistanceMetric, HeapItem, Point3D};
use ordered_float::OrderedFloat;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::BinaryHeap;
use tracing::info;

/// An octree for indexing of 3D points.
///
/// # Type Parameters
///
/// * `T`: The type of additional data stored in each point.
///
/// # Panics
///
/// Panics with `SpartError::InvalidCapacity` if `capacity` is zero.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Octree<T: Clone + PartialEq> {
    boundary: Cube,
    points: Vec<Point3D<T>>,
    capacity: usize,
    divided: bool,
    front_top_left: Option<Box<Octree<T>>>,
    front_top_right: Option<Box<Octree<T>>>,
    front_bottom_left: Option<Box<Octree<T>>>,
    front_bottom_right: Option<Box<Octree<T>>>,
    back_top_left: Option<Box<Octree<T>>>,
    back_top_right: Option<Box<Octree<T>>>,
    back_bottom_left: Option<Box<Octree<T>>>,
    back_bottom_right: Option<Box<Octree<T>>>,
}

impl<T: Clone + PartialEq + std::fmt::Debug> Octree<T> {
    /// Creates a new `Octree` with the specified boundary and capacity.
    ///
    /// # Arguments
    ///
    /// * `boundary` - The cube defining the 3D region covered by this octree.
    /// * `capacity` - The maximum number of points a node can hold before subdividing.
    ///
    /// # Errors
    ///
    /// Returns `SpartError::InvalidCapacity` if `capacity` is zero.
    pub fn new(boundary: &Cube, capacity: usize) -> Result<Self, SpartError> {
        if capacity == 0 {
            return Err(SpartError::InvalidCapacity { capacity });
        }
        info!(
            "Creating new Octree with boundary: {:?} and capacity: {}",
            boundary, capacity
        );
        Ok(Octree {
            boundary: boundary.clone(),
            points: Vec::new(),
            capacity,
            divided: false,
            front_top_left: None,
            front_top_right: None,
            front_bottom_left: None,
            front_bottom_right: None,
            back_top_left: None,
            back_top_right: None,
            back_bottom_left: None,
            back_bottom_right: None,
        })
    }

    /// Subdivides the current octree node into eight child octants.
    ///
    /// After subdivision, all existing points are reinserted into the appropriate children.
    fn subdivide(&mut self) {
        info!("Subdividing Octree at boundary: {:?}", self.boundary);
        let x = self.boundary.x;
        let y = self.boundary.y;
        let z = self.boundary.z;
        let w = self.boundary.width / 2.0;
        let h = self.boundary.height / 2.0;
        let d = self.boundary.depth / 2.0;

        self.front_top_left = Some(Box::new({
            let child = Octree::new(
                &Cube {
                    x,
                    y,
                    z,
                    width: w,
                    height: h,
                    depth: d,
                },
                self.capacity,
            );
            match child {
                Ok(c) => c,
                Err(_) => unreachable!("capacity validated at construction"),
            }
        }));
        self.front_top_right = Some(Box::new({
            let child = Octree::new(
                &Cube {
                    x: x + w,
                    y,
                    z,
                    width: w,
                    height: h,
                    depth: d,
                },
                self.capacity,
            );
            match child {
                Ok(c) => c,
                Err(_) => unreachable!("capacity validated at construction"),
            }
        }));
        self.front_bottom_left = Some(Box::new({
            let child = Octree::new(
                &Cube {
                    x,
                    y: y + h,
                    z,
                    width: w,
                    height: h,
                    depth: d,
                },
                self.capacity,
            );
            match child {
                Ok(c) => c,
                Err(_) => unreachable!("capacity validated at construction"),
            }
        }));
        self.front_bottom_right = Some(Box::new({
            let child = Octree::new(
                &Cube {
                    x: x + w,
                    y: y + h,
                    z,
                    width: w,
                    height: h,
                    depth: d,
                },
                self.capacity,
            );
            match child {
                Ok(c) => c,
                Err(_) => unreachable!("capacity validated at construction"),
            }
        }));
        self.back_top_left = Some(Box::new({
            let child = Octree::new(
                &Cube {
                    x,
                    y,
                    z: z + d,
                    width: w,
                    height: h,
                    depth: d,
                },
                self.capacity,
            );
            match child {
                Ok(c) => c,
                Err(_) => unreachable!("capacity validated at construction"),
            }
        }));
        self.back_top_right = Some(Box::new({
            let child = Octree::new(
                &Cube {
                    x: x + w,
                    y,
                    z: z + d,
                    width: w,
                    height: h,
                    depth: d,
                },
                self.capacity,
            );
            match child {
                Ok(c) => c,
                Err(_) => unreachable!("capacity validated at construction"),
            }
        }));
        self.back_bottom_left = Some(Box::new({
            let child = Octree::new(
                &Cube {
                    x,
                    y: y + h,
                    z: z + d,
                    width: w,
                    height: h,
                    depth: d,
                },
                self.capacity,
            );
            match child {
                Ok(c) => c,
                Err(_) => unreachable!("capacity validated at construction"),
            }
        }));
        self.back_bottom_right = Some(Box::new({
            let child = Octree::new(
                &Cube {
                    x: x + w,
                    y: y + h,
                    z: z + d,
                    width: w,
                    height: h,
                    depth: d,
                },
                self.capacity,
            );
            match child {
                Ok(c) => c,
                Err(_) => unreachable!("capacity validated at construction"),
            }
        }));
        self.divided = true;

        // Reinsert existing points into the appropriate children.
        let points = std::mem::take(&mut self.points);
        for point in points {
            self.insert(point);
        }
    }

    /// Returns mutable references to all eight child octants, if they exist.
    fn children_mut(&mut self) -> Vec<&mut Octree<T>> {
        let mut children = Vec::with_capacity(8);
        if let Some(ref mut child) = self.front_top_left {
            children.push(child.as_mut());
        }
        if let Some(ref mut child) = self.front_top_right {
            children.push(child.as_mut());
        }
        if let Some(ref mut child) = self.front_bottom_left {
            children.push(child.as_mut());
        }
        if let Some(ref mut child) = self.front_bottom_right {
            children.push(child.as_mut());
        }
        if let Some(ref mut child) = self.back_top_left {
            children.push(child.as_mut());
        }
        if let Some(ref mut child) = self.back_top_right {
            children.push(child.as_mut());
        }
        if let Some(ref mut child) = self.back_bottom_left {
            children.push(child.as_mut());
        }
        if let Some(ref mut child) = self.back_bottom_right {
            children.push(child.as_mut());
        }
        children
    }

    /// Returns references to all eight child octants, if they exist.
    fn children(&self) -> Vec<&Octree<T>> {
        let mut children = Vec::with_capacity(8);
        if let Some(ref child) = self.front_top_left {
            children.push(child.as_ref());
        }
        if let Some(ref child) = self.front_top_right {
            children.push(child.as_ref());
        }
        if let Some(ref child) = self.front_bottom_left {
            children.push(child.as_ref());
        }
        if let Some(ref child) = self.front_bottom_right {
            children.push(child.as_ref());
        }
        if let Some(ref child) = self.back_top_left {
            children.push(child.as_ref());
        }
        if let Some(ref child) = self.back_top_right {
            children.push(child.as_ref());
        }
        if let Some(ref child) = self.back_bottom_left {
            children.push(child.as_ref());
        }
        if let Some(ref child) = self.back_bottom_right {
            children.push(child.as_ref());
        }
        children
    }

    /// Computes the squared minimum distance from the given target point to the boundary of this node.
    ///
    /// This value is used to decide whether a subtree can be skipped during k-nearest neighbor search.
    ///
    /// # Arguments
    ///
    /// * `target` - The target 3D point.
    fn min_distance_sq(&self, target: &Point3D<T>) -> f64 {
        let tx = target.x;
        let ty = target.y;
        let tz = target.z;
        let cx = self.boundary.x;
        let cy = self.boundary.y;
        let cz = self.boundary.z;
        let cw = self.boundary.width;
        let ch = self.boundary.height;
        let cd = self.boundary.depth;

        let dx = if tx < cx {
            cx - tx
        } else if tx > cx + cw {
            tx - (cx + cw)
        } else {
            0.0
        };

        let dy = if ty < cy {
            cy - ty
        } else if ty > cy + ch {
            ty - (cy + ch)
        } else {
            0.0
        };

        let dz = if tz < cz {
            cz - tz
        } else if tz > cz + cd {
            tz - (cz + cd)
        } else {
            0.0
        };

        dx * dx + dy * dy + dz * dz
    }

    /// Inserts a 3D point into the octree.
    ///
    /// If the point is not within the boundary, it is ignored.
    /// If the current node is full, the node subdivides and attempts to insert the point into a child.
    ///
    /// # Arguments
    ///
    /// * `point` - The 3D point to insert.
    ///
    /// # Returns
    ///
    /// `true` if the point was successfully inserted, `false` otherwise.
    pub fn insert(&mut self, point: Point3D<T>) -> bool {
        if !self.boundary.contains(&point) {
            return false;
        }

        if !self.divided {
            if self.points.len() < self.capacity {
                self.points.push(point);
                return true;
            }
            self.subdivide();
        }

        if self
            .front_top_left
            .as_mut()
            .is_some_and(|c| c.insert(point.clone()))
        {
            return true;
        }
        if self
            .front_top_right
            .as_mut()
            .is_some_and(|c| c.insert(point.clone()))
        {
            return true;
        }
        if self
            .front_bottom_left
            .as_mut()
            .is_some_and(|c| c.insert(point.clone()))
        {
            return true;
        }
        if self
            .front_bottom_right
            .as_mut()
            .is_some_and(|c| c.insert(point.clone()))
        {
            return true;
        }
        if self
            .back_top_left
            .as_mut()
            .is_some_and(|c| c.insert(point.clone()))
        {
            return true;
        }
        if self
            .back_top_right
            .as_mut()
            .is_some_and(|c| c.insert(point.clone()))
        {
            return true;
        }
        if self
            .back_bottom_left
            .as_mut()
            .is_some_and(|c| c.insert(point.clone()))
        {
            return true;
        }
        if self
            .back_bottom_right
            .as_mut()
            .is_some_and(|c| c.insert(point.clone()))
        {
            return true;
        }

        unreachable!("A point within the parent boundary should always fit in a child boundary.");
    }

    /// Inserts a bulk of points into the octree.
    ///
    /// # Arguments
    ///
    /// * `points` - The points to insert.
    pub fn insert_bulk(&mut self, points: &[Point3D<T>]) {
        if points.is_empty() {
            return;
        }

        let points_within_boundary: Vec<Point3D<T>> = points
            .iter()
            .filter(|p| self.boundary.contains(p))
            .cloned()
            .collect();

        if points_within_boundary.is_empty() {
            return;
        }

        if !self.divided && self.points.len() + points_within_boundary.len() <= self.capacity {
            self.points.extend(points_within_boundary);
            return;
        }

        if !self.divided {
            self.subdivide();
        }

        let mut points_to_insert = points_within_boundary;
        if self.divided {
            let mut children_points: [Vec<Point3D<T>>; 8] = [
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
                vec![],
            ];

            for point in points_to_insert.drain(..) {
                if self
                    .front_top_left
                    .as_ref()
                    .map(|c| c.boundary.contains(&point))
                    .unwrap_or(false)
                {
                    children_points[0].push(point);
                } else if self
                    .front_top_right
                    .as_ref()
                    .map(|c| c.boundary.contains(&point))
                    .unwrap_or(false)
                {
                    children_points[1].push(point);
                } else if self
                    .front_bottom_left
                    .as_ref()
                    .map(|c| c.boundary.contains(&point))
                    .unwrap_or(false)
                {
                    children_points[2].push(point);
                } else if self
                    .front_bottom_right
                    .as_ref()
                    .map(|c| c.boundary.contains(&point))
                    .unwrap_or(false)
                {
                    children_points[3].push(point);
                } else if self
                    .back_top_left
                    .as_ref()
                    .map(|c| c.boundary.contains(&point))
                    .unwrap_or(false)
                {
                    children_points[4].push(point);
                } else if self
                    .back_top_right
                    .as_ref()
                    .map(|c| c.boundary.contains(&point))
                    .unwrap_or(false)
                {
                    children_points[5].push(point);
                } else if self
                    .back_bottom_left
                    .as_ref()
                    .map(|c| c.boundary.contains(&point))
                    .unwrap_or(false)
                {
                    children_points[6].push(point);
                } else if self
                    .back_bottom_right
                    .as_ref()
                    .map(|c| c.boundary.contains(&point))
                    .unwrap_or(false)
                {
                    children_points[7].push(point);
                }
            }

            if !children_points[0].is_empty() {
                if let Some(c) = self.front_top_left.as_mut() {
                    c.insert_bulk(&children_points[0]);
                }
            }
            if !children_points[1].is_empty() {
                if let Some(c) = self.front_top_right.as_mut() {
                    c.insert_bulk(&children_points[1]);
                }
            }
            if !children_points[2].is_empty() {
                if let Some(c) = self.front_bottom_left.as_mut() {
                    c.insert_bulk(&children_points[2]);
                }
            }
            if !children_points[3].is_empty() {
                if let Some(c) = self.front_bottom_right.as_mut() {
                    c.insert_bulk(&children_points[3]);
                }
            }
            if !children_points[4].is_empty() {
                if let Some(c) = self.back_top_left.as_mut() {
                    c.insert_bulk(&children_points[4]);
                }
            }
            if !children_points[5].is_empty() {
                if let Some(c) = self.back_top_right.as_mut() {
                    c.insert_bulk(&children_points[5]);
                }
            }
            if !children_points[6].is_empty() {
                if let Some(c) = self.back_bottom_left.as_mut() {
                    c.insert_bulk(&children_points[6]);
                }
            }
            if !children_points[7].is_empty() {
                if let Some(c) = self.back_bottom_right.as_mut() {
                    c.insert_bulk(&children_points[7]);
                }
            }
        }
    }

    /// Performs a k-nearest neighbor search for the target point.
    ///
    /// # Arguments
    ///
    /// * `target` - The 3D point for which to find the k nearest neighbors.
    /// * `k` - The number of nearest neighbors to retrieve.
    ///
    /// # Returns
    ///
    /// A vector of the k nearest 3D points, ordered from nearest to farthest.
    ///
    /// # Note
    ///
    /// The pruning logic for the search is based on Euclidean distance. Custom distance metrics
    /// that are not compatible with Euclidean distance may lead to incorrect results or reduced
    /// performance.
    pub fn knn_search<M: DistanceMetric<Point3D<T>>>(
        &self,
        target: &Point3D<T>,
        k: usize,
    ) -> Vec<Point3D<T>> {
        if k == 0 {
            return Vec::new();
        }
        let mut heap: BinaryHeap<HeapItem<T>> = BinaryHeap::new();
        self.knn_search_helper::<M>(target, k, &mut heap);
        heap.into_sorted_vec()
            .into_iter()
            .filter_map(|item| item.point_3d)
            .collect()
    }

    /// Helper method for recursively performing the k-nearest neighbor search.
    fn knn_search_helper<M: DistanceMetric<Point3D<T>>>(
        &self,
        target: &Point3D<T>,
        k: usize,
        heap: &mut BinaryHeap<HeapItem<T>>,
    ) {
        for point in &self.points {
            let dist_sq = M::distance_sq(point, target);
            let item = HeapItem {
                neg_distance: OrderedFloat(-dist_sq),
                point_2d: None,
                point_3d: Some(point.clone()),
            };
            heap.push(item);
            if heap.len() > k {
                heap.pop();
            }
        }
        if self.divided {
            for child in self.children() {
                if heap.len() == k {
                    if let Some(top) = heap.peek() {
                        let current_farthest = -top.neg_distance.into_inner();
                        if child.min_distance_sq(target) > current_farthest {
                            continue;
                        }
                    }
                }
                child.knn_search_helper::<M>(target, k, heap);
            }
        }
    }

    /// Performs a range search, returning all points within the specified radius of the center point.
    ///
    /// # Arguments
    ///
    /// * `center` - The center of the search range.
    /// * `radius` - The search radius.
    ///
    /// # Returns
    ///
    /// A vector of 3D points within the specified range.
    ///
    /// # Note
    ///
    /// The pruning logic for the search is based on Euclidean distance. Custom distance metrics
    /// that are not compatible with Euclidean distance may lead to incorrect results or reduced
    /// performance.
    pub fn range_search<M: DistanceMetric<Point3D<T>>>(
        &self,
        center: &Point3D<T>,
        radius: f64,
    ) -> Vec<Point3D<T>> {
        if radius < 0.0 {
            return Vec::new();
        }
        let mut found = Vec::new();
        let radius_sq = radius * radius;
        if self.min_distance_sq(center) > radius_sq {
            return found;
        }
        for point in &self.points {
            if M::distance_sq(point, center) <= radius_sq {
                found.push(point.clone());
            }
        }
        if self.divided {
            for child in self.children() {
                found.extend(child.range_search::<M>(center, radius));
            }
        }
        found
    }

    /// Deletes a point from the octree.
    ///
    /// Returns `true` if the point was found and deleted.
    ///
    /// # Arguments
    ///
    /// * `point` - The 3D point to delete.
    pub fn delete(&mut self, point: &Point3D<T>) -> bool {
        if !self.boundary.contains(point) {
            return false;
        }
        let mut deleted = false;
        if self.divided {
            for child in self.children_mut() {
                if child.delete(point) {
                    deleted = true;
                    break;
                }
            }
            self.try_merge();
            return deleted;
        }
        if let Some(pos) = self.points.iter().position(|p| p == point) {
            self.points.remove(pos);
            info!("Deleting point {:?} from Octree", point);
            true
        } else {
            false
        }
    }

    /// Attempts to merge child nodes back into the parent node if possible.
    ///
    /// If all children are not divided and their total number of points is within capacity,
    /// the children are merged into the parent node.
    fn try_merge(&mut self) {
        if !self.divided {
            return;
        }
        for child in self.children_mut() {
            child.try_merge();
        }
        let children = self.children();
        if children.iter().all(|child| !child.divided) {
            let total_points: usize = children.iter().map(|child| child.points.len()).sum();
            if total_points <= self.capacity {
                let mut merged_points = Vec::with_capacity(total_points);
                if let Some(child) = self.front_top_left.take() {
                    merged_points.extend(child.points);
                }
                if let Some(child) = self.front_top_right.take() {
                    merged_points.extend(child.points);
                }
                if let Some(child) = self.front_bottom_left.take() {
                    merged_points.extend(child.points);
                }
                if let Some(child) = self.front_bottom_right.take() {
                    merged_points.extend(child.points);
                }
                if let Some(child) = self.back_top_left.take() {
                    merged_points.extend(child.points);
                }
                if let Some(child) = self.back_top_right.take() {
                    merged_points.extend(child.points);
                }
                if let Some(child) = self.back_bottom_left.take() {
                    merged_points.extend(child.points);
                }
                if let Some(child) = self.back_bottom_right.take() {
                    merged_points.extend(child.points);
                }
                info!(
                    "Merging children into parent node at boundary {:?} with {} points",
                    self.boundary,
                    merged_points.len()
                );
                self.points = merged_points;
                self.divided = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::EuclideanDistance;

    #[test]
    fn test_insert_rejects_outside_boundary() {
        let boundary = Cube {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            width: 10.0,
            height: 10.0,
            depth: 10.0,
        };
        let mut tree: Octree<&str> = Octree::new(&boundary, 2).unwrap();
        let outside = Point3D::new(20.0, 20.0, 20.0, Some("O"));
        assert!(!tree.insert(outside));
    }

    #[test]
    fn test_insert_accepts_boundary_points() {
        let boundary = Cube {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            width: 10.0,
            height: 10.0,
            depth: 10.0,
        };
        let mut tree: Octree<&str> = Octree::new(&boundary, 1).unwrap();
        let edge = Point3D::new(10.0, 10.0, 10.0, Some("E"));
        assert!(tree.insert(edge));
    }

    #[test]
    fn test_range_search_zero_radius_returns_exact_match() {
        let boundary = Cube {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            width: 100.0,
            height: 100.0,
            depth: 100.0,
        };
        let mut tree: Octree<&str> = Octree::new(&boundary, 2).unwrap();
        let target = Point3D::new(25.0, 25.0, 25.0, Some("T"));
        tree.insert(target.clone());
        tree.insert(Point3D::new(25.0, 25.0, 26.0, Some("N")));

        let results = tree.range_search::<EuclideanDistance>(&target, 0.0);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], target);
    }

    #[test]
    fn test_delete_existing_point() {
        let boundary = Cube {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            width: 100.0,
            height: 100.0,
            depth: 100.0,
        };
        let mut tree: Octree<&str> = Octree::new(&boundary, 2).unwrap();
        let p1 = Point3D::new(10.0, 10.0, 10.0, Some("A"));
        let p2 = Point3D::new(20.0, 20.0, 20.0, Some("B"));
        tree.insert(p1.clone());
        tree.insert(p2);

        assert!(tree.delete(&p1));
        let results = tree.knn_search::<EuclideanDistance>(&p1, 1);
        assert_ne!(results[0], p1);
        assert!(!tree.delete(&p1));
    }

    #[test]
    fn test_empty_tree_queries() {
        let boundary = Cube {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            width: 100.0,
            height: 100.0,
            depth: 100.0,
        };
        let mut tree: Octree<&str> = Octree::new(&boundary, 2).unwrap();
        let target = Point3D::new(5.0, 5.0, 5.0, None::<&str>);

        let knn_results = tree.knn_search::<EuclideanDistance>(&target, 5);
        assert!(knn_results.is_empty());

        let range_results = tree.range_search::<EuclideanDistance>(&target, 10.0);
        assert!(range_results.is_empty());

        assert!(!tree.delete(&target));
    }

    #[test]
    fn test_knn_edge_cases() {
        let boundary = Cube {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            width: 100.0,
            height: 100.0,
            depth: 100.0,
        };
        let mut tree: Octree<&str> = Octree::new(&boundary, 4).unwrap();
        let points = vec![
            Point3D::new(10.0, 10.0, 10.0, Some("A")),
            Point3D::new(20.0, 20.0, 20.0, Some("B")),
            Point3D::new(30.0, 30.0, 30.0, Some("C")),
        ];
        let num_points = points.len();
        tree.insert_bulk(&points);

        let target = Point3D::new(15.0, 15.0, 15.0, None::<&str>);
        let knn_results = tree.knn_search::<EuclideanDistance>(&target, 0);
        assert!(knn_results.is_empty());

        let knn_results = tree.knn_search::<EuclideanDistance>(&target, num_points + 5);
        assert_eq!(knn_results.len(), num_points);
    }

    #[test]
    fn test_duplicates_delete_one() {
        let boundary = Cube {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            width: 100.0,
            height: 100.0,
            depth: 100.0,
        };
        let mut tree: Octree<&str> = Octree::new(&boundary, 4).unwrap();
        let p1 = Point3D::new(10.0, 10.0, 10.0, Some("A"));
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
    fn test_range_search_includes_boundary_point() {
        let boundary = Cube {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            width: 100.0,
            height: 100.0,
            depth: 100.0,
        };
        let mut tree: Octree<&str> = Octree::new(&boundary, 4).unwrap();
        let center = Point3D::new(50.0, 50.0, 50.0, Some("C"));
        let boundary_point = Point3D::new(60.0, 50.0, 50.0, Some("B"));
        tree.insert(center.clone());
        tree.insert(boundary_point.clone());

        let results = tree.range_search::<EuclideanDistance>(&center, 10.0);
        assert!(results.contains(&boundary_point));
        assert!(results.contains(&center));
    }

    #[test]
    fn test_bulk_insert_empty_noop() {
        let boundary = Cube {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            width: 100.0,
            height: 100.0,
            depth: 100.0,
        };
        let mut tree: Octree<i32> = Octree::new(&boundary, 4).unwrap();
        let empty: Vec<Point3D<i32>> = Vec::new();
        tree.insert_bulk(&empty);
        let target = Point3D::new(10.0, 10.0, 10.0, None::<i32>);
        let results = tree.knn_search::<EuclideanDistance>(&target, 1);
        assert!(results.is_empty());
    }

    #[test]
    fn test_zero_capacity_rejected() {
        let boundary = Cube {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            width: 100.0,
            height: 100.0,
            depth: 100.0,
        };
        let result = Octree::<i32>::new(&boundary, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_range_search_negative_radius_empty() {
        let boundary = Cube {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            width: 100.0,
            height: 100.0,
            depth: 100.0,
        };
        let mut tree: Octree<&str> = Octree::new(&boundary, 2).unwrap();
        let target = Point3D::new(10.0, 10.0, 10.0, Some("T"));
        tree.insert(target.clone());

        let results = tree.range_search::<EuclideanDistance>(&target, -1.0);
        assert!(results.is_empty());
    }
}
