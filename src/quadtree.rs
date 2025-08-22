//! ## Quadtree Implementation
//!
//! This module implements a quadtree for indexing of 2D points. The quadtree partitions a
//! rectangular region (defined by a `Rectangle`) into four quadrants (northeast, northwest, southeast,
//! and southwest) when the number of points in a region exceeds a specified capacity. It provides
//! operations for insertion, k-nearest neighbor (kNN) search, range search, and deletion.
//!
//! ### Example
//!
//! ```
//! use spart::geometry::{EuclideanDistance, Point2D, Rectangle};
//! use spart::quadtree::Quadtree;
//!
//! // Define a boundary for the quadtree.
//! let boundary = Rectangle { x: 0.0, y: 0.0, width: 100.0, height: 100.0 };
//! // Create a quadtree with capacity 4.
//! let mut qt = Quadtree::new(&boundary, 4);
//!
//! // Insert some points.
//! let pt1: Point2D<()> = Point2D::new(10.0, 20.0, None);
//! let pt2: Point2D<()> = Point2D::new(50.0, 50.0, None);
//! qt.insert(pt1);
//! qt.insert(pt2);
//!
//! // Perform a k-nearest neighbor search.
//! let neighbors = qt.knn_search::<EuclideanDistance>(&Point2D::new(12.0, 22.0, None), 1);
//! assert!(!neighbors.is_empty());
//! ```

use crate::exceptions::SpartError;
use crate::geometry::{DistanceMetric, HeapItem, Point2D, Rectangle};
use ordered_float::OrderedFloat;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use std::collections::BinaryHeap;
use tracing::{debug, info};

/// A Quadtree for indexing of 2D points.
///
/// # Type Parameters
///
/// * `T`: The type of additional data stored in each point.
///
/// # Panics
///
/// Panics with `SpartError::InvalidCapacity` if `capacity` is zero.
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Quadtree<T: Clone + PartialEq> {
    boundary: Rectangle,
    points: Vec<Point2D<T>>,
    capacity: usize,
    divided: bool,
    northeast: Option<Box<Quadtree<T>>>,
    northwest: Option<Box<Quadtree<T>>>,
    southeast: Option<Box<Quadtree<T>>>,
    southwest: Option<Box<Quadtree<T>>>,
}

impl<T: Clone + PartialEq + std::fmt::Debug> Quadtree<T> {
    /// Creates a new `Quadtree` with the specified boundary and capacity.
    ///
    /// # Arguments
    ///
    /// * `boundary` - The rectangular region covered by this quadtree.
    /// * `capacity` - The maximum number of points a node can hold before subdividing.
    ///
    /// # Panics
    ///
    /// Panics with `SpartError::InvalidCapacity` if `capacity` is zero.
    pub fn new(boundary: &Rectangle, capacity: usize) -> Self {
        if capacity == 0 {
            panic!("{}", SpartError::InvalidCapacity { capacity });
        }
        info!(
            "Creating new Quadtree with boundary: {:?} and capacity: {}",
            boundary, capacity
        );
        Quadtree {
            boundary: boundary.clone(),
            points: Vec::new(),
            capacity,
            divided: false,
            northeast: None,
            northwest: None,
            southeast: None,
            southwest: None,
        }
    }

    /// Subdivides the current quadtree node into four child quadrants.
    ///
    /// After subdivision, all existing points are reinserted into the appropriate children.
    fn subdivide(&mut self) {
        info!("Subdividing Quadtree at boundary: {:?}", self.boundary);
        let x = self.boundary.x;
        let y = self.boundary.y;
        let w = self.boundary.width / 2.0;
        let h = self.boundary.height / 2.0;
        self.northeast = Some(Box::new(Quadtree::new(
            &Rectangle {
                x: x + w,
                y,
                width: w,
                height: h,
            },
            self.capacity,
        )));
        self.northwest = Some(Box::new(Quadtree::new(
            &Rectangle {
                x,
                y,
                width: w,
                height: h,
            },
            self.capacity,
        )));
        self.southeast = Some(Box::new(Quadtree::new(
            &Rectangle {
                x: x + w,
                y: y + h,
                width: w,
                height: h,
            },
            self.capacity,
        )));
        self.southwest = Some(Box::new(Quadtree::new(
            &Rectangle {
                x,
                y: y + h,
                width: w,
                height: h,
            },
            self.capacity,
        )));
        self.divided = true;
        // Reinsert existing points into the appropriate children.
        let old_points = std::mem::take(&mut self.points);
        for point in old_points {
            let inserted = self.insert(point);
            if !inserted {
                debug!("Failed to reinsert point during subdivision");
            }
        }
    }

