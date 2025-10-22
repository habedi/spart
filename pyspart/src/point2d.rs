use pyo3::basic::CompareOp;
use pyo3::prelude::*;
use spart::geometry::Point2D;

use crate::types::PyData;

#[pyclass(name = "Point2D", get_all)]
#[derive(Debug)]
pub struct PyPoint2D {
    pub x: f64,
    pub y: f64,
    pub data: PyObject,
}

impl Clone for PyPoint2D {
    fn clone(&self) -> Self {
        Python::with_gil(|py| PyPoint2D {
            x: self.x,
            y: self.y,
            data: self.data.clone_ref(py),
        })
    }
}

impl PartialEq for PyPoint2D {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x
            && self.y == other.y
            && Python::with_gil(|py| {
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

    /// Python equality comparison.
    fn __eq__(&self, other: &Self) -> bool {
        self.eq(other)
    }

    /// Python hash function (not hashable due to mutable data).
    fn __hash__(&self) -> PyResult<isize> {
        Err(pyo3::exceptions::PyTypeError::new_err(
            "Point2D objects are not hashable",
        ))
    }
}

impl From<PyPoint2D> for Point2D<PyData> {
    fn from(p: PyPoint2D) -> Self {
        Point2D::new(p.x, p.y, Some(PyData(p.data)))
    }
}

impl From<&Point2D<PyData>> for PyPoint2D {
    fn from(p: &Point2D<PyData>) -> Self {
        Python::with_gil(|py| PyPoint2D {
            x: p.x,
            y: p.y,
            data: p
                .data
                .as_ref()
                .expect("Point2D data should not be None in Python bindings")
                .0
                .clone_ref(py),
        })
    }
}

impl From<Point2D<PyData>> for PyPoint2D {
    fn from(p: Point2D<PyData>) -> Self {
        (&p).into()
    }
}
