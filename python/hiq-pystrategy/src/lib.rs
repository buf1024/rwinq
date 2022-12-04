use pyo3::prelude::*;

mod runner;
mod types;

use runner::Runner;

/// A Python module implemented in Rust.
#[pymodule]
fn hiq_pystrategy(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    // import logging
    // import my_module

    // FORMAT = '%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s'
    // logging.basicConfig(format=FORMAT)
    // logging.getLogger().setLevel(logging.INFO)
    // my_module.log_something()
    m.add_class::<Runner>()?;
    Ok(())
}
