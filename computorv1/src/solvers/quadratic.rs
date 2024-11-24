use crate::math_tools::basic;
use crate::math_tools::fixed_point::fixed_point::FixedPoint;
use crate::math_tools::polynomial::Polynomial;

pub enum QuadraticSolution {
    NoRealSolution,
    OneRealSolution(f64),
    TwoRealSolutions(f64, f64),
}

// pub fn solve_quadratic(a: f64, b: f64, c: f64) -> QuadraticSolution {
//     if a == 0.0 {
//         panic!("Coefficient 'a' cannot be 0 in a quadratic equation. Use a linear solver.")
//     }
//
//     let delta: f64 = b * b - (4.0 * a * c);
//     if delta < 0.0 {
//         return QuadraticSolution::NoRealSolution;
//     }
//
//     let sqrt_delta: f64 = basic::square_root(delta).unwrap();
//
//     // let x1: f64 = 2.0 * c / (-b + sqrt_delta);
//     // let x2: f64 = 2.0 * c / (-b - sqrt_delta);
//
//     let (x1, x2) = if b >= 0.0 {
//         let x1 = (-b - sqrt_delta) / (2.0 * a);
//         let x2 = (2.0 * c) / (-b - sqrt_delta);
//         (x1, x2)
//     } else {
//         let x1 = (2.0 * c) / (-b + sqrt_delta);
//         let x2 = (-b + sqrt_delta) / (2.0 * a);
//         (x1, x2)
//     };
//
//     if delta == 0.0 {
//         return QuadraticSolution::OneRealSolution(x1);
//     }
//     return QuadraticSolution::TwoRealSolutions(x1, x2);
// }

pub fn solve_quadratic(polynomial: &Polynomial) -> Option<Vec<FixedPoint>> {
    let a: f64 = polynomial.coefficients[0].to_f64();
    let b: f64 = polynomial.coefficients[1].to_f64();
    let c: f64 = polynomial.coefficients[2].to_f64();

    if a == 0.0 {
        panic!("Coefficient 'a' cannot be 0 in a quadratic equation. Use a linear solver.")
    }

    let delta: f64 = b * b - (4.0 * a * c);
    let sqrt_delta: f64 = basic::square_root(delta).unwrap();
    let x1: f64 = (-b - sqrt_delta) / (2.0 * a);
    let x2: f64 = (-b + sqrt_delta) / (2.0 * a);

    let _solution: QuadraticSolution = if delta < 0.0 {
        QuadraticSolution::NoRealSolution
    } else if delta == 0.0 {
        QuadraticSolution::OneRealSolution(x1)
    } else {
        QuadraticSolution::TwoRealSolutions(x1, x2)
    };

    // store the solution in polynomial
    todo!()
}
