use std::fmt;
use std::fmt::Formatter;
use crate::math_tools::fixed_point::fixed_point::FixedPoint;

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

impl fmt::Display for FixedPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign_str: &str = if self.sign < 0 { "-" } else { "" };

        let integer_part: i64 = self.integer.abs();
        let decimal_part: i64 = self.decimal.abs();
        let scale_str: String = format!("{:0width$}", decimal_part, width = self.scale as usize);

        if self.scale > 0 {
            write!(f, "{}{}.{},", sign_str, integer_part, scale_str)
        } else {
            write!(f, "{}{}", sign_str, integer_part)
        }
    }
}
