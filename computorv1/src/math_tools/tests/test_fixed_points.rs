#[cfg(test)]
mod tests {
    use crate::math_tools::fixed_point::fixed_point::FixedPoint;

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