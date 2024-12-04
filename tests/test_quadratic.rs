#![allow(clippy::clone_on_copy)]

#[cfg(test)]
mod tests {
    use computorv1::solvers::quadratic::solve_quadratic;

    #[test]
    fn test_no_real_solutions() {
        let polynomials: Vec<Vec<f64>> = vec![
            vec![1.0, 0.0, 1.0],
            vec![4.0, 0.0, 2.0],
            vec![5.0, 0.0, 3.0],
            vec![2.0, 0.0, 1.0],
            vec![9.0, 0.0, 4.0],
            vec![5.0, 4.0, 1.0],
            vec![7.0, -3.0, 2.0],
            vec![1.0, 1.0, 1.0],
            vec![8.0, -2.0, 5.0],
            vec![4.0, 1.0, 3.0],
            vec![10.0, 6.0, 1.0],
            vec![9.0, 5.0, 4.0],
            vec![2.0, -1.0, 1.0],
            vec![3.0, 2.0, 6.0],
            vec![3.0, 2.0, 1.0],
        ];

        for poly in polynomials.into_iter() {
            let solutions: Option<Vec<f64>> = solve_quadratic(&poly);

            if let Some(solutions) = solutions {
                assert_eq!(solutions.len(), 2);
                assert!(solutions[1] > 0.0)
            } else {
                panic!("Expected complex solutions, but got None.");
            }
        }
    }

    #[test]
    fn test_one_solution() {
        let polynomials: Vec<Vec<f64>> = vec![
            vec![1.0, -2.0, 1.0],
            vec![4.0, 4.0, 1.0],
            vec![9.0, -6.0, 1.0],
            vec![1.0, -4.0, 4.0],
            vec![4.0, -12.0, 9.0],
            vec![25.0, -10.0, 1.0],
            vec![1.0, -8.0, 16.0],
            vec![4.0, -20.0, 25.0],
            vec![1.0, 12.0, 36.0],
        ];

        let expected: Vec<Vec<f64>> = vec![
            vec![1.0],
            vec![-2.0],
            vec![3.0],
            vec![0.5],
            vec![0.6666666666666666],
            vec![5.0],
            vec![0.25],
            vec![0.4],
            vec![-0.16666666666666666],
        ];

        for (poly, exp) in polynomials.into_iter().zip(expected.into_iter()) {
            let solutions: Option<Vec<f64>> = solve_quadratic(&poly);

            if let Some(sols) = solutions {
                assert_eq!(sols.len(), exp.len(), "Expected one solution.");
                for expected_sol in &exp {
                    assert!(
                        sols.contains(expected_sol),
                        "Expected solution: {} (got {})",
                        expected_sol,
                        sols[0]
                    );
                }
            } else {
                panic!("Expected a solution, but got None.");
            }
        }
    }

    #[test]
    fn test_two_solutions() {
        let polynomials: Vec<Vec<f64>> = vec![
            vec![4.0, -5.0, 1.0],
            vec![6.0, -5.0, 1.0],
            vec![-4.0, 3.0, 1.0],
            vec![3.0, -7.0, 2.0],
            vec![-8.0, 2.0, 3.0],
            vec![-10.0, -3.0, 1.0],
            vec![5.0, 6.0, 1.0],
            vec![-9.0, -4.0, 5.0],
            vec![6.0, -11.0, 3.0],
        ];

        let expected: Vec<Vec<f64>> = vec![
            vec![1.0, 4.0],
            vec![2.0, 3.0],
            vec![1.0, -4.0],
            vec![0.5, 3.0],
            vec![1.3333333333333333, -2.0],
            vec![-2.0, 5.0],
            vec![-5.0, -1.0],
            vec![1.8, -1.0],
            vec![0.6666666666666666, 3.0],
        ];

        for (poly, exp) in polynomials.into_iter().zip(expected.into_iter()) {
            let solutions: Option<Vec<f64>> = solve_quadratic(&poly);

            if let Some(sols) = solutions {
                assert_eq!(sols.len(), exp.len(), "Expected two real solutions.");
                for expected_sol in &exp {
                    assert!(
                        sols.contains(expected_sol),
                        "Expected solutions: {}, but got {:?}.",
                        expected_sol,
                        sols,
                    );
                }
            } else {
                panic!("Expected two real solutions, but got None.");
            }
        }
    }

    #[test]
    #[should_panic(expected = "Wrong solver used.")]
    fn test_panic_on_zero_a() {
        solve_quadratic(&[1.0, 2.0, 0.0]);
    }
}
