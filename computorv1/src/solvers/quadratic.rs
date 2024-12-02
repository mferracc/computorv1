use crate::math_tools::basic;

pub fn solve_quadratic(coefficients: &[f64]) -> Option<Vec<f64>> {
    if coefficients.len() != 3 || coefficients[2] == 0.0 {
        panic!("Wrong solver used.")
    }

    let a: f64 = coefficients[2];
    let b: f64 = coefficients[1];
    let c: f64 = coefficients[0];

    let delta: f64 = b * b - (a * c * 4.0);
    println!("Quadratic equation with discriminant ∆ = {:.}^2 - 4 * {:.} * {:.} = {:.}", b, a, c, delta);
    let sqrt_delta = match basic::square_root(delta) {
        Some(value) => value,
        None => {
            println!("∆ < 0 => No real solutions.");
            return None
        },
    };
    let x1: f64 = (-b - sqrt_delta) / (2.0 * a);
    let x2: f64 = (2.0 * c) / (-b - sqrt_delta);

    if delta == 0.0 {
        println!("∆ = 0 => One single solution: x0 = {:.} / (2 * {:.}) = {:.}", -b, a, x1);
        Some(vec![x1])
    } else {
        dbg!(&x1, &x2);
        println!("∆ > 0 => Two solutions:");
        println!("x1 = ({:.} - √∆) / (2 * {:.}) = {:.}", -b, a, x1);
        println!("x2 = ({:.} + √∆) / (2 * {:.}) = {:.}", -b, a, x2);
        Some(vec![x1, x2])
    }
}
