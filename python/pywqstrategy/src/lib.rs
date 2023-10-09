use pyo3::{exceptions::PyException, prelude::*};
use rwqstrategy::Bar;

mod runner;
mod types;

mod ma;
use ma::ta_ma;

// use runner::Runner;
// use types::Stat;

// #[pyfunction]
// fn stat_result(data: &str, hit: usize, hit_max: usize) -> PyResult<Stat> {
//     let data: Vec<Bar> =
//         serde_json::from_str(data).map_err(|e| PyException::new_err(e.to_string()))?;
//     let stat = rwqstrategy::select::stat_result(&data, hit, hit_max)
//         .map_err(|e| PyException::new_err(e.to_string()))?;
//     Ok(Stat::from(stat))
// }

/// A Python module implemented in Rust.
#[pymodule]
fn pywqstrategy(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    // import logging
    // import my_module

    // FORMAT = '%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s'
    // logging.basicConfig(format=FORMAT)
    // logging.getLogger().setLevel(logging.INFO)
    // my_module.log_something()
    // m.add_function(wrap_pyfunction!(stat_result, m)?)?;
    m.add_function(wrap_pyfunction!(ta_ma, m)?)?;
    // m.add_class::<Runner>()?;
    Ok(())
}
