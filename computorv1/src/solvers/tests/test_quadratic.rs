#[cfg(test)]
mod tests {
    use std::vec;
    use crate::constants::math_tools_constants::{NEGATIVE, POSITIVE};
    use crate::math_tools::fixed_point::fixed_point::FixedPoint;
    use crate::math_tools::polynomial::Polynomial;
    use crate::solvers::quadratic::{solve_quadratic};

    #[test]
    fn test_no_solution() {
        let polynomials: Vec<Polynomial> = vec![
            Polynomial::new("X^2 + 1").unwrap(),
            Polynomial::new("2*X^2 + 4").unwrap(),
            Polynomial::new("3*X^2 + 5").unwrap(),
            Polynomial::new("X^2 + 2").unwrap(),
            Polynomial::new("4*X^2 + 9").unwrap(),
            Polynomial::new("X^2 + 4*X + 5").unwrap(),
            Polynomial::new("2*X^2 - 3*X + 7").unwrap(),
            Polynomial::new("X^2 + X + 1").unwrap(),
            Polynomial::new("5*X^2 - 2*X + 8").unwrap(),
            Polynomial::new("3*X^2 + X + 4").unwrap(),
            Polynomial::new("X^2 + 6*X + 10").unwrap(),
            Polynomial::new("4*X^2 + 5*X + 9").unwrap(),
            Polynomial::new("X^2 - X + 2").unwrap(),
            Polynomial::new("6*X^2 + 2*X + 3").unwrap(),
            Polynomial::new("X^2 + 2*X + 3").unwrap(),

        ];

        for poly in polynomials {
            let solutions: Option<Vec<FixedPoint>> = solve_quadratic(&poly.coefficients);
            assert!(matches!(solutions, None));
        }
    }

    #[test]
    fn test_one_solution() {
        let polynomials: Vec<Polynomial> = vec![
            Polynomial::new("X^2 - 2*X + 1").unwrap(),
            Polynomial::new("X^2 + 4*X + 4").unwrap(),
            Polynomial::new("X^2 - 6*X + 9").unwrap(),
            Polynomial::new("4*X^2 - 4*X + 1").unwrap(),
            Polynomial::new("9*X^2 - 12*X + 4").unwrap(),
            Polynomial::new("X^2 - 10*X + 25").unwrap(),
            Polynomial::new("16*X^2 - 8*X + 1").unwrap(),
            Polynomial::new("25*X^2 - 20*X + 4").unwrap(),
            Polynomial::new("36*X^2 + 12*X + 1").unwrap(),
        ];

        let expected: Vec<Vec<FixedPoint>> = vec![
            vec![FixedPoint::new(1, 0, POSITIVE)],
            vec![FixedPoint::new(-2, 0, POSITIVE)],
            vec![FixedPoint::new(3, 0, POSITIVE)],
            vec![FixedPoint::new(0, 5, NEGATIVE)],
            vec![FixedPoint::new(0, 666666666666666, POSITIVE)],
            vec![FixedPoint::new(5, 0, POSITIVE)],
            vec![FixedPoint::new(0, 25, POSITIVE)],
            vec![FixedPoint::new(0, 4, POSITIVE)],
            vec![FixedPoint::new(0, 166666666666666, NEGATIVE)],
        ];

        for (poly, exp) in polynomials.into_iter().zip(expected.into_iter()) {
            let solutions: Option<Vec<FixedPoint>> = solve_quadratic(&poly.coefficients);

            if let Some(sols) = solutions {
                assert_eq!(sols.len(), exp.len(), "Expected one solution.");
                for expected_sol in &exp {
                    assert!(sols.contains(expected_sol), "Expected solution: {} (got {})", expected_sol, sols[0].to_f64());
                }
            } else {
                panic!("Expected a solution, but got None.");
            }
        }
    }

    #[test]
    fn test_two_solutions() {
        let polynomials: Vec<Polynomial> = vec![
            Polynomial::new("X^2 - 5*X + 4").unwrap(),
            Polynomial::new("X^2 - 5*X + 6").unwrap(),
            Polynomial::new("X^2 + 3*X - 4").unwrap(),
            Polynomial::new("2*X^2 - 7*X + 3").unwrap(),
            Polynomial::new("X^2 - 4*X - 12").unwrap(),
            Polynomial::new("3*X^2 + 2*X - 8").unwrap(),
            Polynomial::new("X^2 - 3*X - 10").unwrap(),
            Polynomial::new("X^2 + 6*X + 5").unwrap(),
            Polynomial::new("4*X^2 - 4*X - 8").unwrap(),
            Polynomial::new("5*X^2 - 4*X - 9").unwrap(),
            Polynomial::new("3*X^2 - 11*X + 6").unwrap(),
        ];

        let expected: Vec<Vec<FixedPoint>> = vec![
            vec![FixedPoint::new(1, 0, POSITIVE), FixedPoint::new(4, 0, POSITIVE)],
            vec![FixedPoint::new(2, 0, POSITIVE), FixedPoint::new(3, 0, POSITIVE)],
            vec![FixedPoint::new(1, 0, NEGATIVE), FixedPoint::new(4, 0, POSITIVE)],
            vec![FixedPoint::new(0, 5, POSITIVE), FixedPoint::new(3, 0, POSITIVE)],
            vec![FixedPoint::new(6, 0, POSITIVE), FixedPoint::new(2, 0, POSITIVE)],
            vec![FixedPoint::new(2, 0, NEGATIVE), FixedPoint::new(1, 333333333333333, POSITIVE)],
            vec![FixedPoint::new(5, 0, POSITIVE), FixedPoint::new(2, 0, POSITIVE)],
            vec![FixedPoint::new(1, 0, NEGATIVE), FixedPoint::new(5, 0, POSITIVE)],
            vec![FixedPoint::new(1, 0, NEGATIVE), FixedPoint::new(2, 0, POSITIVE)],
            vec![FixedPoint::new(1, 8, NEGATIVE), FixedPoint::new(1, 0, POSITIVE)],
            vec![FixedPoint::new(0, 666666666666666, NEGATIVE), FixedPoint::new(3, 0, POSITIVE)],
        ];

        for (poly, exp) in polynomials.into_iter().zip(expected.into_iter()) {
            let solutions: Option<Vec<FixedPoint>> = solve_quadratic(&poly.coefficients);

            if let Some(sols) = solutions {
                assert_eq!(sols.len(), exp.len(), "Expected two solutions.");
                for expected_sol in &exp {
                    assert!(
                        sols.contains(expected_sol),
                        "Expected solutions: {}, but got {:?}.",
                        expected_sol,
                        sols,
                    );
                }
            } else {
                panic!("Expected two solutions, but got None.");
            }
        }
    }

    #[test]
    #[should_panic(
        expected = "Wrong solver used."
    )]
    fn test_panic_on_zero_a() {
        let polynomial: Polynomial = Polynomial::new("2*X + 1").unwrap();
        solve_quadratic(&polynomial.coefficients);
    }

    #[test]
    #[should_panic(
        expected = "Multiplication overflow"
    )]
    fn test_large_coefficients() {
        let polynomial: Polynomial = Polynomial::new("100000000*X^2 - 10000000000*X + 100000000").unwrap();
        let _solutions: Option<Vec<FixedPoint>> = solve_quadratic(&polynomial.coefficients);
    }
}