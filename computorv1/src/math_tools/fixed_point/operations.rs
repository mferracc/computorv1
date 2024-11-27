use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use crate::constants::math_tools_constants::MAX_SCALE;
use crate::math_tools::fixed_point::fixed_point::FixedPoint;

impl Add for FixedPoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self;
        result += rhs;
        result
    }
}

impl AddAssign for FixedPoint {
    fn add_assign(&mut self, rhs: Self) {
        let (self_scaled, rhs_scaled) = Self::scale_to_match(self, &rhs);

        let a: i128 = (self_scaled.integer * self_scaled.scale + self_scaled.decimal) as i128;
        let b: i128 = (rhs_scaled.integer * rhs_scaled.scale + rhs_scaled.decimal) as i128;
        let result: i128 = a * self.sign as i128 + b * rhs.sign as i128;

        self.integer = (result / self_scaled.scale as i128) as i64;
        self.decimal = (result % self_scaled.scale as i128) as i64;
        self.scale = self_scaled.scale;
        dbg!(&self, self.scale);
        self.scale_to_fit();
        dbg!(&self, self.scale);
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

impl SubAssign for FixedPoint {
    fn sub_assign(&mut self, other: Self) {
        let other = FixedPoint::new(other.integer, other.decimal, -other.sign);
        *self += other;
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

impl MulAssign for FixedPoint {
    fn mul_assign(&mut self, rhs: Self) {
        let (self_scaled, rhs_scaled) = Self::scale_to_match(self, &rhs);

        let product_total: i64 = self_scaled.integer * rhs_scaled.scale * rhs_scaled.integer
            + self_scaled.integer * rhs_scaled.decimal
            + self_scaled.decimal * rhs_scaled.integer
            + (self_scaled.decimal * rhs_scaled.decimal) / self_scaled.scale;

        let result_scale = self_scaled.scale;
        self.integer = product_total / result_scale;
        self.decimal = product_total % result_scale;
        self.sign *= rhs.sign;
        self.scale_to_fit();
    }
}

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
        let mut remainder: i128 = numerator % denominator;
        let mut new_scale: i64 = MAX_SCALE;
        let mut new_decimal: i64 = 0;

        while remainder != 0 && new_scale > 1 {
            remainder *= 10;
            new_decimal = new_decimal * 10 + (remainder / denominator) as i64;
            remainder %= denominator;
            new_scale /= 10;
        }

        self.integer = result_integer.abs() as i64;
        self.decimal = new_decimal;
        self.sign = if numerator.signum() == denominator.signum() { 1 } else { -1 };
        self.scale = new_scale;
    }
}

impl Neg for FixedPoint {
    type Output = FixedPoint;

    fn neg(self) -> FixedPoint {
        -&self
    }
}


impl Neg for &FixedPoint {
    type Output = FixedPoint;

    fn neg(self) -> FixedPoint {
        FixedPoint {
            integer: self.integer,
            decimal: self.decimal,
            sign: -self.sign,
            scale: self.scale,
        }
    }
}
