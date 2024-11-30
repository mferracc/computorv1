use crate::constants::math_tools_constants::ZERO;
use crate::math_tools::fixed_point::fixed_point::FixedPoint;

pub fn solve_linear(coefficients: &Vec<FixedPoint>) -> Option<Vec<FixedPoint>> {
    if coefficients.len() < 2 {
        return None;
    }

    let a: FixedPoint = coefficients[1];
    let b: FixedPoint = coefficients[0];

    if a == ZERO {
        println!("Linear equation with no solution (a = 0).");
        None
    } else {
        let solution: FixedPoint = -b/a;
        dbg!(&solution);
        println!("Linear equation with one solution: -b/a = {:.}/{:.}", -b.to_f64(), a.to_f64());
        Some(vec![solution])
    }
}
