use pyo3::{exceptions::PyException, prelude::*};
use pywqcmm::{to_python, to_rust};

#[pyfunction]
pub fn calc_chip_dist(
    data: PyObject,
    ac: Option<f32>,
    chip_dist: Option<PyObject>,
) -> PyResult<PyObject> {
    let data = Python::with_gil(|py| to_rust(data.as_ref(py)))?;
    let chip_dist = match chip_dist {
        Some(dist) => Python::with_gil(|py| to_rust(dist.as_ref(py)))?,
        None => None,
    };

    to_python(
        &rwqfetch::calc_chip_dist(&data, ac, chip_dist)
            .map_err(|e| PyException::new_err(e.to_string()))?,
    )
}

#[pyfunction]
pub fn calc_winner(
    chip_dist: PyObject,
    data: Option<PyObject>,
    price: Option<f32>,
) -> PyResult<PyObject> {
    let dist = Python::with_gil(|py| to_rust(chip_dist.as_ref(py)))?;
    let data = match data {
        Some(data) => Python::with_gil(|py| to_rust(data.as_ref(py)))?,
        None => None,
    };

    to_python(
        &rwqfetch::calc_winner(&dist, data.as_ref(), price)
            .map_err(|e| PyException::new_err(e.to_string()))?,
    )
}
#[pyfunction]
pub fn calc_cost(chip_dist: PyObject, ratio: isize) -> PyResult<PyObject> {
    let dist = Python::with_gil(|py| to_rust(chip_dist.as_ref(py)))?;
    to_python(&rwqfetch::calc_cost(&dist, ratio).map_err(|e| PyException::new_err(e.to_string()))?)
}
