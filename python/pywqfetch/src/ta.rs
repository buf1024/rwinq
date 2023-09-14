use std::collections::BTreeMap;

use pyo3::{
    exceptions::PyException,
    prelude::*,
    types::{PyDict, PyList},
};

use crate::types::ChipDist;

#[pyfunction]
pub fn calc_chip_dist(
    data: PyObject,
    ac: Option<f32>,
    chip_dist: Option<PyObject>,
) -> PyResult<ChipDist> {
    let data = extract_data(data)?;
    let chip_dist = if let Some(dist) = chip_dist {
        Some(extract_chip_dist(dist)?)
    } else {
        None
    };
    let res = rwqfetch::calc_chip_dist(&data, ac, chip_dist)
        .map_err(|e| PyException::new_err(format!("calc_chip_dist error: {}", e)))
        .map(ChipDist::from)?;
    Ok(res)
}

#[pyfunction]
pub fn calc_winner(
    chip_dist: PyObject,
    data: Option<PyObject>,
    price: Option<f32>,
) -> PyResult<BTreeMap<i32, f64>> {
    let dist = extract_chip_dist(chip_dist)?;
    let data = if let Some(data) = data {
        Some(extract_data(data)?)
    } else {
        None
    };
    let res = rwqfetch::calc_winner(&dist, data.as_ref(), price)
        .map_err(|e| PyException::new_err(format!("calc_chip_dist error: {}", e)))?;
    Ok(res)
}
#[pyfunction]
pub fn calc_cost(chip_dist: PyObject, ratio: isize) -> PyResult<BTreeMap<i32, f64>> {
    let dist = extract_chip_dist(chip_dist)?;
    let res = rwqfetch::calc_cost(&dist, ratio)
        .map_err(|e| PyException::new_err(format!("calc_chip_dist error: {}", e)))?;
    Ok(res)
}

fn extract_chip_dist(data: PyObject) -> PyResult<rwqfetch::ChipDist> {
    Python::with_gil(|py| {
        let py_dict: &PyDict = data.downcast(py)?;
        let mut chip: rwqfetch::Chip = BTreeMap::new();
        let mut chip_list: rwqfetch::ChipList = BTreeMap::new();
        for (key, val) in py_dict.into_iter() {
            let key: String = key.extract()?;
            if key == "chip" {
                chip = val.extract()?;
            }

            if key == "chip_list" {
                chip_list = val.extract()?;
            }
        }

        Ok(rwqfetch::ChipDist { chip, chip_list })
    })
}

fn extract_data(data: PyObject) -> PyResult<Vec<rwqfetch::Bar>> {
    Python::with_gil(|py| {
        let mut bars = Vec::new();
        let py_list: &PyList = data.downcast(py)?;
        for item in py_list.into_iter() {
            let mut bar = rwqfetch::Bar::default();
            let py_item: &PyDict = item.downcast()?;
            for (key, val) in py_item {
                let key: String = key.extract()?;
                if key == "code" {
                    bar.code = val.extract()?;
                }
                if key == "name" {
                    bar.name = val.extract()?;
                }
                if key == "trade_date" {
                    bar.trade_date = val.extract()?;
                }
                if key == "open" {
                    bar.open = val.extract()?;
                }
                if key == "close" {
                    bar.close = val.extract()?;
                }
                if key == "high" {
                    bar.high = val.extract()?;
                }
                if key == "low" {
                    bar.low = val.extract()?;
                }
                if key == "volume" {
                    bar.volume = val.extract()?;
                }
                if key == "amount" {
                    bar.amount = val.extract()?;
                }
                if key == "turnover" {
                    bar.turnover = val.extract()?;
                }
                if key == "chg_pct" {
                    bar.chg_pct = val.extract()?;
                }
                if key == "volume_chg_pct" {
                    bar.volume_chg_pct = val.extract()?;
                }
                if key == "amount_chg_pct" {
                    bar.amount_chg_pct = val.extract()?;
                }
                if key == "hfq_factor" {
                    bar.hfq_factor = val.extract()?;
                }
            }

            bars.push(bar);
        }
        Ok(bars)
    })
}
