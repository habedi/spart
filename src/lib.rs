//! # Spart
//!
//! A collection of space partitioning tree data structures for indexing points in 2D and 3D.
//!
//! ## API
//!
//! All five trees implement [`index::SpatialIndex`], so they share a common API for inserting,
//! removing, and searching points. The trees differ in their partitioning strategy,
//! which affects their performance and memory usage.
//!
//! | Tree | Dimensions | Bounded |
//! |---|---|---|
//! | [`quadtree::Quadtree`] | 2D | yes |
//! | [`octree::Octree`] | 3D | yes |
//! | [`kdtree::KdTree`] | 2D and 3D | no |
//! | [`rtree::RTree`] | 2D and 3D | no |
//! | [`rstar_tree::RStarTree`] | 2D and 3D | no |
//!
//! ## Example
//!
//! ```
//! use spart::geometry::{EuclideanDistance, Point2D, Rectangle};
//! use spart::quadtree::Quadtree;
//!
//! let boundary = Rectangle { x: 0.0, y: 0.0, width: 100.0, height: 100.0 };
//! let mut tree: Quadtree<&str> = Quadtree::new(&boundary, 4).unwrap();
//! tree.insert(Point2D::new(10.0, 20.0, Some("a")));
//! tree.insert(Point2D::new(80.0, 30.0, Some("b")));
//!
//! let nearest = tree.knn_search::<EuclideanDistance>(&Point2D::new(12.0, 22.0, None), 1);
//! assert_eq!(nearest[0].data, Some("a"));
//! ```

pub mod errors;
pub mod geometry;
pub mod index;
pub mod kdtree;
mod knn;
pub mod octree;
pub mod quadtree;
pub mod rstar_tree;
pub mod rtree;
mod rtree_common;
