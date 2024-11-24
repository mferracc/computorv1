#[cfg(test)]
mod tests {
    use crate::constants::math_tools_constants::{NEGATIVE, POSITIVE};
    use crate::math_tools::fixed_point::fixed_point::FixedPoint;
    use crate::math_tools::polynomial::Polynomial;
    use crate::solvers::quadratic::{solve_quadratic};

    #[test]
    fn test_no_solution() {
        let polynomial: Polynomial = Polynomial::new("X^2 + 1").unwrap();
        let solutions: Option<Vec<FixedPoint>> = solve_quadratic(&polynomial);

        assert!(matches!(solutions, None));
    }

    #[test]
    fn test_one_solution() {
        let polynomial: Polynomial = Polynomial::new("-X^2 - 2*X + 1").unwrap();
        let solutions: Option<Vec<FixedPoint>> = solve_quadratic(&polynomial);

        let expected: Vec<FixedPoint> = vec![FixedPoint::new(1, 0, POSITIVE)];

        if let Some(sols) = solutions {
            assert_eq!(sols.len(), expected.len(), "Expected one solution.");
            for expected_sol in &expected {
                assert!(sols.contains(expected_sol), "Expected solution: {}", expected_sol);
            }
        } else {
            panic!("Expected a solution, but got None.");
        }
    }

    #[test]
    fn test_two_solutions() {
        let polynomial: Polynomial = Polynomial::new("-X^2 + 3*X + 2").unwrap();
        let solutions: Option<Vec<FixedPoint>> = solve_quadratic(&polynomial);

        let expected: Vec<FixedPoint> = vec![
            FixedPoint::new(1, 0, POSITIVE),
            FixedPoint::new(2, 0, POSITIVE),
        ];

        if let Some(sols) = solutions {
            assert_eq!(sols.len(), expected.len(), "Expected two solutions.");

            for expected_sol in &expected {
                assert!(sols.contains(expected_sol), "Expected solution: {}", expected_sol);
            }
        } else {
            panic!("Expected two solutions, but got None.");
        }
    }

    #[test]
    #[should_panic(
        expected = "Coefficient 'a' cannot be 0 in a quadratic equation. Use a linear solver."
    )]
    fn test_panic_on_zero_a() {
        let polynomial: Polynomial = Polynomial::new("2*X + 1").unwrap();
        solve_quadratic(&polynomial);
    }

    #[test]
    fn test_small_coefficients() {
        let polynomial: Polynomial = Polynomial::new("1e-8*X^2 + 1e-4*X + 1e-2").unwrap();
        let solutions: Option<Vec<FixedPoint>> = solve_quadratic(&polynomial);

        if let Some(ref sol) = solutions {
            if sol.len() == 2 {
                assert!(sol[0].is_finite(), "Expected finite x1, got {}", sol[0]);
                assert!(sol[1].is_finite(), "Expected finite x2, got {}", sol[1]);
            } else {
                panic!("Expected two real solutions for small coefficients.");
            }
        } else {
            panic!("Expected some solutions, but got None.");
        }
    }

    #[test]
    fn test_large_coefficients() {
        let polynomial: Polynomial = Polynomial::new("1e8*X^2 - 1e10*X + 1e8").unwrap();
        let solutions: Option<Vec<FixedPoint>> = solve_quadratic(&polynomial);

        if let Some(solutions) = solutions {
            assert_eq!(solutions.len(), 2, "Expected two solutions.");
            let tolerance: FixedPoint = FixedPoint::new_with_scale(1, 0, 1, 6);
            let expected_x1: FixedPoint = FixedPoint::new_with_scale(10000001, 0, 1, 0);
            let expected_x2: FixedPoint = FixedPoint::new_with_scale(9999999, 0, 1, 0);

            assert!(
                solutions[0].approx_eq(&expected_x1, &tolerance),
                "x1 is imprecise: {}",
                solutions[0]
            );
            assert!(
                solutions[1].approx_eq(&expected_x2, &tolerance),
                "x2 is imprecise: {}",
                solutions[1]
            );
        } else {
            panic!("Expected two real solutions for large coefficients.");
        }
    }

    #[test]
    fn test_precision_issue() {
        let polynomial: Polynomial = Polynomial::new("X^2 - 4*X + 3.9999999").unwrap();
        let solutions: Option<Vec<FixedPoint>> = solve_quadratic(&polynomial);

        if let Some(solutions) = solutions {
            assert_eq!(solutions.len(), 2, "Expected two solutions.");
            let expected_x1 = FixedPoint::new_with_scale(20003163, 0, 1, 4); // 2.0003163
            let tolerance = FixedPoint::new_with_scale(1, 0, 1, 4); // 1e-4 tolerance

            assert!(
                solutions[0].approx_eq(&expected_x1, &tolerance),
                "x1 is imprecise: {}",
                solutions[0]
            );
        } else {
            panic!("Expected two real solutions.");
        }
    }

    #[test]
    fn test_stress_test_precision_issue() {
        let polynomial: Polynomial = Polynomial::new("X^2 + 1e16*X + 1").unwrap();
        let solutions: Option<Vec<FixedPoint>> = solve_quadratic(&polynomial);

        if let Some(solutions) = solutions {
            assert_eq!(solutions.len(), 2, "Expected two solutions.");
            let tolerance: FixedPoint = FixedPoint::new_with_scale(1, 0, 1, 6);
            let expected_x1: FixedPoint = FixedPoint::new_with_scale(-1e16 as i64, 0, NEGATIVE, 0);
            let expected_x2: FixedPoint = FixedPoint::new_with_scale(1, 0, 1, 16);

            assert!(
                solutions[0].approx_eq(&expected_x1, &tolerance),
                "x1 is imprecise: {}",
                solutions[0]
            );
            assert!(
                solutions[1].approx_eq(&expected_x2, &tolerance),
                "x2 is imprecise: {}",
                solutions[1]
            );
        } else {
            panic!("Expected two real solutions.");
        }
    }
}