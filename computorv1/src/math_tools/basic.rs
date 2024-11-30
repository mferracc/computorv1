use crate::constants::math_tools_constants::{TOLERANCE, ZERO};
use crate::math_tools::fixed_point::fixed_point::FixedPoint;

/// Compute the value of the square root using the Babylonian method (or Newton method).
pub fn square_root(value: FixedPoint) -> Option<FixedPoint> {
    if value < ZERO {
        return None;
    } else if value == ZERO {
        return Some(ZERO);
    }

    let value: f64 = value.to_f64();
    let mut guess: f64 = value;

    loop {
        let next_guess: f64 = (guess + value / guess) / 2.0;

        if (next_guess - guess).abs() < TOLERANCE {
            return Some(FixedPoint::new_from_f_64(next_guess));
        }
        guess = next_guess;
    }
}
