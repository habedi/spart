use pyo3::basic::CompareOp;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::{PyBytes, PyDict, PyType};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::fs::File;

use spart::geometry::{Cube, EuclideanDistance, Point2D, Point3D, Rectangle};
use spart::kd_tree::KdTree;
use spart::octree::Octree;
use spart::quadtree::Quadtree;
use spart::r_star_tree::RStarTree;
use spart::r_tree::RTree;

// A wrapper around PyObject to allow it to be used as a generic parameter in spart's data structures.
struct PyData(PyObject);

impl Clone for PyData {
    fn clone(&self) -> Self {
        Python::with_gil(|py| {
            PyData(self.0.clone_ref(py))
        })
    }
}

impl PartialEq for PyData {
    fn eq(&self, other: &Self) -> bool {
        Python::with_gil(|py| {
            match self.0.bind(py).rich_compare(&other.0, CompareOp::Eq) {
                Ok(result) => result.is_truthy().unwrap_or(false),
                Err(_) => false,
            }
        })
    }
}
impl Eq for PyData {}

impl PartialOrd for PyData {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Python::with_gil(|py| {
            let self_obj = self.0.bind(py);
            let other_obj = other.0.bind(py);
            if let Ok(result) = self_obj.rich_compare(other_obj, CompareOp::Lt) {
                if result.is_truthy().unwrap_or(false) {
                    return Some(std::cmp::Ordering::Less);
                }
            }
            if let Ok(result) = self_obj.rich_compare(other_obj, CompareOp::Gt) {
                if result.is_truthy().unwrap_or(false) {
                    return Some(std::cmp::Ordering::Greater);
                }
            }
            if let Ok(result) = self_obj.rich_compare(other_obj, CompareOp::Eq) {
                if result.is_truthy().unwrap_or(false) {
                    return Some(std::cmp::Ordering::Equal);
                }
            }
            None
        })
    }
}

impl Ord for PyData {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap_or(std::cmp::Ordering::Equal)
    }
}

// Implement Debug manually since PyObject doesn't implement it.
impl std::fmt::Debug for PyData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        Python::with_gil(|py| {
            write!(f, "PyData({})", self.0.bind(py).repr().unwrap())
        })
    }
}

impl Serialize for PyData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        Python::with_gil(|py| {
            let pickle = py.import("pickle").map_err(serde::ser::Error::custom)?;
            let bound_self = self.0.bind(py);
            let bytes = pickle.call_method1("dumps", (bound_self,)).map_err(serde::ser::Error::custom)?;
            let bytes: &[u8] = bytes.extract().map_err(serde::ser::Error::custom)?;
            serializer.serialize_bytes(bytes)
        })
    }
}

impl<'de> Deserialize<'de> for PyData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let bytes: Vec<u8> = Vec::deserialize(deserializer)?;
        Python::with_gil(|py| {
            let pickle = py.import("pickle").map_err(serde::de::Error::custom)?;
            let obj = pickle.call_method("loads", (PyBytes::new(py, &bytes),), None).map_err(serde::de::Error::custom)?;
            Ok(PyData(obj.into()))
        })
    }
}


#[pyclass(name = "Point2D", get_all)]
#[derive(Debug)]
struct PyPoint2D {
    x: f64,
    y: f64,
    data: PyObject,
}

impl Clone for PyPoint2D {
    fn clone(&self) -> Self {
        Python::with_gil(|py| {
            PyPoint2D {
                x: self.x,
                y: self.y,
                data: self.data.clone_ref(py),
            }
        })
    }
}

impl PartialEq for PyPoint2D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && Python::with_gil(|py| {
            match self.data.bind(py).rich_compare(&other.data, CompareOp::Eq) {
                Ok(result) => result.is_truthy().unwrap_or(false),
                Err(_) => false,
            }
        })
    }
}

#[pymethods]
impl PyPoint2D {
    #[new]
    fn new(x: f64, y: f64, data: PyObject) -> Self {
        PyPoint2D { x, y, data }
    }
}

impl From<PyPoint2D> for Point2D<PyData> {
    fn from(p: PyPoint2D) -> Self {
        Point2D::new(p.x, p.y, Some(PyData(p.data)))
    }
}

impl From<Point2D<PyData>> for PyPoint2D {
    fn from(p: Point2D<PyData>) -> Self {
        PyPoint2D {
            x: p.x,
            y: p.y,
            data: p.data.unwrap().0,
        }
    }
}

