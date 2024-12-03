#[cfg(test)]
mod tests {
    use computorv1::math_tools::basic::square_root;

    #[test]
    fn test_square_root() {
        assert_eq!(square_root(25.0), Some(5.0));
        assert_eq!(square_root(9.0), Some(3.0));
        assert_eq!(square_root(4.0), Some(2.0));
        assert_eq!(square_root(0.0), Some(0.0));
        assert_eq!(square_root(-1.0), None);
        assert_eq!(square_root(-100.0), None);
        assert_eq!(square_root(1e10), Some(100000.0));
        assert_eq!(square_root(1e20), Some(1e10));
    }
}