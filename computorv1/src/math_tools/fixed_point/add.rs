use std::ops::{Add, AddAssign};
use crate::math_tools::fixed_point::fixed_point::FixedPoint;

impl Add for FixedPoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result: FixedPoint = self;
        result += rhs;
        result
    }
}

impl AddAssign for FixedPoint {
    fn add_assign(&mut self, rhs: Self) {
        let (self_scaled, rhs_scaled): (FixedPoint, FixedPoint) = Self::scale_to_match(self, &rhs);

        let a: i128 = (self_scaled.integer * self_scaled.scale + self_scaled.decimal) as i128;
        let b: i128 = (rhs_scaled.integer * rhs_scaled.scale + rhs_scaled.decimal) as i128;
        let result: i128 = a * self.sign as i128 + b * rhs.sign as i128;

        self.integer = (result / self_scaled.scale as i128) as i64;
        self.decimal = (result % self_scaled.scale as i128) as i64;
        self.scale = self_scaled.scale;
        self.scale_to_fit();
        self.trim_ending_zeros();
    }
}

impl Add<i64> for FixedPoint {
    type Output = FixedPoint;

    fn add(self, rhs: i64) -> FixedPoint {
        let mut result: FixedPoint = self;
        result += rhs;
        result
    }
}

impl AddAssign<i64> for FixedPoint {
    fn add_assign(&mut self, rhs: i64) {
        let to_add: FixedPoint = Self::new(rhs, 0, rhs/rhs);
        *self += to_add;
        self.trim_ending_zeros();
    }
}
