use crate::math_tools::basic;

pub enum QuadraticSolution {
    NoSolution,
    OneSolution(f64),
    TwoSolutions(f64, f64),
}

pub fn solve_quadratic(a: f64, b: f64, c: f64) -> QuadraticSolution {
    if a == 0.0 {
        panic!("Coefficient 'a' cannot be 0 in a quadratic equation. Use a linear solver.")
    }

    let delta: f64 = b * b - (4.0 * a * c);
    if delta < 0.0 {
        return QuadraticSolution::NoSolution;
    }

    let sqrt_delta: f64 = basic::square_root(delta).unwrap();

    let x1: f64 = (-b - sqrt_delta) / (2.0*a);
    let x2: f64 = (-b + sqrt_delta) / (2.0*a);

    if delta == 0.0 {
        return QuadraticSolution::OneSolution(x1);
    }
    return QuadraticSolution::TwoSolutions(x1, x2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_solution() {
        let solution: QuadraticSolution = solve_quadratic(1.0, 0.0, 1.0);
        assert!(matches!(solution, QuadraticSolution::NoSolution));
    }

    #[test]
    fn test_one_solution() {
        let solution: QuadraticSolution = solve_quadratic(1.0, -2.0, 1.0);
        if let QuadraticSolution::OneSolution(x) = solution {
            assert_eq!(x, 1.0);
        } else {
            panic!("Expected one solution.");
        }
    }

    #[test]
    fn test_two_solutions() {
        let solution: QuadraticSolution = solve_quadratic(1.0, -3.0, 2.0);
        if let  QuadraticSolution::TwoSolutions(x1, x2) = solution {
            assert_eq!(x1, 1.0);
            assert_eq!(x2, 2.0);
        } else {
            panic!("Expected two solutions");
        }
    }

    #[test]
    #[should_panic(expected = "Coefficient 'a' cannot be 0 in a quadratic equation. Use a linear solver.")]
    fn test_panic_on_zero_a() {
        solve_quadratic(0.0, 2.0, 1.0);
    }
}