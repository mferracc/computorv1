use crate::constants::math_tools_constants::ZERO;
use crate::math_tools::basic;
use crate::math_tools::fixed_point::fixed_point::FixedPoint;

pub fn solve_quadratic(coefficients: &Vec<FixedPoint>) -> Option<Vec<FixedPoint>> {
    if coefficients.len() != 3 {
        panic!("Wrong solver used.")
    }

    let a: FixedPoint = coefficients[2];
    let b: FixedPoint = coefficients[1];
    let c: FixedPoint = coefficients[0];

    let delta: FixedPoint = b * b - (a * c * 4);
    println!("Quadratic equation with discriminant ∆ = {:.}^2 - 4 * {:.} * {:.} = {:.}", b.to_f64(), a.to_f64(), c.to_f64(), delta.to_f64());
    let sqrt_delta = match basic::square_root(delta) {
        Some(value) => value,
        None => {
            println!("∆ < 0 => No real solutions.");
            return None
        },
    };
    let x1: FixedPoint = (-b - sqrt_delta) / (a * 2);
    let x2: FixedPoint = (sqrt_delta - b) / (a * 2);

    if delta == ZERO {
        println!("∆ = 0 => One single solution: x0 = {:.} / (2 * {:.}) = {:.}", -b.to_f64(), a.to_f64(), x1.to_f64());
        Some(vec![x1])
    } else {
        dbg!(&x1, &x2);
        println!("∆ > 0 => Two solutions:");
        println!("x1 = ({:.} - √∆) / (2 * {:.}) = {:.}", -b.to_f64(), a.to_f64(), x1.to_f64());
        println!("x2 = ({:.} + √∆) / (2 * {:.}) = {:.}", -b.to_f64(), a.to_f64(), x2.to_f64());
        Some(vec![x1, x2])
    }
}
