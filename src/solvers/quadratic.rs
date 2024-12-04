use crate::math_tools::basic;

pub fn solve_quadratic(coefficients: &[f64]) -> Option<Vec<f64>> {
    if coefficients.len() != 3 || coefficients[2] == 0.0 {
        panic!("Wrong solver used.")
    }

    let a: f64 = coefficients[2];
    let b: f64 = coefficients[1];
    let c: f64 = coefficients[0];

    let delta: f64 = b * b - (a * c * 4.0);
    println!(
        "Quadratic equation with discriminant ∆ = {:.}^2 - 4 * {:.} * {:.} = {:.}",
        b, a, c, delta
    );

    let sqrt_delta: f64 = basic::square_root(delta.abs());

    if delta == 0.0 {
        compute_single_solution(a, b)
    } else if delta > 0.0 {
        compute_two_real_solutions(a, b, c, sqrt_delta)
    } else {
        compute_two_complex_solutions(a, b, sqrt_delta)
    }
}

fn compute_single_solution(a: f64, b: f64) -> Option<Vec<f64>> {
    let x0: f64 = -b / (2.0 * a);
    println!(
        "∆ = 0 => One single solution: x0 = {:.} / (2 * {:.}) = {:.}",
        -b, a, x0
    );
    Some(vec![x0])
}

fn compute_two_real_solutions(a: f64, b: f64, c: f64, sqrt_delta: f64) -> Option<Vec<f64>> {
    let x1: f64 = (-b - sqrt_delta) / (2.0 * a);
    let x2: f64 = (2.0 * c) / (-b - sqrt_delta);
    println!("∆ > 0 => Two real solutions:");
    println!("x1 = ({:.} - √∆) / (2 * {:.}) = {:.}", -b, a, x1);
    println!("x2 = ({:.} + √∆) / (2 * {:.}) = {:.}", -b, a, x2);
    Some(vec![x1, x2])
}

fn compute_two_complex_solutions(a: f64, b: f64, sqrt_delta: f64) -> Option<Vec<f64>> {
    let real_part: f64 = -b / (2.0 * a);
    let imaginary_part: f64 = sqrt_delta / (2.0 * a);
    println!("∆ < 0 => Two complex solutions:");
    println!(
        "x1 = ({:.} - i√-∆) / (2 * {:.}) = {:.} + i * {:.}",
        -b, a, real_part, imaginary_part
    );
    println!(
        "x2 = ({:.} + i√-∆) / (2 * {:.}) = {:.} - i * {:.}",
        -b, a, real_part, imaginary_part
    );
    Some(vec![real_part, imaginary_part])
}
