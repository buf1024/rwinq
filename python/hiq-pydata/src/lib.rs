use pyo3::prelude::*;

mod hiq_sync;

use hiq_sync::{BlockHiqSync, HiqSync};

mod hiq_mongo;
use hiq_mongo::{BlockMongoFetch, MongoFetch};

/// A Python module implemented in Rust.
#[pymodule]
fn hiq_pydata(_py: Python, m: &PyModule) -> PyResult<()> {
    pyo3_log::init();
    // import logging
    // import my_module

    // FORMAT = '%(levelname)s %(name)s %(asctime)-15s %(filename)s:%(lineno)d %(message)s'
    // logging.basicConfig(format=FORMAT)
    // logging.getLogger().setLevel(logging.INFO)
    // my_module.log_something()
    m.add_class::<HiqSync>()?;
    m.add_class::<BlockHiqSync>()?;
    m.add_class::<MongoFetch>()?;
    m.add_class::<BlockMongoFetch>()?;
    Ok(())
}
