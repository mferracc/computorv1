use std::collections::HashMap;
use crate::math_tools::fixed_point::fixed_point::FixedPoint;
use crate::parser::tools::{parse_equation, split_input, sum_coefficients};

pub fn parse_input(input: &str) -> Result<Vec<FixedPoint>, String> {
    let equation_members: (String, String) = split_input(input)?;

    let left_coefficients: HashMap<usize, FixedPoint> = parse_equation(&equation_members.0)?;
    let right_coefficients: HashMap<usize, FixedPoint> = parse_equation(&equation_members.1)?;

    Ok(sum_coefficients(left_coefficients, right_coefficients))
}