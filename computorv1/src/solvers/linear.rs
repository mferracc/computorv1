use crate::constants::math_tools_constants::POSITIVE;
use crate::math_tools::fixed_point::fixed_point::FixedPoint;

pub fn solve_linear(coefficients: &Vec<FixedPoint>) -> Option<Vec<FixedPoint>> {
    if coefficients.len() < 2 {
        return None;
    }

    let a: FixedPoint = coefficients[1];
    let b: FixedPoint = coefficients[0];

    if a == FixedPoint::new(0,0,POSITIVE) {
        None
    } else {
        let solution: FixedPoint = -b/a;
        Some(vec![solution])
    }
}
