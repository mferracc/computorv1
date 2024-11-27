#[cfg(test)]
mod tests {
    use crate::constants::math_tools_constants::{NEGATIVE, POSITIVE};
    use crate::math_tools::fixed_point::fixed_point::FixedPoint;

    #[test]
    fn test_add_assign() {
        let mut a: FixedPoint = FixedPoint::new(1, 500, POSITIVE);
        let b: FixedPoint = FixedPoint::new(1, 600, POSITIVE);
        a += b;
        assert!(a == FixedPoint::new(3, 100, POSITIVE));
    }

    #[test]
    fn test_sub_assign() {
        let mut a: FixedPoint = FixedPoint::new(3, 200, POSITIVE);
        let b: FixedPoint = FixedPoint::new(1, 500, POSITIVE);
        a -= b;
        assert!(a == FixedPoint::new(1, 700, POSITIVE));
    }

    #[test]
    fn test_mul_assign() {
        let mut a: FixedPoint = FixedPoint::new(1, 500, POSITIVE);
        let b: FixedPoint = FixedPoint::new(2, 0, POSITIVE);
        a *= b;
        dbg!(&a);
        assert!(a == FixedPoint::new(3, 0, POSITIVE));
    }

    #[test]
    fn test_div_basic() {
        let a: FixedPoint = FixedPoint::new(10, 0, POSITIVE);
        let b: FixedPoint = FixedPoint::new(2, 0, POSITIVE);

        let result: FixedPoint = a / b;

        assert_eq!(result.integer, 5);
        assert_eq!(result.decimal, 0);
        assert_eq!(result.sign, 1);
    }

    #[test]
    fn test_div_with_decimal() {
        let a: FixedPoint = FixedPoint::new(5, 50, POSITIVE);
        let b: FixedPoint = FixedPoint::new(2, 0, POSITIVE);

        let result: FixedPoint = a / b;

        assert_eq!(result.integer, 2);
        assert_eq!(result.decimal, 75);
        assert_eq!(result.sign, 1);
    }

    #[test]
    fn test_div_negative_numbers() {
        let a: FixedPoint = FixedPoint::new(10, 0, NEGATIVE);
        let b: FixedPoint = FixedPoint::new(2, 0, POSITIVE);

        let result: FixedPoint = a / b;

        assert_eq!(result.integer, 5);
        assert_eq!(result.decimal, 0);
        assert_eq!(result.sign, -1);
    }

    #[test]
    fn test_div_negative_divisor() {
        let a: FixedPoint = FixedPoint::new(10, 0, POSITIVE);
        let b: FixedPoint = FixedPoint::new(2, 0, NEGATIVE);

        let result: FixedPoint = a / b;

        assert_eq!(result.integer, 5);
        assert_eq!(result.decimal, 0);
        assert_eq!(result.sign, -1);
    }

    #[test]
    fn test_div_assign() {
        let mut a: FixedPoint = FixedPoint::new(10, 0, POSITIVE);
        let b: FixedPoint = FixedPoint::new(2, 0, POSITIVE);

        a /= b;

        assert_eq!(a.integer, 5);
        assert_eq!(a.decimal, 0);
        assert_eq!(a.sign, 1);
    }

    #[test]
    #[should_panic(expected = "Division by zero!")]
    fn test_div_by_zero() {
        let a: FixedPoint = FixedPoint::new(10, 0, POSITIVE);
        let b: FixedPoint = FixedPoint::new(0, 0, POSITIVE);

        let _ = a / b;
    }

    #[test]
    fn test_div_small_values() {
        let a: FixedPoint = FixedPoint::new(0, 5, POSITIVE);
        let b: FixedPoint = FixedPoint::new(0, 2, POSITIVE);

        let result: FixedPoint = a / b;

        assert_eq!(result.integer, 2);
        assert_eq!(result.decimal, 5);
        assert_eq!(result.sign, 1);
    }

    #[test]
    fn test_div_large_values() {
        let a: FixedPoint = FixedPoint::new(1_000_000, 0, POSITIVE);
        let b: FixedPoint = FixedPoint::new(2, 0, POSITIVE);

        let result: FixedPoint = a / b;

        assert_eq!(result.integer, 500_000);
        assert_eq!(result.decimal, 0);
        assert_eq!(result.sign, 1);
    }

    #[test]
    fn test_div_precision_loss() {
        let a: FixedPoint = FixedPoint::new(1, 0, POSITIVE);
        let b: FixedPoint = FixedPoint::new(3, 0, POSITIVE);

        let result: FixedPoint = a / b;

        assert_eq!(result.integer, 0);
        assert_eq!(result.decimal, 333333333333333);
        assert_eq!(result.sign, 1);
    }

    #[test]
    fn test_div_sign_handling() {
        let a: FixedPoint = FixedPoint::new(10, 0, NEGATIVE);
        let b: FixedPoint = FixedPoint::new(2, 0, NEGATIVE);

        let result: FixedPoint = a / b;

        assert_eq!(result.integer, 5);
        assert_eq!(result.decimal, 0);
        assert_eq!(result.sign, 1);
    }
}
