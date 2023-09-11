use pyo3::prelude::*;

#[pyfunction]
#[allow(non_snake_case)]
pub(crate) fn ta_ma(bar: Vec<f32>, ma_type: usize) -> PyResult<Vec<f32>> {
    Ok(rwqstrategy::ta::MA(&bar, ma_type))
}