    /// Inserts a point into the quadtree.
    ///
    /// If the point is not within the boundary, it is ignored.
    /// If the current node is full, the node subdivides and attempts to insert the point into a child.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to insert.
    ///
    /// # Returns
    ///
    /// `true` if the point was successfully inserted, `false` otherwise.
    pub fn insert(&mut self, point: Point2D<T>) -> bool {
        if !self.boundary.contains(&point) {
            debug!("Point {:?} is out of bounds of {:?}", point, self.boundary);
            return false;
        }
        if self.divided {
            let children = self.children_mut();
            let num_children = children.len();
            for (i, child) in children.into_iter().enumerate() {
                // Insert into each child until one accepts the point.
                if i < num_children - 1 {
                    if child.insert(point.clone()) {
                        return true;
                    }
                } else {
                    return child.insert(point);
                }
            }
            return false;
        }
        if self.points.len() < self.capacity {
            info!("Inserting point {:?} into Quadtree", point);
            self.points.push(point);
            return true;
        }
        self.subdivide();
        self.insert(point)
    }

    /// Inserts a bulk of points into the quadtree.
    ///
    /// # Arguments
    ///
    /// * `points` - The points to insert.
    pub fn insert_bulk(&mut self, points: &[Point2D<T>]) {
        if points.is_empty() {
            return;
        }

        // Filter out points that are not within the boundary
        let points_within_boundary: Vec<Point2D<T>> = points
            .iter()
            .filter(|p| self.boundary.contains(p))
            .cloned()
            .collect();

        if points_within_boundary.is_empty() {
            return;
        }

        // If the current node is not divided and has enough capacity, add the points
        if !self.divided && self.points.len() + points_within_boundary.len() <= self.capacity {
            self.points.extend(points_within_boundary);
            return;
        }

        // If the current node is not divided but adding the new points would exceed the capacity,
        // subdivide the node and distribute the existing and new points among the children.
        if !self.divided {
            self.subdivide();
        }

        // If the node is already divided, distribute the new points among the children.
        let mut points_to_insert = points_within_boundary;
        if self.divided {
            let mut children_points: [Vec<Point2D<T>>; 4] = [vec![], vec![], vec![], vec![]];

            for point in points_to_insert.drain(..) {
                if self.northeast.as_ref().unwrap().boundary.contains(&point) {
                    children_points[0].push(point);
                } else if self.northwest.as_ref().unwrap().boundary.contains(&point) {
                    children_points[1].push(point);
                } else if self.southeast.as_ref().unwrap().boundary.contains(&point) {
                    children_points[2].push(point);
                } else if self.southwest.as_ref().unwrap().boundary.contains(&point) {
                    children_points[3].push(point);
                }
            }

            if !children_points[0].is_empty() {
                self.northeast
                    .as_mut()
                    .unwrap()
                    .insert_bulk(&children_points[0]);
            }
            if !children_points[1].is_empty() {
                self.northwest
                    .as_mut()
                    .unwrap()
                    .insert_bulk(&children_points[1]);
            }
            if !children_points[2].is_empty() {
                self.southeast
                    .as_mut()
                    .unwrap()
                    .insert_bulk(&children_points[2]);
            }
            if !children_points[3].is_empty() {
                self.southwest
                    .as_mut()
                    .unwrap()
                    .insert_bulk(&children_points[3]);
            }
        }
    }

    /// Returns mutable references to the four child quadrants, if they exist.
    fn children_mut(&mut self) -> Vec<&mut Quadtree<T>> {
        let mut children = Vec::with_capacity(4);
        if let Some(ref mut child) = self.northeast {
            children.push(child.as_mut());
        }
        if let Some(ref mut child) = self.northwest {
            children.push(child.as_mut());
        }
        if let Some(ref mut child) = self.southeast {
            children.push(child.as_mut());
        }
        if let Some(ref mut child) = self.southwest {
            children.push(child.as_mut());
        }
        children
    }

    /// Returns references to the four child quadrants, if they exist.
    fn children(&self) -> Vec<&Quadtree<T>> {
        let mut children = Vec::with_capacity(4);
        if let Some(ref child) = self.northeast {
            children.push(child.as_ref());
        }
        if let Some(ref child) = self.northwest {
            children.push(child.as_ref());
        }
        if let Some(ref child) = self.southeast {
            children.push(child.as_ref());
        }
        if let Some(ref child) = self.southwest {
            children.push(child.as_ref());
        }
        children
    }

