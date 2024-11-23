#[cfg(test)]
mod tests {
    use crate::math_tools::basic::square_root;
    use crate::math_tools::fixed_point::FixedPoint;

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

    #[test]
    fn test_add_assign() {
        let mut a = FixedPoint::new(1, 500, 1);
        let b = FixedPoint::new(1, 600, 1);
        a += b;
        assert!(a == FixedPoint::new(3, 100, 1));
    }

    #[test]
    fn test_sub_assign() {
        let mut a = FixedPoint::new(3, 200, 1);
        let b = FixedPoint::new(1, 500, 1);
        a -= b;
        assert!(a == FixedPoint::new(1, 700, 1));
    }

    #[test]
    fn test_mul_assign() {
        let mut a = FixedPoint::new(1, 500, 1);
        let b = FixedPoint::new(2, 0, 1);
        a *= b;
        dbg!(&a);
        assert!(a == FixedPoint::new(3, 0, 1));
    }
}