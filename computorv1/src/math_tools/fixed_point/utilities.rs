use std::cmp::Ordering;
use crate::constants::math_tools_constants::{NEGATIVE, POSITIVE};
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
}