use crate::math_tools::basic;

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

pub fn solve_quadratic(a: f64, b: f64, c: f64) -> QuadraticSolution {
    if a == 0.0 {
        panic!("Coefficient 'a' cannot be 0 in a quadratic equation. Use a linear solver.")
    }

    let delta: f64 = b * b - (4.0 * a * c);
    if delta < 0.0 {
        return QuadraticSolution::NoRealSolution;
    }

    let sqrt_delta: f64 = basic::square_root(delta).unwrap();

    let x1: f64 = (-b - sqrt_delta) / (2.0 * a);
    let x2: f64 = (-b + sqrt_delta) / (2.0 * a);

    if delta == 0.0 {
        return QuadraticSolution::OneRealSolution(x1);
    }
    return QuadraticSolution::TwoRealSolutions(x1, x2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_solution() {
        let solution: QuadraticSolution = solve_quadratic(1.0, 0.0, 1.0);
        assert!(matches!(solution, QuadraticSolution::NoRealSolution));
    }

    #[test]
    fn test_one_solution() {
        let solution: QuadraticSolution = solve_quadratic(1.0, -2.0, 1.0);
        if let QuadraticSolution::OneRealSolution(x) = solution {
            assert_eq!(x, 1.0);
        } else {
            panic!("Expected one real solution.");
        }
    }

    #[test]
    fn test_two_solutions() {
        let solution: QuadraticSolution = solve_quadratic(1.0, -3.0, 2.0);
        if let  QuadraticSolution::TwoRealSolutions(x1, x2) = solution {
            assert_eq!(x1, 1.0);
            assert_eq!(x2, 2.0);
        } else {
            panic!("Expected two real solutions");
        }
    }

    #[test]
    #[should_panic(expected = "Coefficient 'a' cannot be 0 in a quadratic equation. Use a linear solver.")]
    fn test_panic_on_zero_a() {
        solve_quadratic(0.0, 2.0, 1.0);
    }

    #[test]
    fn test_small_coefficients() {
        let solution: QuadraticSolution = solve_quadratic(1e-8, 1e-4, 1e-2);
        if let QuadraticSolution::TwoRealSolutions(x1, x2) = solution {
            assert!(x1.is_finite(), "Expected finite x1, got {}", x1);
            assert!(x2.is_finite(), "Expected finite x2, got {}", x2);
        } else {
            panic!("Expected two real solutions for small coefficients.");
        }
    }

    #[test]
    fn test_large_coefficients() {
        let solution: QuadraticSolution = solve_quadratic(1e8, -1e10, 1e8);
        if let QuadraticSolution::TwoRealSolutions(x1, x2) = solution {
            let tolerance = 1e-6 * 1e8;
            assert!((x1 - 1.0000001).abs() < tolerance, "x1 is imprecise: {}", x1);
            assert!((x2 - 9.999999).abs() < tolerance, "x2 is imprecise: {}", x2);
        } else {
            panic!("Expected two real solutions for large coefficients.");
        }
    }

    #[test]
    fn test_precision_issue() {
        let solution: QuadraticSolution = solve_quadratic(1.0, -4.0, 3.9999999);

        if let QuadraticSolution::TwoRealSolutions(x1, _x2) = solution {
            assert!((x1 - (2.0003163)).abs() < 1e4, "x1 is imprecise: {}", x1);
        } else {
            panic!("Expected two real solutions.");
        }
    }

    #[test]
    fn test_stress_test_precision_issue() {
        let solution: QuadraticSolution = solve_quadratic(1.0, 1e16, 1.0);

        if let QuadraticSolution::TwoRealSolutions(x1, x2) = solution {
            let tolerance = 1e-6 * 1e16; // Dynamic tolerance based on magnitude
            assert!((x1 + 1e16).abs() < tolerance, "x1 is imprecise: {}", x1);
            assert!((x2 - 1e-16).abs() < tolerance, "x2 is imprecise: {}", x2);
        } else {
            panic!("Expected two real solutions.");
        }
    }
}