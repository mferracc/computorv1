use crate::constants::math_tools_constants::{MAX_SCALE};

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
