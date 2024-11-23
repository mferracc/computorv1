use std::ops::{Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign};
use std::cmp::PartialEq;
use std::fmt;
use std::fmt::Formatter;
use crate::constants::{MAX_SCALE, NEGATIVE};

pub struct FixedPoint {
    pub integer: i64,
    pub decimal: i64,
    pub sign: i64,
    pub scale: i64,
}

impl FixedPoint {
    pub fn new(integer: i64, decimal: i64, sign: i64) -> Self {
        let mut scale: i64 = 1;
        let mut temp_decimal: i64 = decimal;

        while temp_decimal != 0 && scale < MAX_SCALE {
            temp_decimal /= 10;
            scale *= 10;
        }

        Self::new_with_scale(integer, decimal, sign, scale)
    }

    pub fn new_with_scale(integer: i64, decimal: i64, sign: i64, scale: i64) -> Self {
        FixedPoint {
            integer,
            decimal,
            sign,
            scale,
        }
    }

    pub fn to_f64(&self) -> f64 {
        let total = self.sign * (self.integer * self.scale + self.decimal);
        total as f64 / self.scale as f64
    }

    /// Private part

    fn scale_to_match(a: &FixedPoint, b: &FixedPoint) -> (FixedPoint, FixedPoint) {
        if a.scale == b.scale {
            (a.clone(), b.clone())
        } else if a.scale < b.scale {
            let scale_factor = b.scale / a.scale;
            (
                FixedPoint {
                    integer: a.integer,
                    decimal: a.decimal * scale_factor,
                    sign: a.sign,
                    scale: b.scale,
                },
                b.clone()
            )
        } else {
            let scale_factor = a.scale / b.scale;
            (
                a.clone(),
                FixedPoint {
                    integer: b.integer,
                    decimal: b.decimal * scale_factor,
                    sign: b.sign,
                    scale: a.scale,
                }
            )
        }
    }

    fn scale_to_fit(&mut self) {
        if self.decimal < 0 || self.integer < 0 {
            self.sign = NEGATIVE;
        }
        self.integer = self.integer.abs();
        self.decimal = self.decimal.abs();
        while self.decimal % 10 == 0 && self.decimal != 0 {
            self.decimal /= 10;
            self.scale /= 10;
        }
    }
}

impl Clone for FixedPoint {
    fn clone(&self) -> Self {
        FixedPoint {
            integer: self.integer,
            decimal: self.decimal,
            sign: self.sign,
            scale: self.scale,
        }
    }
}

impl Add for FixedPoint {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let mut result = self.clone();
        result += rhs;
        result
    }
}

impl Sub for FixedPoint {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let mut result: Self = self.clone();
        result -= rhs;
        result
    }
}

impl Mul for FixedPoint {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut result: Self = self.clone();
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
        let (self_scaled, other_scaled) = Self::scale_to_match(self, &other);

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

impl PartialEq for FixedPoint {
    fn eq(&self, other: &Self) -> bool {
        let (a, b): (Self, Self) = Self::scale_to_match(self, other);

        a.integer == b.integer && a.decimal == b.decimal
    }
}

impl fmt::Debug for FixedPoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let decimal_places: usize = self.scale.to_string().len() - 1;
        let decimal_str = format!("{:0width$}", self.decimal, width = decimal_places);

        write!(
            f,
            "FixedPoint {{ {}.{} }}",
            self.sign * self.integer,
            decimal_str,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
