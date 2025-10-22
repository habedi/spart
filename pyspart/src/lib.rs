//! Python bindings for the spart spatial data structures library.
//!
//! This module provides Python bindings for various spatial indexing data structures
//! implemented in Rust, including Quadtrees, Octrees, K-d Trees, R-Trees, and R*-Trees.
//!
//! # Module Organization
//!
//! - `types` - PyData wrapper for bridging Python objects with Rust
//! - `geometry` - Geometric boundary extractors (PyRectangle, PyCube)
//! - `point2d` and `point3d` - Point type implementations
//! - `quadtree` - 2D space partitioning tree
//! - `octree` - 3D space partitioning tree
//! - `kdtree` - K-dimensional trees for nearest neighbor search
//! - `rtree` - R-tree spatial index
//! - `rstar_tree` - R*-tree with improved split heuristics
//!
//! # Key Design Notes
//!
//! ## Return Type Differences
//! Different tree structures return search results differently:
//! - Quadtree, Octree, KdTree: Return references (use `(&p).into()`)
//! - RTree, RStarTree: Return owned values (use `p.into()`)
//!
//! ## Data Handling
//! All points must have non-None data when converted from Python. Conversions will
//! panic with descriptive messages if None data is encountered.
//!
//! # Example
//!
//! ```python
//! from pyspart import Point2D, Quadtree
//!
//! # Create a quadtree
//! boundary = {"x": 0.0, "y": 0.0, "width": 100.0, "height": 100.0}
//! tree = Quadtree(boundary, capacity=4)
//!
//! # Insert points
//! tree.insert(Point2D(10.0, 20.0, {"id": 1}))
//! tree.insert(Point2D(30.0, 40.0, {"id": 2}))
//!
//! # Search
//! results = tree.knn_search(Point2D(15.0, 25.0, None), k=1)
//! ```

use pyo3::prelude::*;

mod geometry;
mod kdtree;
mod octree;
mod point2d;
mod point3d;
mod quadtree;
mod rstar_tree;
mod rtree;
mod types;

use kdtree::{PyKdTree2D, PyKdTree3D};
use octree::PyOctree;
use point2d::PyPoint2D;
use point3d::PyPoint3D;
use quadtree::PyQuadtree;
use rstar_tree::{PyRStarTree2D, PyRStarTree3D};
use rtree::{PyRTree2D, PyRTree3D};

#[pymodule]
fn pyspart(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyPoint2D>()?;
    m.add_class::<PyPoint3D>()?;
    m.add_class::<PyQuadtree>()?;
    m.add_class::<PyOctree>()?;
    m.add_class::<PyKdTree2D>()?;
    m.add_class::<PyKdTree3D>()?;
    m.add_class::<PyRTree2D>()?;
    m.add_class::<PyRTree3D>()?;
    m.add_class::<PyRStarTree2D>()?;
    m.add_class::<PyRStarTree3D>()?;
    Ok(())
}
