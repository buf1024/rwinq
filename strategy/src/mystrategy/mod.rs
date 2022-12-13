use crate::{Error, Result, Strategy};

use self::{exam_strategy::ExamStrategy, right_side::RightSide};

mod exam_strategy;
mod right_side;

pub fn get_strategy(name: &str) -> Result<Box<dyn Strategy>> {
    match name {
        "ExamStrategy" => Ok(Box::new(ExamStrategy::default())),
        "RightSide" => Ok(Box::new(RightSide::default())),
        _ => Err(Error::Custom(format!("strategy {} not found", name))),
    }
}

pub fn strategies() -> Vec<String> {
    vec![String::from("ExamStrategy"), String::from("RightSide")]
}
