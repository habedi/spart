use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyType;
use std::fs::File;

use spart::geometry::{EuclideanDistance, Point2D};
use spart::quadtree::Quadtree;

use crate::geometry::PyRectangle;
use crate::point2d::PyPoint2D;
use crate::types::PyData;

#[pyclass(name = "Quadtree")]
pub struct PyQuadtree {
    tree: Quadtree<PyData>,
}

#[pymethods]
impl PyQuadtree {
    #[new]
    fn new(boundary: PyRectangle, capacity: usize) -> PyResult<Self> {
        let tree = Quadtree::new(&boundary.0, capacity)
            .map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyQuadtree { tree })
    }

    /// Inserts a point into the quadtree.
    ///
    /// Args:
    ///     point (Point2D): The point to insert.
    ///
    /// Returns:
    ///     bool: True if the point was successfully inserted, False otherwise.
    fn insert(&mut self, point: PyPoint2D) -> bool {
        self.tree.insert(point.into())
    }

    /// Inserts multiple points into the quadtree efficiently.
    ///
    /// Args:
    ///     points (list[Point2D]): A list of points to insert.
    fn insert_bulk(&mut self, points: Vec<PyPoint2D>) {
        let rust_points: Vec<Point2D<PyData>> = points.into_iter().map(|p| p.into()).collect();
        self.tree.insert_bulk(&rust_points);
    }

    /// Deletes a point from the quadtree.
    ///
    /// Args:
    ///     point (Point2D): The point to delete.
    ///
    /// Returns:
    ///     bool: True if the point was found and deleted, False otherwise.
    fn delete(&mut self, point: PyPoint2D) -> bool {
        let p: Point2D<PyData> = point.into();
        self.tree.delete(&p)
    }

    /// Finds the k nearest neighbors to the given point.
    ///
    /// Finds the k nearest neighbors to the given point.
    ///
    /// Args:
    ///     point (Point2D): The query point to search from.
    ///     k (int): The number of nearest neighbors to find.
    ///
    /// Returns:
    ///     list[Point2D]: A list of the k nearest points found.
    fn knn_search(&self, point: PyPoint2D, k: usize) -> Vec<PyPoint2D> {
        let p: Point2D<PyData> = point.into();
        self.tree
            .knn_search::<EuclideanDistance>(&p, k)
            .into_iter()
            .map(|p| (&p).into())
            .collect()
    }

    /// Finds all points within a given radius of the query point.
    ///
    /// Args:
    ///     point (Point2D): The center point to search from.
    ///     radius (float): The search radius (using Euclidean distance).
    ///
    /// Returns:
    ///     list[Point2D]: All points within the specified radius.
    fn range_search(&self, point: PyPoint2D, radius: f64) -> Vec<PyPoint2D> {
        let p: Point2D<PyData> = point.into();
        self.tree
            .range_search::<EuclideanDistance>(&p, radius)
            .into_iter()
            .map(|p| (&p).into())
            .collect()
    }

    /// Saves the tree to a file.
    ///
    /// Args:
    ///     path (str): The path to the file.
    fn save(&self, path: &str) -> PyResult<()> {
        let file = File::create(path)?;
        bincode::serialize_into(file, &self.tree).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    /// Loads a tree from a file.
    ///
    /// Args:
    ///     path (str): The path to the file.
    ///
    /// Returns:
    ///     The loaded tree.
    #[classmethod]
    fn load(_cls: &Bound<PyType>, path: &str) -> PyResult<Self> {
        let file = File::open(path)?;
        let tree =
            bincode::deserialize_from(file).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyQuadtree { tree })
    }
}
