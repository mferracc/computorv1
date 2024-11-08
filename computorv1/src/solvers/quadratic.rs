pub fn solve_quadratic(a: f64, b: f64, c: f64) -> Option<f64> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use crate::solve_linear;
    use super::*;

    #[test]
    fn test_simple_solution() {
        assert_eq!(solve_quadratic(1.0, 2.0, 3.0), Some(54.4))
    }
}