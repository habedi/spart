//! # Spart
//!
//! A collection of space partitioning tree data structures for indexing points in 2D and 3D.
//!
//! Five trees are provided, and all of them implement [`index::SpatialIndex`], so they answer the
//! same set of operations and can be swapped for one another:
//!
//! | Tree | Dimensions | Bounded | Notes |
//! |---|---|---|---|
//! | [`quadtree::Quadtree`] | 2D | yes | subdivides a rectangle once a node exceeds its capacity |
//! | [`octree::Octree`] | 3D | yes | the 3D counterpart, over a cube |
//! | [`kdtree::KdTree`] | 2D and 3D | no | binary splits on a rotating axis, kept balanced by partial rebuilding |
//! | [`rtree::RTree`] | 2D and 3D | no | bounding-volume tree with Guttman quadratic split |
//! | [`rstar_tree::RStarTree`] | 2D and 3D | no | R*-tree refinements: forced reinsertion and margin-based split |
//!
//! ### Example
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
//!
//! ### Features
//!
//! * `serde`: derives `Serialize` and `Deserialize` for every tree and geometric type.
//! * `enable_log`: routes this crate's `tracing` output through the `log` facade.
//!
//! ### Logging
//!
//! Spart emits [`tracing`](https://docs.rs/tracing) events but never installs a subscriber. Choosing
//! one is the application's decision, so whatever subscriber it installs will pick Spart's events up
//! with no feature flag and no setup call.

#![deny(missing_docs)]
// Nothing in this crate needs `unsafe`. The one exception used to be the `ctor` constructor that
// installed a tracing subscriber before `main`; that is gone.
#![forbid(unsafe_code)]

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
