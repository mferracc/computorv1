use crate::constants::math_tools_constants::MAX_SCALE;

#[derive(Clone, Copy)]
pub struct FixedPoint {
    pub integer: i64,
    pub decimal: i64,
    pub sign: i64,
    pub scale: i64,
}

impl FixedPoint {
    pub const fn new(integer: i64, decimal: i64, sign: i64) -> Self {
        let mut scale: i64 = 1;
        let mut temp_decimal: i64 = decimal;

        while temp_decimal != 0 && scale < MAX_SCALE {
            temp_decimal /= 10;
            scale *= 10;
        }
        Self::new_with_scale(integer, decimal, sign, scale)
    }

    pub const fn new_with_scale(mut integer: i64, mut decimal: i64, mut sign: i64, mut scale: i64) -> Self {
        if integer == 0 && decimal == 0 {
            sign = 1;
        } else if integer < 0 || decimal < 0 {
            sign = -1;
            integer = integer.abs();
            decimal = decimal.abs();
        }

        while decimal % 10 == 0 && scale > 1 {
            decimal /= 10;
            scale /= 10;
        }

        FixedPoint {
            integer,
            decimal,
            sign,
            scale,
        }
    }

    pub fn new_from_f_64(value: f64) -> Self {
        let precision: u32 = 10;
        let scale: i64 = 10_i64.pow(precision);

        let sign: i64 = if value < 0.0 { -1 } else { 1 };

        let scaled_value: i64 = (value.abs() * scale as f64).round() as i64;

        let integer: i64 = scaled_value / scale;
        let decimal: i64 = scaled_value % scale;

        let integer: i64 = if sign < 0 { -integer } else { integer };
        let decimal: i64 = if sign < 0 { scale - decimal } else { decimal };

        Self::new_with_scale(integer.abs(), decimal, sign, scale)
    }

    pub fn to_f64(&self) -> f64 {
        let total: i64 = self.sign * (self.integer * self.scale + self.decimal);
        total as f64 / self.scale as f64
    }
}
