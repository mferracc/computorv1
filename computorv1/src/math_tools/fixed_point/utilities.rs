use std::cmp::Ordering;
use crate::constants::math_tools_constants::{MULTIPLICATION_OVERFLOW, NEGATIVE, POSITIVE};
use crate::math_tools::fixed_point::fixed_point::FixedPoint;

impl FixedPoint {
    pub fn abs(&self) -> FixedPoint {
        if self.sign == NEGATIVE {
            self.negate()
        } else {
            self.clone()
        }
    }

    pub fn negate(&self) -> FixedPoint {
        FixedPoint {
            sign: if self.sign == POSITIVE { NEGATIVE } else { POSITIVE },
            ..*self
        }
    }

    pub fn scale_to_match(a: &FixedPoint, b: &FixedPoint) -> (FixedPoint, FixedPoint) {
        match a.scale.cmp(&b.scale) {
            Ordering::Equal => (a.clone(), b.clone()),
            Ordering::Less => {
                let scale_factor = b.scale / a.scale;
                (
                    FixedPoint {
                        integer: a.integer,
                        decimal: a.decimal * scale_factor,
                        sign: a.sign,
                        scale: b.scale,
                    },
                    b.clone(),
                )
            }
            Ordering::Greater => {
                let scale_factor = a.scale / b.scale;
                (
                    a.clone(),
                    FixedPoint {
                        integer: b.integer,
                        decimal: b.decimal * scale_factor,
                        sign: b.sign,
                        scale: a.scale,
                    },
                )
            }
        }
    }

    pub fn scale_to_fit(&mut self) {
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

    pub fn to_raw_value(&self) -> i128 {
        let sign_multiplier: i32 = if self.sign < 0 { -1 } else { 1 };
        let combined_value: i128 = (self.integer as i128 * self.scale as i128) + (self.decimal as i128);
        combined_value * sign_multiplier as i128
    }

    pub fn trim_ending_zeros(&mut self) {
        while self.decimal != 0 && self.decimal % 10 == 0 && self.scale > 1 {
            self.decimal /= 10;
            self.scale /= 10;
        }
    }

    pub fn calculate_product_total(&self, rhs: Self) -> Result<i64, &'static str> {
        let (self_scaled, rhs_scaled): (FixedPoint, FixedPoint) = Self::scale_to_match(self, &rhs);

        let integer_mul: i64 = self_scaled.integer.checked_mul(rhs_scaled.scale)
            .ok_or(MULTIPLICATION_OVERFLOW)?;

        let integer_rhs_mul: i64 = integer_mul.checked_mul(rhs_scaled.integer)
            .ok_or(MULTIPLICATION_OVERFLOW)?;

        let integer_decimal_1: i64 = self_scaled.integer.checked_mul(rhs_scaled.decimal)
            .ok_or(MULTIPLICATION_OVERFLOW)?;

        let integer_decimal_2: i64 = self_scaled.decimal.checked_mul(rhs_scaled.integer)
            .ok_or(MULTIPLICATION_OVERFLOW)?;

        let decimal_decimal_mul: i64 = self_scaled.decimal.checked_mul(rhs_scaled.decimal)
            .ok_or(MULTIPLICATION_OVERFLOW)?;

        let product_total: i64 = integer_rhs_mul
            + integer_decimal_1
            + integer_decimal_2
            + (decimal_decimal_mul / self_scaled.scale);

        Ok(product_total)
    }
}
