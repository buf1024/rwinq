use pyo3::prelude::*;

mod pywqsync;
use pywqsync::{BlockSync, Sync};

mod pywqmongo;
use pywqmongo::{BlockMongoLoader, MongoLoader};

/// A Python module implemented in Rust.
#[pymodule]
fn pywqdata(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    // import logging
    // import my_module

    // FORMAT = '%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s'
    // logging.basicConfig(format=FORMAT)
    // logging.getLogger().setLevel(logging.INFO)
    // my_module.log_something()
    m.add_class::<Sync>()?;
    m.add_class::<BlockSync>()?;
    m.add_class::<MongoLoader>()?;
    m.add_class::<BlockMongoLoader>()?;
    Ok(())
}
