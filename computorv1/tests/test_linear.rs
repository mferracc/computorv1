#[cfg(test)]
mod tests {
    use computorv1::solvers::linear::solve_linear;

    #[test]
    fn test_simple_solution() {
        assert_eq!(solve_linear(&[2.0, 4.0]), Some(vec![-0.5]));
    }

    #[test]
    fn test_negative_coefficient_a() {
        assert_eq!(solve_linear(&[-3.0, 9.0]), Some(vec![0.3333333333333333]));
    }

    #[test]
    fn test_negative_coefficient_b() {
        assert_eq!(solve_linear(&[4.0, -8.0]), Some(vec![0.5]));
    }

    #[test]
    fn test_no_solution() {
        assert_eq!(solve_linear(&[0.0, 4.0]), Some(vec![0.0]));
    }

    #[test]
    fn test_zero_both_coefficient() {
        assert_eq!(solve_linear(&[0.0, 0.0]), None);
    }

    #[test]
    fn test_edge_case_small_value() {
    assert_eq!(solve_linear(&[1e-10, 1e-10]), Some(vec![-1.0]));
    }
}