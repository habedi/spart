use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyType;
use std::fs::File;

use spart::geometry::{EuclideanDistance, Point2D, Point3D};
use spart::kdtree::KdTree;

use crate::point2d::PyPoint2D;
use crate::point3d::PyPoint3D;
use crate::types::PyData;

#[pyclass(name = "KdTree2D")]
pub struct PyKdTree2D {
    tree: KdTree<Point2D<PyData>>,
}

#[pymethods]
impl PyKdTree2D {
    #[new]
    fn new() -> Self {
        PyKdTree2D {
            tree: KdTree::new(),
        }
    }

    fn insert(&mut self, point: PyPoint2D) -> PyResult<()> {
        self.tree.insert(point.into()).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn insert_bulk(&mut self, points: Vec<PyPoint2D>) {
        let rust_points: Vec<Point2D<PyData>> = points.into_iter().map(|p| p.into()).collect();
        let _ = self.tree.insert_bulk(rust_points);
    }

    fn delete(&mut self, point: PyPoint2D) -> bool {
        let p: Point2D<PyData> = point.into();
        self.tree.delete(&p)
    }

    fn knn_search(&self, point: PyPoint2D, k: usize) -> Vec<PyPoint2D> {
        let p: Point2D<PyData> = point.into();
        self.tree
            .knn_search::<EuclideanDistance>(&p, k)
            .into_iter()
            .map(|p| (&p).into())
            .collect()
    }

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
        let tree = bincode::deserialize_from(file).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyKdTree2D { tree })
    }
}

#[pyclass(name = "KdTree3D")]
pub struct PyKdTree3D {
    tree: KdTree<Point3D<PyData>>,
}

#[pymethods]
impl PyKdTree3D {
    #[new]
    fn new() -> Self {
        PyKdTree3D {
            tree: KdTree::new(),
        }
    }

    fn insert(&mut self, point: PyPoint3D) -> PyResult<()> {
        self.tree.insert(point.into()).map_err(|e| PyValueError::new_err(e.to_string()))
    }

    fn insert_bulk(&mut self, points: Vec<PyPoint3D>) {
        let rust_points: Vec<Point3D<PyData>> = points.into_iter().map(|p| p.into()).collect();
        let _ = self.tree.insert_bulk(rust_points);
    }

    fn delete(&mut self, point: PyPoint3D) -> bool {
        let p: Point3D<PyData> = point.into();
        self.tree.delete(&p)
    }

    fn knn_search(&self, point: PyPoint3D, k: usize) -> Vec<PyPoint3D> {
        let p: Point3D<PyData> = point.into();
        self.tree
            .knn_search::<EuclideanDistance>(&p, k)
            .into_iter()
            .map(|p| (&p).into())
            .collect()
    }

    fn range_search(&self, point: PyPoint3D, radius: f64) -> Vec<PyPoint3D> {
        let p: Point3D<PyData> = point.into();
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
        let tree = bincode::deserialize_from(file).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyKdTree3D { tree })
    }
}