#[pyclass(name = "Point3D", get_all)]
#[derive(Debug)]
struct PyPoint3D {
    x: f64,
    y: f64,
    z: f64,
    data: PyObject,
}

impl Clone for PyPoint3D {
    fn clone(&self) -> Self {
        Python::with_gil(|py| {
            PyPoint3D {
                x: self.x,
                y: self.y,
                z: self.z,
                data: self.data.clone_ref(py),
            }
        })
    }
}

impl PartialEq for PyPoint3D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z && Python::with_gil(|py| {
            match self.data.bind(py).rich_compare(&other.data, CompareOp::Eq) {
                Ok(result) => result.is_truthy().unwrap_or(false),
                Err(_) => false,
            }
        })
    }
}

#[pymethods]
impl PyPoint3D {
    #[new]
    fn new(x: f64, y: f64, z: f64, data: PyObject) -> Self {
        PyPoint3D { x, y, z, data }
    }
}

impl From<PyPoint3D> for Point3D<PyData> {
    fn from(p: PyPoint3D) -> Self {
        Point3D::new(p.x, p.y, p.z, Some(PyData(p.data)))
    }
}

impl From<Point3D<PyData>> for PyPoint3D {
    fn from(p: Point3D<PyData>) -> Self {
        PyPoint3D {
            x: p.x,
            y: p.y,
            z: p.z,
            data: p.data.unwrap().0,
        }
    }
}

#[derive(Clone)]
struct PyRectangle(Rectangle);

impl<'source> FromPyObject<'source> for PyRectangle {
    fn extract_bound(ob: &Bound<'source, PyAny>) -> PyResult<Self> {
        let dict: &Bound<PyDict> = ob.downcast()?;
        let x: f64 = dict.get_item("x")?.ok_or_else(|| PyValueError::new_err("missing 'x'"))?.extract()?;
        let y: f64 = dict.get_item("y")?.ok_or_else(|| PyValueError::new_err("missing 'y'"))?.extract()?;
        let width: f64 = dict.get_item("width")?.ok_or_else(|| PyValueError::new_err("missing 'width'"))?.extract()?;
        let height: f64 = dict.get_item("height")?.ok_or_else(|| PyValueError::new_err("missing 'height'"))?.extract()?;
        Ok(PyRectangle(Rectangle { x, y, width, height }))
    }
}

#[derive(Clone)]
struct PyCube(Cube);

impl<'source> FromPyObject<'source> for PyCube {
    fn extract_bound(ob: &Bound<'source, PyAny>) -> PyResult<Self> {
        let dict: &Bound<PyDict> = ob.downcast()?;
        let x: f64 = dict.get_item("x")?.ok_or_else(|| PyValueError::new_err("missing 'x'"))?.extract()?;
        let y: f64 = dict.get_item("y")?.ok_or_else(|| PyValueError::new_err("missing 'y'"))?.extract()?;
        let z: f64 = dict.get_item("z")?.ok_or_else(|| PyValueError::new_err("missing 'z'"))?.extract()?;
        let width: f64 = dict.get_item("width")?.ok_or_else(|| PyValueError::new_err("missing 'width'"))?.extract()?;
        let height: f64 = dict.get_item("height")?.ok_or_else(|| PyValueError::new_err("missing 'height'"))?.extract()?;
        let depth: f64 = dict.get_item("depth")?.ok_or_else(|| PyValueError::new_err("missing 'depth'"))?.extract()?;
        Ok(PyCube(Cube { x, y, z, width, height, depth }))
    }
}


#[pyclass(name = "Quadtree")]
struct PyQuadtree {
    tree: Quadtree<PyData>,
}

#[pymethods]
impl PyQuadtree {
    #[new]
    fn new(boundary: PyRectangle, capacity: usize) -> Self {
        PyQuadtree {
            tree: Quadtree::new(&boundary.0, capacity),
        }
    }

    fn insert(&mut self, point: PyPoint2D) -> bool {
        self.tree.insert(point.into())
    }

    fn insert_bulk(&mut self, points: Vec<PyPoint2D>) {
        let rust_points: Vec<Point2D<PyData>> = points.into_iter().map(|p| p.into()).collect();
        self.tree.insert_bulk(&rust_points);
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
            .map(|p| p.into())
            .collect()
    }

