use pyo3::{exceptions::PyException, PyResult};
use tokio::runtime::Runtime;

pub fn runtime() -> PyResult<Runtime> {
    
    Ok(tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .map_err(|e| PyException::new_err(e.to_string()))?)
}
