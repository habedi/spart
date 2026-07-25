//! ## The Common Spatial Index Interface
//!
//! Every tree in this crate answers the same questions: store an item, drop an item, count what is
//! stored, find the nearest items to a query, and find the items inside a region. [`SpatialIndex`]
//! states those operations once so that code can be written against "a spatial index" rather than
//! against one specific tree, and so that one test harness can hold every tree to the same contract.
//!
//! ### Example
//!
//! ```
//! use spart::geometry::{EuclideanDistance, Point2D, Rectangle};
//! use spart::index::SpatialIndex;
//! use spart::kdtree::KdTree;
//! use spart::quadtree::Quadtree;
//!
//! // One function, any tree.
//! fn nearest_label<I>(index: &I, query: &Point2D<&'static str>) -> Option<&'static str>
//! where
//!     I: SpatialIndex<Item = Point2D<&'static str>>,
//! {
//!     index
//!         .knn_search::<EuclideanDistance>(query, 1)
//!         .first()
//!         .and_then(|point| point.data)
//! }
//!
//! let boundary = Rectangle { x: 0.0, y: 0.0, width: 100.0, height: 100.0 };
//! let mut quadtree: Quadtree<&'static str> = Quadtree::new(&boundary, 4).unwrap();
//! let mut kdtree: KdTree<Point2D<&'static str>> = KdTree::new();
//! for point in [Point2D::new(10.0, 10.0, Some("near")), Point2D::new(90.0, 90.0, Some("far"))] {
//!     // Called through the trait, so both trees answer with the same `Result<bool, _>`.
//!     assert!(SpatialIndex::insert(&mut quadtree, point.clone()).unwrap());
//!     assert!(SpatialIndex::insert(&mut kdtree, point).unwrap());
//! }
//!
//! let query = Point2D::new(12.0, 12.0, None);
//! assert_eq!(nearest_label(&quadtree, &query), Some("near"));
//! assert_eq!(nearest_label(&kdtree, &query), Some("near"));
//! ```

use crate::errors::SpartError;
use crate::geometry::DistanceMetric;

/// The operations every spatial index in this crate provides.
///
/// ### Contract
///
/// * `insert` returns `Ok(false)` for an item the index declines to store, which for a tree built
///   over a fixed boundary means an item outside it. It returns `Err` only when the item cannot be
///   interpreted at all, such as a point whose dimension does not match the tree's.
/// * `insert_bulk` reports how many items it stored. It stores every item it accepts and skips the
///   rest, so a count below `items.len()` means the remainder was declined, not that nothing was
///   stored. An `Err` is different: it means the batch was rejected before anything was stored, so
///   the index is unchanged and the whole batch can be retried.
/// * Queries borrow from the index rather than cloning, so a caller that wants owned items clones
///   only the ones it keeps.
/// * `knn_search` returns at most `k` items ordered nearest first. Items at equal distance are
///   ordered deterministically for a given tree and insertion history, but the order is the tree's
///   traversal order rather than the order they were inserted in, and it differs between trees. Do
///   not rely on which of several equidistant items comes back first.
/// * `range_search_bbox` returns exactly the items the volume contains, on every tree.
///
/// Pruning throughout assumes the metric agrees with Euclidean distance on which of two points is
/// nearer. A metric that does not may return fewer items than it should.
///
/// ### Inherent Methods Take Precedence
///
/// Each tree also has inherent methods of the same names, kept because they can be more precise
/// about what that particular tree does: `Quadtree::insert` returns a plain `bool` because a quadtree
/// cannot fail for any other reason, and `RTree::insert` returns nothing at all because it cannot
/// fail. Rust resolves `tree.insert(item)` to the inherent method, so reach for this trait through
/// `SpatialIndex::insert(&mut tree, item)` or from a generic function when you want the uniform
/// contract above.
pub trait SpatialIndex {
    /// The item the index stores.
    type Item;

    /// The volume used for box queries, such as `Rectangle` in 2D or `Cube` in 3D.
    type Volume;

    /// Number of items held.
    fn len(&self) -> usize;

    /// Whether the index holds nothing.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Removes every item, keeping the index's configuration.
    fn clear(&mut self);

    /// Whether an equal item is held.
    fn contains(&self, item: &Self::Item) -> bool;

    /// Stores one item, reporting whether it was accepted.
    ///
    /// # Errors
    ///
    /// Returns [`SpartError::DimensionMismatch`] when the item's dimension does not match the
    /// index's.
    fn insert(&mut self, item: Self::Item) -> Result<bool, SpartError>;

    /// Stores many items, returning how many were accepted.
    ///
    /// # Errors
    ///
    /// Returns [`SpartError::DimensionMismatch`] without storing anything when an item's dimension
    /// does not match the index's.
    fn insert_bulk(&mut self, items: Vec<Self::Item>) -> Result<usize, SpartError>;

    /// Removes one item equal to `item`, reporting whether one was found.
    ///
    /// With duplicates stored, exactly one copy is removed.
    fn delete(&mut self, item: &Self::Item) -> bool;

    /// The `k` items nearest to `query`, nearest first.
    fn knn_search<M: DistanceMetric<Self::Item>>(
        &self,
        query: &Self::Item,
        k: usize,
    ) -> Vec<&Self::Item>;

    /// Every item within `radius` of `query`. A negative radius matches nothing.
    fn range_search<M: DistanceMetric<Self::Item>>(
        &self,
        query: &Self::Item,
        radius: f64,
    ) -> Vec<&Self::Item>;

    /// Every item inside the query volume.
    fn range_search_bbox(&self, query: &Self::Volume) -> Vec<&Self::Item>;
}
