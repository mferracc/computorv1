/// Compute the value of the square root using the Babylonian method (or Newton method).
pub fn square_root(value: f64) -> Option<f64> {
    if value < 0.0 {
        return None;
    } else if value == 0.0 {
        return Some(0.0);
    }

    let mut guess: f64 = value;
    let tolerance: f64 = 1e-20;

    loop {
        let next_guess = (guess + value / guess) / 2.0;

        if (next_guess - guess).abs() < tolerance {
            return Some(next_guess);
        }
        guess = next_guess;
    }
}
