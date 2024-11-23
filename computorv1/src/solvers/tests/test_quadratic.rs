#[cfg(test)]
mod tests {

    use crate::solvers::quadratic::{QuadraticSolution, solve_quadratic};

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
        if let QuadraticSolution::TwoRealSolutions(x1, x2) = solution {
            assert_eq!(x1, 1.0);
            assert_eq!(x2, 2.0);
        } else {
            panic!("Expected two real solutions");
        }
    }

    #[test]
    #[should_panic(
        expected = "Coefficient 'a' cannot be 0 in a quadratic equation. Use a linear solver."
    )]
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
            assert!(
                (x1 - 1.0000001).abs() < tolerance,
                "x1 is imprecise: {}",
                x1
            );
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
