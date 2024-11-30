use std::ops::{Div, DivAssign};
use crate::constants::math_tools_constants::MAX_SCALE;
use crate::math_tools::fixed_point::fixed_point::FixedPoint;

impl Div for FixedPoint {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        let mut result: Self = self;
        result /= rhs;
        result
    }
}

impl DivAssign for FixedPoint {
    fn div_assign(&mut self, rhs: FixedPoint) {
        let (a, b): (FixedPoint, FixedPoint) = Self::scale_to_match(self, &rhs);
        let numerator: i128 = a.to_raw_value();
        let denominator: i128 = b.to_raw_value();

        if denominator == 0 {
            panic!("Division by zero!");
        }

        let result_integer: i128 = numerator / denominator;
        let mut remainder: i128 = (numerator % denominator).abs();
        let mut new_scale: i64 = 1;
        let mut new_decimal: i64 = 0;

        while remainder != 0 && new_scale < MAX_SCALE {
            remainder *= 10;
            new_decimal = new_decimal * 10 + (remainder / denominator) as i64;
            remainder %= denominator;
            new_scale *= 10;
        }

        self.integer = result_integer.abs() as i64;
        self.decimal = new_decimal;
        self.sign = if numerator.signum() == denominator.signum() { 1 } else { -1 };
        self.scale = new_scale;
        self.trim_ending_zeros();
    }
}

impl Div<i64> for FixedPoint {
    type Output = FixedPoint;

    fn div(self, rhs: i64) -> FixedPoint {
        let mut result: FixedPoint = self;
        result /= rhs;
        result
    }
}

impl DivAssign<i64> for FixedPoint {
    fn div_assign(&mut self, rhs: i64) {
        if rhs == 0 {
            panic!("Division by zero!");
        }

        let to_add: FixedPoint = Self::new(rhs, 0, rhs/rhs);
        *self /= to_add;
        self.trim_ending_zeros();
    }
}
