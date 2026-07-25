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
//! let mut qt = Quadtree::new(&boundary, 4).unwrap();
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

use crate::errors::SpartError;
use crate::geometry::{DistanceMetric, HasMinDistance, Point2D, Rectangle, span};
use crate::knn::KnnHeap;
#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use tracing::info;

/// Deepest level a node will subdivide to.
///
/// Without a cap, points that coincide, or very nearly do, subdivide forever: they always land in
/// the same child, so no split ever separates them. Beyond this depth a node simply keeps its
/// points, growing past `capacity` rather than recursing. A node at depth 32 covers 2^-32 of the
/// original boundary, so only genuinely (near-)coincident points ever reach it.
const MAX_DEPTH: usize = 32;

/// A Quadtree for indexing of 2D points.
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
pub struct Quadtree<T: Clone + PartialEq> {
    boundary: Rectangle,
    points: Vec<Point2D<T>>,
    capacity: usize,
    divided: bool,
    /// Distance from the root, used to stop subdividing at [`MAX_DEPTH`].
    depth: usize,
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
    /// # Errors
    ///
    /// Returns `SpartError::InvalidCapacity` if `capacity` is zero.
    pub fn new(boundary: &Rectangle, capacity: usize) -> Result<Self, SpartError> {
        if capacity == 0 {
            return Err(SpartError::InvalidCapacity { capacity });
        }
        info!(
            "Creating new Quadtree with boundary: {:?} and capacity: {}",
            boundary, capacity
        );
        Ok(Self::with_depth(boundary.clone(), capacity, 0))
    }

    /// Creates an empty node covering `boundary` at the given depth below the root.
    fn with_depth(boundary: Rectangle, capacity: usize, depth: usize) -> Self {
        Quadtree {
            boundary,
            points: Vec::new(),
            capacity,
            divided: false,
            depth,
            northeast: None,
            northwest: None,
            southeast: None,
            southwest: None,
        }
    }

    /// Subdivides the current quadtree node into four child quadrants.
    ///
    /// The children's far edges are derived from the parent's own edges rather than from halved
    /// extents, so the four of them tile the parent exactly. A gap of even a single ULP would let a
    /// point be stored in a child whose boundary does not contain it, and search pruning would then
    /// skip right over it.
    ///
    /// After subdivision, all existing points are moved down into the appropriate children.
    fn subdivide(&mut self) {
        info!("Subdividing Quadtree at boundary: {:?}", self.boundary);
        let Rectangle {
            x,
            y,
            width,
            height,
        } = self.boundary;
        let mid_x = x + width / 2.0;
        let mid_y = y + height / 2.0;
        let west = span(x, mid_x);
        let east = span(mid_x, x + width);
        let north = span(y, mid_y);
        let south = span(mid_y, y + height);

        let quadrant = |qx: f64, qy: f64, qw: f64, qh: f64| {
            Some(Box::new(Self::with_depth(
                Rectangle {
                    x: qx,
                    y: qy,
                    width: qw,
                    height: qh,
                },
                self.capacity,
                self.depth + 1,
            )))
        };
        self.northwest = quadrant(x, y, west, north);
        self.northeast = quadrant(mid_x, y, east, north);
        self.southwest = quadrant(x, mid_y, west, south);
        self.southeast = quadrant(mid_x, mid_y, east, south);
        self.divided = true;

        // Move existing points down into the children.
        for point in std::mem::take(&mut self.points) {
            self.insert_within(point);
        }
    }

    /// Index of the quadrant a point belongs to: bit 0 selects east over west, bit 1 south over
    /// north. Matches the order of [`Quadtree::children`].
    fn child_index(&self, point: &Point2D<T>) -> usize {
        let east = point.x >= self.boundary.x + self.boundary.width / 2.0;
        let south = point.y >= self.boundary.y + self.boundary.height / 2.0;
        usize::from(east) | (usize::from(south) << 1)
    }

    /// Inserts a point already known to lie inside this node's boundary.
    ///
    /// Routing compares against the node's midpoints instead of testing each child's boundary, so
    /// exactly one child is always selected. Testing boundaries meant that once floating-point
    /// rounding crept in a point could match the parent and yet match no child at all. That is how
    /// coincident points used to reach `unreachable!()` on insert and vanish silently on bulk insert.
    fn insert_within(&mut self, point: Point2D<T>) {
        if !self.divided {
            // At the depth cap the node becomes a bucket: (near-)coincident points can never be
            // separated by another split, so subdividing further would recurse without end.
            if self.points.len() < self.capacity || self.depth >= MAX_DEPTH {
                self.points.push(point);
                return;
            }
            self.subdivide();
        }

        let index = self.child_index(&point);
        if self.child_mut(index).is_none() {
            // A subdivided node always has all four children. If one is somehow missing, keep the
            // point here rather than dropping it.
            self.points.push(point);
            return;
        }
        if let Some(child) = self.child_mut(index) {
            child.insert_within(point);
        }
    }

