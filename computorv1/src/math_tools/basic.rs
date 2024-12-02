use crate::constants::math_tools_constants::TOLERANCE;

/// Compute the value of the square root using the Babylonian method (or Newton method).
pub fn square_root(value: f64) -> Option<f64> {
    if value < 0.0 {
        return None;
    } else if value == 0.0 {
        return Some(0.0);
    }

    let value: f64 = value;
    let mut guess: f64 = value;

    loop {
        let next_guess: f64 = (guess + value / guess) / 2.0;

        if (next_guess - guess).abs() < TOLERANCE {
            return Some(next_guess);
        }
        guess = next_guess;
    }
}
