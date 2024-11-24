use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};
use crate::math_tools::fixed_point::fixed_point::FixedPoint;

impl Add for FixedPoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result += rhs;
        result
    }
}

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

impl Mul for FixedPoint {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result: Self = self;
        result *= rhs;
        result
    }
}

impl AddAssign for FixedPoint {
    fn add_assign(&mut self, other: Self) {
        let (self_scaled, other_scaled) = Self::scale_to_match(self, &other);

        let a: i128 = (self_scaled.integer * self_scaled.scale + self_scaled.decimal) as i128;
        let b: i128 = (other_scaled.integer * other_scaled.scale + other_scaled.decimal) as i128;
        let result: i128 = a * self.sign as i128 + b * other.sign as i128;

        self.integer = (result / self_scaled.scale as i128) as i64;
        self.decimal = (result % self_scaled.scale as i128) as i64;
        self.scale = self_scaled.scale;
        dbg!(&self, self.scale);
        self.scale_to_fit();
        dbg!(&self, self.scale);
    }
}

impl SubAssign for FixedPoint {
    fn sub_assign(&mut self, other: Self) {
        let other = FixedPoint::new(other.integer, other.decimal, -other.sign);
        *self += other;
    }
}

impl MulAssign for FixedPoint {
    fn mul_assign(&mut self, other: Self) {
        let (self_scaled, other_scaled) = Self::scale_to_match(self, &other);

        let product_total: i64 = self_scaled.integer * other_scaled.scale * other_scaled.integer
            + self_scaled.integer * other_scaled.decimal
            + self_scaled.decimal * other_scaled.integer
            + (self_scaled.decimal * other_scaled.decimal) / self_scaled.scale;

        let result_scale = self_scaled.scale;
        self.integer = product_total / result_scale;
        self.decimal = product_total % result_scale;
        self.sign *= other.sign;
        self.scale_to_fit();
    }
}