    /// Inserts a point into the quadtree.
    ///
    /// If the point is not within the boundary, it is ignored.
    /// If the current node is full, the node subdivides and inserts the point into the child whose
    /// quadrant contains it.
    ///
    /// # Arguments
    ///
    /// * `point` - The point to insert.
    ///
    /// # Returns
    ///
    /// `true` if the point was successfully inserted, `false` if it lies outside the boundary.
    pub fn insert(&mut self, point: Point2D<T>) -> bool {
        if !self.boundary.contains(&point) {
            return false;
        }
        self.insert_within(point);
        true
    }

    /// Inserts many points into the quadtree at once.
    ///
    /// Points outside the boundary are skipped rather than reported individually, so the return
    /// value says how many of them were actually stored.
    ///
    /// # Arguments
    ///
    /// * `points` - The points to insert.
    ///
    /// # Returns
    ///
    /// The number of points that were inserted.
    pub fn insert_bulk(&mut self, points: &[Point2D<T>]) -> usize {
        let mut inserted = 0;
        for point in points {
            if self.boundary.contains(point) {
                self.insert_within(point.clone());
                inserted += 1;
            }
        }
        inserted
    }

    /// The four child quadrants in [`Quadtree::child_index`] order, or `None` where absent.
    fn children(&self) -> [Option<&Quadtree<T>>; 4] {
        [
            self.northwest.as_deref(),
            self.northeast.as_deref(),
            self.southwest.as_deref(),
            self.southeast.as_deref(),
        ]
    }

    /// The child at `index` in [`Quadtree::child_index`] order.
    fn child_mut(&mut self, index: usize) -> Option<&mut Quadtree<T>> {
        match index {
            0 => self.northwest.as_deref_mut(),
            1 => self.northeast.as_deref_mut(),
            2 => self.southwest.as_deref_mut(),
            _ => self.southeast.as_deref_mut(),
        }
    }

    /// Computes the squared minimum distance from the given target point to the boundary of this node.
    ///
    /// This is used to decide if a subtree can be skipped during k-nearest neighbor search.
    ///
    /// # Arguments
    ///
    /// * `target` - The target point.
    fn min_distance_sq(&self, target: &Point2D<T>) -> f64 {
        HasMinDistance::min_distance_sq(&self.boundary, target)
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
    ) -> Vec<&Point2D<T>> {
        let mut heap = KnnHeap::new(k);
        self.knn_search_helper::<M>(target, &mut heap);
        heap.into_sorted_vec()
    }

