use crate::parser::tools::{parse_equation, split_input, sum_coefficients};
use std::collections::HashMap;

pub fn parse_input(input: &str) -> Result<Vec<f64>, String> {
    let equation_members: (String, String) = split_input(input)?;

    let left_coefficients: HashMap<usize, f64> = parse_equation(&equation_members.0)?;
    let right_coefficients: HashMap<usize, f64> = parse_equation(&equation_members.1)?;

    Ok(sum_coefficients(left_coefficients, right_coefficients))
}
