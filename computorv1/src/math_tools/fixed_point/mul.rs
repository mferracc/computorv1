use std::ops::{Mul, MulAssign};
use crate::math_tools::fixed_point::fixed_point::FixedPoint;

impl Mul for FixedPoint {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result: Self = self;
        result *= rhs;
        result
    }
}

impl MulAssign for FixedPoint {
    fn mul_assign(&mut self, rhs: Self) {
        let (self_scaled, rhs_scaled): (FixedPoint, FixedPoint) = Self::scale_to_match(self, &rhs);

        let product_total: i64 = match self.calculate_product_total(rhs_scaled) {
            Ok(total) => total,
            Err(e) => panic!("{}", e)
        };

        let result_scale: i64 = self_scaled.scale;
        self.integer = product_total / result_scale;
        self.decimal = product_total % result_scale;
        self.sign *= rhs.sign;
        self.scale_to_fit();
        self.trim_ending_zeros();
    }
}

impl Mul<i64> for FixedPoint {
    type Output = FixedPoint;

    fn mul(self, rhs: i64) -> FixedPoint {
        let mut result: FixedPoint = self;
        result *= rhs;
        result
    }
}

impl MulAssign<i64> for FixedPoint {
    fn mul_assign(&mut self, rhs: i64) {
        let to_add: FixedPoint = Self::new(rhs, 0, rhs/rhs);
        *self *= to_add;
        self.trim_ending_zeros();
    }
}

