use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use spart::geometry::Point3D;

use crate::types::PyData;

#[pyclass(name = "Point3D", get_all)]
#[derive(Debug)]
pub struct PyPoint3D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub data: PyObject,
}

impl Clone for PyPoint3D {
    fn clone(&self) -> Self {
        Python::with_gil(|py| PyPoint3D {
            x: self.x,
            y: self.y,
            z: self.z,
            data: self.data.clone_ref(py),
        })
    }
}

impl PartialEq for PyPoint3D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.y == other.y
            && self.z == other.z
            && Python::with_gil(|py| {
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

impl From<&Point3D<PyData>> for PyPoint3D {
    fn from(p: &Point3D<PyData>) -> Self {
        Python::with_gil(|py| PyPoint3D {
            x: p.x,
            y: p.y,
            z: p.z,
            data: p.data.as_ref().unwrap().0.clone_ref(py),
        })
    }
}
