/// Compute the value of the square root using the Babylonian method (or Newton method).
pub fn square_root(value: f64) -> Option<f64>
{
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_square_root() {
        assert_eq!(square_root(25.0), Some(5.0));
        assert_eq!(square_root(9.0), Some(3.0));
        assert_eq!(square_root(4.0), Some(2.0));
        assert_eq!(square_root(0.0), Some(0.0));
        assert_eq!(square_root(-1.0), None);
        assert_eq!(square_root(-100.0), None);
        assert_eq!(square_root(1e10), Some(100000.0));
        assert_eq!(square_root(1e20), Some(1e10));
    }
}