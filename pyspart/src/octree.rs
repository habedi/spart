use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyType;
use std::fs::File;

use spart::geometry::{EuclideanDistance, Point3D};
use spart::octree::Octree;

use crate::geometry::{PyCube};
use crate::point3d::PyPoint3D;
use crate::types::PyData;

#[pyclass(name = "Octree")]
pub struct PyOctree {
    tree: Octree<PyData>,
}

#[pymethods]
impl PyOctree {
    #[new]
    fn new(boundary: PyCube, capacity: usize) -> PyResult<Self> {
        let tree =
            Octree::new(&boundary.0, capacity).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyOctree { tree })
    }

    /// Inserts a point into the octree.
    ///
    /// Args:
    ///     point (Point3D): The point to insert.
    ///
    /// Returns:
    ///     bool: True if the point was successfully inserted, False otherwise.
    fn insert(&mut self, point: PyPoint3D) -> bool {
        self.tree.insert(point.into())
    }

    /// Inserts multiple points into the octree efficiently.
    ///
    /// Args:
    ///     points (list[Point3D]): A list of points to insert.
    /// Points outside the tree's boundary are skipped.
    ///
    /// Returns:
    ///     int: The number of points that were inserted.
    fn insert_bulk(&mut self, points: Vec<PyPoint3D>) -> usize {
        let rust_points: Vec<Point3D<PyData>> = points.into_iter().map(|p| p.into()).collect();
        self.tree.insert_bulk(&rust_points)
    }

    /// Deletes a point from the octree.
    ///
    /// Args:
    ///     point (Point3D): The point to delete.
    ///
    /// Returns:
    ///     bool: True if the point was found and deleted, False otherwise.
    fn delete(&mut self, point: PyPoint3D) -> bool {
        let p: Point3D<PyData> = point.into();
        self.tree.delete(&p)
    }

    /// Finds the k nearest neighbors to the given point.
    ///
    /// Args:
    ///     point (Point3D): The query point to search from.
    ///     k (int): The number of nearest neighbors to find.
    ///
    /// Returns:
    ///     list[Point3D]: A list of the k nearest points found.
    fn knn_search(&self, point: PyPoint3D, k: usize) -> Vec<PyPoint3D> {
        let p: Point3D<PyData> = point.into();
        self.tree
            .knn_search::<EuclideanDistance>(&p, k)
            .into_iter()
            .map(|p| p.into())
            .collect()
    }

    /// Finds all points within a given radius of the query point.
    ///
    /// Args:
    ///     point (Point3D): The center point to search from.
    ///     radius (float): The search radius (using Euclidean distance).
    ///
    /// Returns:
    ///     list[Point3D]: All points within the specified radius.
    fn range_search(&self, point: PyPoint3D, radius: f64) -> Vec<PyPoint3D> {
        let p: Point3D<PyData> = point.into();
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
    ///     point (Point3D): The point to look for.
    ///
    /// Returns:
    ///     bool: True when an equal point is stored.
    fn __contains__(&self, point: PyPoint3D) -> bool {
        let p: Point3D<PyData> = point.into();
        self.tree.contains(&p)
    }

    /// Whether an equal point is stored in the tree.
    ///
    /// Args:
    ///     point (Point3D): The point to look for.
    ///
    /// Returns:
    ///     bool: True when an equal point is stored.
    fn contains(&self, point: PyPoint3D) -> bool {
        let p: Point3D<PyData> = point.into();
        self.tree.contains(&p)
    }

    /// Finds all points inside a query box.
    ///
    /// Args:
    ///     query (CubeDict): The box to search, as a dict of x, y, z, width, height, depth.
    ///
    /// Returns:
    ///     list[Point3D]: All points inside the box.
    fn range_search_bbox(&self, query: PyCube) -> Vec<PyPoint3D> {
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
        Ok(PyOctree { tree })
    }
}