    fn range_search(&self, point: PyPoint2D, radius: f64) -> Vec<PyPoint2D> {
        let p: Point2D<PyData> = point.into();
        self.tree
            .range_search::<EuclideanDistance>(&p, radius)
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
        let tree = bincode::deserialize_from(file).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyQuadtree { tree })
    }
}

#[pyclass(name = "Octree")]
struct PyOctree {
    tree: Octree<PyData>,
}

#[pymethods]
impl PyOctree {
    #[new]
    fn new(boundary: PyCube, capacity: usize) -> Self {
        PyOctree {
            tree: Octree::new(&boundary.0, capacity),
        }
    }

    fn insert(&mut self, point: PyPoint3D) -> bool {
        self.tree.insert(point.into())
    }

    fn insert_bulk(&mut self, points: Vec<PyPoint3D>) {
        let rust_points: Vec<Point3D<PyData>> = points.into_iter().map(|p| p.into()).collect();
        self.tree.insert_bulk(&rust_points);
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
            .map(|p| p.into())
            .collect()
    }

    fn range_search(&self, point: PyPoint3D, radius: f64) -> Vec<PyPoint3D> {
        let p: Point3D<PyData> = point.into();
        self.tree
            .range_search::<EuclideanDistance>(&p, radius)
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
        let tree = bincode::deserialize_from(file).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyOctree { tree })
    }
}

#[pyclass(name = "KdTree2D")]
struct PyKdTree2D {
    tree: KdTree<Point2D<PyData>>,
}

#[pymethods]
impl PyKdTree2D {
    #[new]
    fn new() -> Self {
        PyKdTree2D {
            tree: KdTree::new(2),
        }
    }

    fn insert(&mut self, point: PyPoint2D) {
        self.tree.insert(point.into())
    }

    fn insert_bulk(&mut self, points: Vec<PyPoint2D>) {
        let mut rust_points: Vec<Point2D<PyData>> = points.into_iter().map(|p| p.into()).collect();
        self.tree.insert_bulk(&mut rust_points);
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
            .map(|p| p.into())
            .collect()
    }

    fn range_search(&self, point: PyPoint2D, radius: f64) -> Vec<PyPoint2D> {
        let p: Point2D<PyData> = point.into();
        self.tree
            .range_search::<EuclideanDistance>(&p, radius)
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
        let tree = bincode::deserialize_from(file).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyKdTree2D { tree })
    }
}

#[pyclass(name = "KdTree3D")]
struct PyKdTree3D {
    tree: KdTree<Point3D<PyData>>,
}

#[pymethods]
impl PyKdTree3D {
    #[new]
    fn new() -> Self {
        PyKdTree3D {
            tree: KdTree::new(3),
        }
    }

    fn insert(&mut self, point: PyPoint3D) {
        self.tree.insert(point.into())
    }

    fn insert_bulk(&mut self, points: Vec<PyPoint3D>) {
        let mut rust_points: Vec<Point3D<PyData>> = points.into_iter().map(|p| p.into()).collect();
        self.tree.insert_bulk(&mut rust_points);
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
            .map(|p| p.into())
            .collect()
    }

    fn range_search(&self, point: PyPoint3D, radius: f64) -> Vec<PyPoint3D> {
        let p: Point3D<PyData> = point.into();
        self.tree
            .range_search::<EuclideanDistance>(&p, radius)
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
        let tree = bincode::deserialize_from(file).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyKdTree3D { tree })
    }
}

#[pyclass(name = "RTree2D")]
struct PyRTree2D {
    tree: RTree<Point2D<PyData>>,
}

#[pymethods]
impl PyRTree2D {
    #[new]
    fn new(max_entries: usize) -> Self {
        PyRTree2D {
            tree: RTree::new(max_entries),
        }
    }

    fn insert(&mut self, point: PyPoint2D) {
        self.tree.insert(point.into())
    }

    fn insert_bulk(&mut self, points: Vec<PyPoint2D>) {
        let rust_points: Vec<Point2D<PyData>> = points.into_iter().map(|p| p.into()).collect();
        self.tree.insert_bulk(rust_points);
    }

    fn delete(&mut self, point: PyPoint2D) -> bool {
        let p: Point2D<PyData> = point.into();
        self.tree.delete(&p)
    }

    fn range_search(&self, point: PyPoint2D, radius: f64) -> Vec<PyPoint2D> {
        let p: Point2D<PyData> = point.into();
        self.tree
            .range_search::<EuclideanDistance>(&p, radius)
            .into_iter()
            .cloned()
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
        let tree = bincode::deserialize_from(file).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyRTree2D { tree })
    }
}

