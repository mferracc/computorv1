use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::ops::Sub;
use crate::constants::math_tools_constants::{MAX_VALUE, POSITIVE, NEGATIVE};
use crate::math_tools::fixed_point::fixed_point::FixedPoint;

impl PartialEq for FixedPoint {
    fn eq(&self, other: &Self) -> bool {
        let (a, b): (Self, Self) = Self::scale_to_match(self, other);

        a.integer == b.integer && a.decimal == b.decimal
    }
}

impl PartialOrd for FixedPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let (self_scaled, other_scaled) = Self::scale_to_match(self, &other);

        match (self.sign, other.sign) {
            (POSITIVE, NEGATIVE) => Some(Ordering::Greater),
            (NEGATIVE, POSITIVE) => Some(Ordering::Less),
            _ => {
                self_scaled.to_f64().partial_cmp(&other_scaled.to_f64())
            }
        }
    }
}

impl FixedPoint {
    pub fn is_finite(&self) -> bool {
        let value = self.integer * self.scale + self.decimal;
        value.abs() <= MAX_VALUE
    }
    pub fn approx_eq(&self, other: &FixedPoint, tolerance: &FixedPoint) -> bool {
        let diff = self.sub(other).abs();
        diff <= *tolerance
    }
}

