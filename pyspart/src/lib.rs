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
