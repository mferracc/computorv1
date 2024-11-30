#[cfg(test)]
mod tests {
    use crate::constants::math_tools_constants::{NEGATIVE, POSITIVE, ZERO};
    use crate::math_tools::basic::square_root;
    use crate::math_tools::fixed_point::fixed_point::FixedPoint;

    #[test]
    fn test_square_root() {
        assert_eq!(square_root(FixedPoint::new(25, 0, POSITIVE)), Some(FixedPoint::new(5, 0, POSITIVE)));
        assert_eq!(square_root(FixedPoint::new(9, 0, POSITIVE)), Some(FixedPoint::new(3, 0, POSITIVE)));
        assert_eq!(square_root(FixedPoint::new(4, 0, POSITIVE)), Some(FixedPoint::new(2, 0, POSITIVE)));
        assert_eq!(square_root(ZERO), Some(ZERO));
        assert_eq!(square_root(FixedPoint::new(1, 0, NEGATIVE)), None);
        assert_eq!(square_root(FixedPoint::new(100, 0, NEGATIVE)), None);
        assert_eq!(square_root(FixedPoint::new(17, 64, POSITIVE)), Some(FixedPoint::new(4, 2, POSITIVE)));
        assert_eq!(square_root(FixedPoint::new(1, 69, POSITIVE)), Some(FixedPoint::new(1, 3, POSITIVE)));
    }
}