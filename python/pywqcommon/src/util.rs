use pyo3::{exceptions::PyException, PyAny, PyObject, PyResult, Python};
use serde::{Deserialize, Serialize};
use tokio::runtime::Runtime;

pub fn runtime() -> PyResult<Runtime> {
    Ok(tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| PyException::new_err(e.to_string()))?)
}

pub fn to_python<T>(data: &T) -> PyResult<PyObject>
where
    T: ?Sized + Serialize,
{
    Ok(Python::with_gil(move |py| pythonize::pythonize(py, &data))
        .map_err(|e| PyException::new_err(e.to_string()))?)
}

pub fn to_rust<'de, T>(data: &'de PyAny) -> PyResult<T>
where
    T: Deserialize<'de>,
{
    pythonize::depythonize(data).map_err(|e| PyException::new_err(e.to_string()))
}
