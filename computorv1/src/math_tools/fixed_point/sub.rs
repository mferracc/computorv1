use std::ops::{Sub, SubAssign};
use crate::math_tools::fixed_point::fixed_point::FixedPoint;

impl Sub for FixedPoint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result: Self = self;
        result -= rhs;
        result
    }
}

impl Sub<&FixedPoint> for &FixedPoint {
    type Output = FixedPoint;

    fn sub(self, rhs: &FixedPoint) -> Self::Output {
        let mut result: FixedPoint = self.clone();
        result -= rhs.clone();
        result
    }
}

impl SubAssign for FixedPoint {
    fn sub_assign(&mut self, other: Self) {
        let other: FixedPoint = FixedPoint::new(other.integer, other.decimal, -other.sign);
        *self += other;
        self.trim_ending_zeros();
    }
}

impl Sub<i64> for FixedPoint {
    type Output = FixedPoint;

    fn sub(self, rhs: i64) -> FixedPoint {
        let mut result: FixedPoint = self;
        result -= rhs;
        result
    }
}

impl SubAssign<i64> for FixedPoint {
    fn sub_assign(&mut self, rhs: i64) {
        let to_add: FixedPoint = Self::new(rhs, 0, rhs/rhs);
        *self -= to_add;
        self.trim_ending_zeros();
    }
}