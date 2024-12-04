use crate::constants::math_tools_constants::{FRACT_PRECISION, TOLERANCE};

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

pub fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

pub fn convert_to_irreductible(value: f64) -> Option<(i64, i64)> {
    if value.fract() == 0.0 {
        return None;
    }

    let (numerator, denominator): (i64, i64) = (
        (value * FRACT_PRECISION as f64).round() as i64,
        FRACT_PRECISION
    );


    let divisor: i64 = gcd(numerator, denominator);

    let (reduced_numerator, reduced_denominator): (i64, i64) = (
        numerator / divisor,
        denominator / divisor
    );

    if reduced_denominator < 0 {
        return Some((-reduced_numerator, -reduced_denominator));
    }

    Some((reduced_numerator, reduced_denominator))
}