#[pyclass(name = "RTree3D")]
struct PyRTree3D {
    tree: RTree<Point3D<PyData>>,
}

#[pymethods]
impl PyRTree3D {
    #[new]
    fn new(max_entries: usize) -> Self {
        PyRTree3D {
            tree: RTree::new(max_entries),
        }
    }

    fn insert(&mut self, point: PyPoint3D) {
        self.tree.insert(point.into())
    }

    fn insert_bulk(&mut self, points: Vec<PyPoint3D>) {
        let rust_points: Vec<Point3D<PyData>> = points.into_iter().map(|p| p.into()).collect();
        self.tree.insert_bulk(rust_points);
    }

    fn delete(&mut self, point: PyPoint3D) -> bool {
        let p: Point3D<PyData> = point.into();
        self.tree.delete(&p)
    }

    fn range_search(&self, point: PyPoint3D, radius: f64) -> Vec<PyPoint3D> {
        let p: Point3D<PyData> = point.into();
        self.tree
            .range_search::<EuclideanDistance>(&p, radius)
            .into_iter()
            .cloned()
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
        let tree = bincode::deserialize_from(file).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyRTree3D { tree })
    }
}


#[pyclass(name = "RStarTree2D")]
struct PyRStarTree2D {
    tree: RStarTree<Point2D<PyData>>,
}

#[pymethods]
impl PyRStarTree2D {
    #[new]
    fn new(max_entries: usize) -> Self {
        PyRStarTree2D {
            tree: RStarTree::new(max_entries),
        }
    }

    fn insert(&mut self, point: PyPoint2D) {
        self.tree.insert(point.into())
    }

    fn insert_bulk(&mut self, points: Vec<PyPoint2D>) {
        let rust_points: Vec<Point2D<PyData>> = points.into_iter().map(|p| p.into()).collect();
        self.tree.insert_bulk(rust_points);
    }

    fn delete(&mut self, point: PyPoint2D) -> bool {
        let p: Point2D<PyData> = point.into();
        self.tree.delete(&p)
    }

    fn range_search(&self, point: PyPoint2D, radius: f64) -> Vec<PyPoint2D> {
        let p: Point2D<PyData> = point.into();
        self.tree
            .range_search::<EuclideanDistance>(&p, radius)
            .into_iter()
            .cloned()
            .map(|p| p.into())
            .collect()
    }

    fn knn_search(&self, point: PyPoint2D, k: usize) -> Vec<PyPoint2D> {
        let p: Point2D<PyData> = point.into();
        self.tree
            .knn_search::<EuclideanDistance>(&p, k)
            .into_iter()
            .cloned()
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
        let tree = bincode::deserialize_from(file).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyRStarTree2D { tree })
    }
}

#[pyclass(name = "RStarTree3D")]
struct PyRStarTree3D {
    tree: RStarTree<Point3D<PyData>>,
}

#[pymethods]
impl PyRStarTree3D {
    #[new]
    fn new(max_entries: usize) -> Self {
        PyRStarTree3D {
            tree: RStarTree::new(max_entries),
        }
    }

    fn insert(&mut self, point: PyPoint3D) {
        self.tree.insert(point.into())
    }

    fn insert_bulk(&mut self, points: Vec<PyPoint3D>) {
        let rust_points: Vec<Point3D<PyData>> = points.into_iter().map(|p| p.into()).collect();
        self.tree.insert_bulk(rust_points);
    }

    fn delete(&mut self, point: PyPoint3D) -> bool {
        let p: Point3D<PyData> = point.into();
        self.tree.delete(&p)
    }

    fn range_search(&self, point: PyPoint3D, radius: f64) -> Vec<PyPoint3D> {
        let p: Point3D<PyData> = point.into();
        self.tree
            .range_search::<EuclideanDistance>(&p, radius)
            .into_iter()
            .cloned()
            .map(|p| p.into())
            .collect()
    }

    fn knn_search(&self, point: PyPoint3D, k: usize) -> Vec<PyPoint3D> {
        let p: Point3D<PyData> = point.into();
        self.tree
            .knn_search::<EuclideanDistance>(&p, k)
            .into_iter()
            .cloned()
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
        let tree = bincode::deserialize_from(file).map_err(|e| PyValueError::new_err(e.to_string()))?;
        Ok(PyRStarTree3D { tree })
    }
}


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
