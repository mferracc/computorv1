use crate::math_tools::fixed_point::fixed_point::FixedPoint;

/// Fixed point numbers
pub const MAX_SCALE: i64 = 1_000_000_000_000_000;
pub const MAX_VALUE: i64 = 1_000_000_000;
pub const POSITIVE: i64 = 1;
pub const NEGATIVE: i64 = -1;
pub const ZERO: FixedPoint = FixedPoint::new(0,0,POSITIVE);
pub const MULTIPLICATION_OVERFLOW: &str = "Multiplication overflow";

/// Square root
pub const TOLERANCE: f64 = 1e-20;
