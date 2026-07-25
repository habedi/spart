use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyType;
use std::fs::File;

use spart::geometry::{EuclideanDistance, Point2D};
use spart::quadtree::Quadtree;

use crate::geometry::{PyRectangle};
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
    /// Points outside the tree's boundary are skipped.
    ///
    /// Args:
    ///     points (list[Point2D]): A list of points to insert.
    ///
    /// Returns:
    ///     int: The number of points that were inserted.
    fn insert_bulk(&mut self, points: Vec<PyPoint2D>) -> usize {
        let rust_points: Vec<Point2D<PyData>> = points.into_iter().map(|p| p.into()).collect();
        self.tree.insert_bulk(&rust_points)
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
            .map(|p| p.into())
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
            .map(|p| p.into())
            .collect()
    }

    /// Number of points held in the tree.
    fn __len__(&self) -> usize {
        self.tree.len()
    }

    /// Number of points held in the tree.
    ///
    /// Returns:
    ///     int: The number of points.
    fn len(&self) -> usize {
        self.tree.len()
    }

    /// Whether the tree holds no points.
    ///
    /// Returns:
    ///     bool: True when the tree is empty.
    fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }

    /// Removes every point from the tree.
    fn clear(&mut self) {
        self.tree.clear();
    }

    /// Whether an equal point is stored in the tree.
    ///
    /// Args:
    ///     point (Point2D): The point to look for.
    ///
    /// Returns:
    ///     bool: True when an equal point is stored.
    fn __contains__(&self, point: PyPoint2D) -> bool {
        let p: Point2D<PyData> = point.into();
        self.tree.contains(&p)
    }

    /// Whether an equal point is stored in the tree.
    ///
    /// Args:
    ///     point (Point2D): The point to look for.
    ///
    /// Returns:
    ///     bool: True when an equal point is stored.
    fn contains(&self, point: PyPoint2D) -> bool {
        let p: Point2D<PyData> = point.into();
        self.tree.contains(&p)
    }

    /// Finds all points inside a query box.
    ///
    /// Args:
    ///     query (RectangleDict): The box to search, as a dict of x, y, width, height.
    ///
    /// Returns:
    ///     list[Point2D]: All points inside the box.
    fn range_search_bbox(&self, query: PyRectangle) -> Vec<PyPoint2D> {
        self.tree
            .range_search_bbox(&query.0)
            .into_iter()
            .map(|p| p.into())
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
