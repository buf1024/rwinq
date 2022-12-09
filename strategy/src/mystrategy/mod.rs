use crate::{Error, Result, Strategy};

use self::exam_strategy::ExamStrategy;

mod exam_strategy;

pub fn get_strategy(name: &str) -> Result<Box<dyn Strategy>> {
    if name == "ExamStrategy" {
        let s = ExamStrategy {};

        return Ok(Box::new(s));
    }
    Err(Error::Custom(format!("strategy {} not found", name)))
}

pub fn strategies() -> Vec<String> {
    vec![String::from("ExamStrategy")]
}