    /// Helper method for performing the recursive k-nearest neighbor search.
    fn knn_search_helper<'a, M: DistanceMetric<Point2D<T>>>(
        &'a self,
        target: &Point2D<T>,
        heap: &mut KnnHeap<&'a Point2D<T>>,
    ) {
        for point in &self.points {
            heap.offer(M::distance_sq(point, target), point);
        }
        for child in self.children().into_iter().flatten() {
            // `worst` is infinite until the heap is full, so this prunes only once there is a
            // distance worth beating.
            if child.min_distance_sq(target) > heap.worst() {
                continue;
            }
            child.knn_search_helper::<M>(target, heap);
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
        for child in self.children().into_iter().flatten() {
            found.extend(child.range_search::<M>(center, radius));
        }
        found
    }

    /// Performs a range search over a query rectangle, returning every point inside it.
    ///
    /// # Arguments
    ///
    /// * `query` - The rectangle to search.
    pub fn range_search_bbox(&self, query: &Rectangle) -> Vec<Point2D<T>> {
        let mut found = Vec::new();
        self.collect_in_bbox(query, &mut found);
        found
    }

    /// Helper collecting the points inside `query` into `found`.
    fn collect_in_bbox(&self, query: &Rectangle, found: &mut Vec<Point2D<T>>) {
        if !self.boundary.intersects(query) {
            return;
        }
        for point in &self.points {
            if query.contains(point) {
                found.push(point.clone());
            }
        }
        for child in self.children().into_iter().flatten() {
            child.collect_in_bbox(query, found);
        }
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
        if let Some(pos) = self.points.iter().position(|p| p == point) {
            self.points.remove(pos);
            info!("Deleting point {:?} from Quadtree", point);
            return true;
        }
        if !self.divided {
            return false;
        }
        // Insertion routes a point to exactly one quadrant, so only that quadrant can hold it;
        // there is no need to search the other three.
        let index = self.child_index(point);
        let deleted = self
            .child_mut(index)
            .is_some_and(|child| child.delete(point));
        if deleted {
            self.try_merge();
        }
        deleted
    }

    /// Attempts to merge child nodes back into the parent node if possible.
    ///
    /// Only this node is considered. `delete` calls this at every level on its way back up, so the
    /// path that actually changed is already merged bottom-up; recursing over the whole subtree here
    /// made every delete cost time proportional to the size of the tree.
    fn try_merge(&mut self) {
        if !self.divided {
            return;
        }
        let children = self.children();
        if children.iter().flatten().all(|child| !child.divided) {
            let total_points: usize = children.iter().flatten().map(|c| c.points.len()).sum();
            if total_points + self.points.len() <= self.capacity {
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::EuclideanDistance;

    #[test]
    fn test_insert_rejects_outside_boundary() {
        let boundary = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
        };
        let mut tree: Quadtree<&str> = Quadtree::new(&boundary, 2).unwrap();
        let outside = Point2D::new(20.0, 20.0, Some("O"));
        assert!(!tree.insert(outside));
    }

    #[test]
    fn test_insert_accepts_boundary_points() {
        let boundary = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 10.0,
            height: 10.0,
        };
        let mut tree: Quadtree<&str> = Quadtree::new(&boundary, 1).unwrap();
        let edge = Point2D::new(10.0, 10.0, Some("E"));
        assert!(tree.insert(edge));
    }

    #[test]
    fn test_range_search_zero_radius_returns_exact_match() {
        let boundary = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        };
        let mut tree: Quadtree<&str> = Quadtree::new(&boundary, 2).unwrap();
        let target = Point2D::new(25.0, 25.0, Some("T"));
        tree.insert(target.clone());
        tree.insert(Point2D::new(26.0, 25.0, Some("N")));

        let results = tree.range_search::<EuclideanDistance>(&target, 0.0);
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], target);
    }

    #[test]
    fn test_delete_existing_point() {
        let boundary = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        };
        let mut tree: Quadtree<&str> = Quadtree::new(&boundary, 2).unwrap();
        let p1 = Point2D::new(10.0, 10.0, Some("A"));
        let p2 = Point2D::new(20.0, 20.0, Some("B"));
        tree.insert(p1.clone());
        tree.insert(p2);

        assert!(tree.delete(&p1));
        let results = tree.knn_search::<EuclideanDistance>(&p1, 1);
        assert_ne!(results[0], p1);
        assert!(!tree.delete(&p1));
    }

    #[test]
    fn test_empty_tree_queries() {
        let boundary = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        };
        let mut tree: Quadtree<&str> = Quadtree::new(&boundary, 2).unwrap();
        let target = Point2D::new(5.0, 5.0, None::<&str>);

        let knn_results = tree.knn_search::<EuclideanDistance>(&target, 5);
        assert!(knn_results.is_empty());

        let range_results = tree.range_search::<EuclideanDistance>(&target, 10.0);
        assert!(range_results.is_empty());

        assert!(!tree.delete(&target));
    }

    #[test]
    fn test_knn_edge_cases() {
        let boundary = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        };
        let mut tree: Quadtree<&str> = Quadtree::new(&boundary, 4).unwrap();
        let points = vec![
            Point2D::new(10.0, 10.0, Some("A")),
            Point2D::new(20.0, 20.0, Some("B")),
            Point2D::new(30.0, 30.0, Some("C")),
        ];
        let num_points = points.len();
        tree.insert_bulk(&points);

        let target = Point2D::new(15.0, 15.0, None::<&str>);
        let knn_results = tree.knn_search::<EuclideanDistance>(&target, 0);
        assert!(knn_results.is_empty());

        let knn_results = tree.knn_search::<EuclideanDistance>(&target, num_points + 5);
        assert_eq!(knn_results.len(), num_points);
    }

    #[test]
    fn test_duplicates_delete_one() {
        let boundary = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        };
        let mut tree: Quadtree<&str> = Quadtree::new(&boundary, 4).unwrap();
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
    fn test_range_search_includes_boundary_point() {
        let boundary = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        };
        let mut tree: Quadtree<&str> = Quadtree::new(&boundary, 4).unwrap();
        let center = Point2D::new(50.0, 50.0, Some("C"));
        let boundary_point = Point2D::new(60.0, 50.0, Some("B"));
        tree.insert(center.clone());
        tree.insert(boundary_point.clone());

        let results = tree.range_search::<EuclideanDistance>(&center, 10.0);
        assert!(results.contains(&boundary_point));
        assert!(results.contains(&center));
    }

    #[test]
    fn test_bulk_insert_empty_noop() {
        let boundary = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        };
        let mut tree: Quadtree<i32> = Quadtree::new(&boundary, 4).unwrap();
        let empty: Vec<Point2D<i32>> = Vec::new();
        tree.insert_bulk(&empty);
        let target = Point2D::new(10.0, 10.0, None::<i32>);
        let results = tree.knn_search::<EuclideanDistance>(&target, 1);
        assert!(results.is_empty());
    }

    #[test]
    fn test_zero_capacity_rejected() {
        let boundary = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        };
        let result = Quadtree::<i32>::new(&boundary, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_range_search_negative_radius_empty() {
        let boundary = Rectangle {
            x: 0.0,
            y: 0.0,
            width: 100.0,
            height: 100.0,
        };
        let mut tree: Quadtree<&str> = Quadtree::new(&boundary, 2).unwrap();
        let target = Point2D::new(10.0, 10.0, Some("T"));
        tree.insert(target.clone());

        let results = tree.range_search::<EuclideanDistance>(&target, -1.0);
        assert!(results.is_empty());
    }
}
