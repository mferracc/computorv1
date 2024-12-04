#[cfg(test)]
mod tests {
    use computorv1::math_tools::basic::square_root;

    #[test]
    fn test_square_root_positive_values() {
        assert_eq!(square_root(25.0), 5.0);
        assert_eq!(square_root(9.0), 3.0);
        assert_eq!(square_root(4.0), 2.0);
        assert_eq!(square_root(0.0), 0.0);
        assert_eq!(square_root(1e10), 100000.0);
        assert_eq!(square_root(1e20), 1e10);
    }

    #[should_panic]
    #[test]
    fn test_square_root_negative_value() {
        square_root(-1.0);
    }
}