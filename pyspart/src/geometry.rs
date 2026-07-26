use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use spart::geometry::{Cube, Rectangle};

#[derive(Clone)]
pub struct PyRectangle(pub Rectangle);

impl<'a, 'py> FromPyObject<'a, 'py> for PyRectangle {
    type Error = PyErr;

    fn extract(ob: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        let dict = ob.cast::<PyDict>()?;
        let x: f64 = dict
            .get_item("x")?
            .ok_or_else(|| PyValueError::new_err("missing 'x'"))?
            .extract()?;
        let y: f64 = dict
            .get_item("y")?
            .ok_or_else(|| PyValueError::new_err("missing 'y'"))?
            .extract()?;
        let width: f64 = dict
            .get_item("width")?
            .ok_or_else(|| PyValueError::new_err("missing 'width'"))?
            .extract()?;
        let height: f64 = dict
            .get_item("height")?
            .ok_or_else(|| PyValueError::new_err("missing 'height'"))?
            .extract()?;
        Ok(PyRectangle(Rectangle {
            x,
            y,
            width,
            height,
        }))
    }
}

#[derive(Clone)]
pub struct PyCube(pub Cube);

impl<'a, 'py> FromPyObject<'a, 'py> for PyCube {
    type Error = PyErr;

    fn extract(ob: Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        let dict = ob.cast::<PyDict>()?;
        let x: f64 = dict
            .get_item("x")?
            .ok_or_else(|| PyValueError::new_err("missing 'x'"))?
            .extract()?;
        let y: f64 = dict
            .get_item("y")?
            .ok_or_else(|| PyValueError::new_err("missing 'y'"))?
            .extract()?;
        let z: f64 = dict
            .get_item("z")?
            .ok_or_else(|| PyValueError::new_err("missing 'z'"))?
            .extract()?;
        let width: f64 = dict
            .get_item("width")?
            .ok_or_else(|| PyValueError::new_err("missing 'width'"))?
            .extract()?;
        let height: f64 = dict
            .get_item("height")?
            .ok_or_else(|| PyValueError::new_err("missing 'height'"))?
            .extract()?;
        let depth: f64 = dict
            .get_item("depth")?
            .ok_or_else(|| PyValueError::new_err("missing 'depth'"))?
            .extract()?;
        Ok(PyCube(Cube {
            x,
            y,
            z,
            width,
            height,
            depth,
        }))
    }
}
