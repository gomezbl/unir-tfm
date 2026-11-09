use pyo3::{exceptions::PyValueError, PyErr};
use qsim_core::QSimError;

pub(crate) fn to_python_error(error: QSimError) -> PyErr {
    PyValueError::new_err(error.to_string())
}