    /// Computes the squared minimum distance from the given target point to the boundary of this node.
    ///
    /// This is used to decide if a subtree can be skipped during k-nearest neighbor search.
    ///
    /// # Arguments
    ///
    /// * `target` - The target point.
    fn min_distance_sq(&self, target: &Point2D<T>) -> f64 {
        let mut dx = 0.0;
        if target.x < self.boundary.x {
            dx = self.boundary.x - target.x;
        } else if target.x > self.boundary.x + self.boundary.width {
            dx = target.x - (self.boundary.x + self.boundary.width);
        }
        let mut dy = 0.0;
        if target.y < self.boundary.y {
            dy = self.boundary.y - target.y;
        } else if target.y > self.boundary.y + self.boundary.height {
            dy = target.y - (self.boundary.y + self.boundary.height);
        }
        dx * dx + dy * dy
    }

    /// Performs a k-nearest neighbor search for the target point.
    ///
    /// # Arguments
    ///
    /// * `target` - The point for which to find the k nearest neighbors.
    /// * `k` - The number of nearest neighbors to retrieve.
    ///
    /// # Returns
    ///
    /// A vector of the k nearest points, ordered from nearest to farthest.
    ///
    /// # Note
    ///
    /// The pruning logic for the search is based on Euclidean distance. Custom distance metrics
    /// that are not compatible with Euclidean distance may lead to incorrect results or reduced
    /// performance.
    pub fn knn_search<M: DistanceMetric<Point2D<T>>>(
        &self,
        target: &Point2D<T>,
        k: usize,
    ) -> Vec<Point2D<T>> {
        let mut heap: BinaryHeap<HeapItem<T>> = BinaryHeap::new();
        self.knn_search_helper::<M>(target, k, &mut heap);
        heap.into_sorted_vec()
            .into_iter()
            .filter_map(|item| item.point_2d)
            .collect()
    }

    /// Helper method for performing the recursive k-nearest neighbor search.
    fn knn_search_helper<M: DistanceMetric<Point2D<T>>>(
        &self,
        target: &Point2D<T>,
        k: usize,
        heap: &mut BinaryHeap<HeapItem<T>>,
    ) {
        for point in &self.points {
            let dist_sq = M::distance_sq(point, target);
            let item = HeapItem {
                neg_distance: OrderedFloat(-dist_sq),
                point_2d: Some(point.clone()),
                point_3d: None,
            };
            heap.push(item);
            if heap.len() > k {
                heap.pop();
            }
        }
        if self.divided {
            for child in self.children() {
                if heap.len() == k {
                    let current_farthest = -heap.peek().unwrap().neg_distance.into_inner();
                    if child.min_distance_sq(target) > current_farthest {
                        continue;
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
    /// A vector of points within the range.
    ///
    /// # Note
    ///
    /// The pruning logic for the search is based on Euclidean distance. Custom distance metrics
    /// that are not compatible with Euclidean distance may lead to incorrect results or reduced
    /// performance.
    pub fn range_search<M: DistanceMetric<Point2D<T>>>(
        &self,
        center: &Point2D<T>,
        radius: f64,
    ) -> Vec<Point2D<T>> {
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

    /// Deletes a point from the quadtree.
    ///
    /// Returns `true` if the point was found and deleted.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to delete.
    pub fn delete(&mut self, point: &Point2D<T>) -> bool {
        if !self.boundary.contains(point) {
            return false;
        }
        let mut deleted = false;
        if self.divided {
            for child in self.children_mut() {
                if child.delete(point) {
                    deleted = true;
                }
            }
            self.try_merge();
            return deleted;
        }
        if let Some(pos) = self.points.iter().position(|p| p == point) {
            info!("Deleting point {:?} from Quadtree", point);
            self.points.remove(pos);
            return true;
        }
        false
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
                if let Some(child) = self.northeast.take() {
                    merged_points.extend(child.points);
                }
                if let Some(child) = self.northwest.take() {
                    merged_points.extend(child.points);
                }
                if let Some(child) = self.southeast.take() {
                    merged_points.extend(child.points);
                }
                if let Some(child) = self.southwest.take() {
                    merged_points.extend(child.points);
                }
                info!(
                    "Merging children into parent node at boundary {:?} with {} points",
                    self.boundary,
                    merged_points.len()
                );
                self.points.extend(merged_points);
                self.divided = false;
            }
        }
    }
}
