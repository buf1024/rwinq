use pyo3::{prelude::*, types::PyDict};

#[derive(Debug, Clone)]
pub struct ChipDist {
    pub(crate) inner: rwqfetch::ChipDist,
}

impl From<rwqfetch::ChipDist> for ChipDist {
    fn from(inner: rwqfetch::ChipDist) -> Self {
        Self { inner }
    }
}

impl IntoPy<PyObject> for ChipDist {
    fn into_py(self, py: Python<'_>) -> PyObject {
        let dict = PyDict::new(py);
        let chip_dict = PyDict::new(py);
        self.inner.chip.iter().for_each(|(k, v)| {
            chip_dict.set_item(*k, *v).unwrap();
        });
        let chip_list_dict = PyDict::new(py);
        self.inner.chip_list.iter().for_each(|(k, v)| {
            let chip_dict = PyDict::new(py);
            v.iter().for_each(|(k2, v2)| {
                chip_dict.set_item(*k2, *v2).unwrap();
            });
            chip_list_dict.set_item(*k, chip_dict).unwrap();
        });
        dict.set_item("chip", chip_dict).unwrap();
        dict.set_item("chip_list", chip_list_dict).unwrap();

        dict.into()
    }
}
