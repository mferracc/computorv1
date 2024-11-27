#[cfg(test)]
mod tests {
    use crate::constants::math_tools_constants::{NEGATIVE, POSITIVE};
    use crate::math_tools::fixed_point::fixed_point::FixedPoint;
    use crate::math_tools::polynomial::Polynomial;
    use crate::solvers::linear::solve_linear;

    #[test]
    fn test_simple_solution() {
        let polynomial: Polynomial = Polynomial::new("2*X + 4").unwrap();
        let solution: Vec<FixedPoint> = vec!(FixedPoint::new(2, 0, NEGATIVE));
        assert_eq!(solve_linear(&polynomial.coefficients), Some(solution));
    }

    #[test]
    fn test_negative_coefficient_a() {
        let polynomial: Polynomial = Polynomial::new("-3*X + 9").unwrap();
        let solution: Vec<FixedPoint> = vec!(FixedPoint::new(3, 0, POSITIVE));
        assert_eq!(solve_linear(&polynomial.coefficients), Some(solution));
    }

    #[test]
    fn test_negative_coefficient_b() {
        let polynomial: Polynomial = Polynomial::new("4*X - 8").unwrap();
        let solution: Vec<FixedPoint> = vec!(FixedPoint::new(2, 0, POSITIVE));
        assert_eq!(solve_linear(&polynomial.coefficients), Some(solution));
    }

    #[test]
    fn test_no_solution() {
        let polynomial: Polynomial = Polynomial::new("0*X + 4").unwrap();
        assert_eq!(solve_linear(&polynomial.coefficients), None);
    }

    #[test]
    fn test_zero_both_coefficient() {
        let polynomial: Polynomial = Polynomial::new("0*X + 0").unwrap();
        assert_eq!(solve_linear(&polynomial.coefficients), None);
    }
}