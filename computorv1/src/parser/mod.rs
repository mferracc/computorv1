use crate::constants::{OPERATORS, EMPTY_INPUT, INVALID_POWER, INVALID_COEFFICIENT, POSITIVE};
use std::collections::HashMap;
use crate::math_tools::fixed_point::FixedPoint;

pub fn parse_input(input: &str) -> Result<Vec<FixedPoint>, String> {
    dbg!(&input);
    let equation_members: (String, String) = split_input(input)?;

    let left_coefficients: HashMap<usize, FixedPoint> = parse_equation(&equation_members.0)?;
    let right_coefficients: HashMap<usize, FixedPoint> = parse_equation(&equation_members.1)?;

    dbg!(&left_coefficients, &right_coefficients);
    Ok(sum_coefficients(left_coefficients, right_coefficients))
}

/// Private part

fn split_input(input: &str) -> Result<(String, String), String> {
    let cleaned_input: String = input.replace(" ", "");

    let (left, right) = cleaned_input
        .split_once('=')
        .unwrap_or((&cleaned_input, ""));

    if cleaned_input.is_empty() || left.is_empty() {
        return Err(EMPTY_INPUT.to_string());
    }

    Ok((left.to_string(), right.to_string()))
}

fn sum_coefficients(
    left: HashMap<usize, FixedPoint>,
    right: HashMap<usize, FixedPoint>
) -> Vec<FixedPoint> {
    let degree = left.keys().chain(right.keys()).copied().max().unwrap_or(0);
    let mut coefficients: Vec<FixedPoint> = vec![FixedPoint::new(0,0, POSITIVE);degree + 1];

    for (&power, coeff) in &left {
        coefficients[power] = coeff.clone();
    }
    for (&power, coeff) in &right {
        coefficients[power] -= coeff.clone();
    }

    coefficients
}

fn parse_equation(input: &str) -> Result<HashMap<usize, FixedPoint>, String> {
    let terms: Vec<String> = split_inclusive(input);
    let mut coefficients: HashMap<usize, FixedPoint> = HashMap::new();

    for term in terms {
        let (power, coefficient): (usize, FixedPoint) = split_term(&term)?;
        coefficients
            .entry(power)
            .and_modify(|c| {*c += coefficient.clone()})
            .or_insert(coefficient.clone());
    }

    Ok(coefficients)
}

fn split_inclusive(input: &str) -> Vec<String> {
    let mut terms: Vec<String> = Vec::new();
    let mut current_term: String = String::new();

    for c in input.chars() {
        if OPERATORS.contains(c) {
            if !current_term.is_empty() {
                terms.push(current_term.to_string());
            }
            current_term = c.to_string();
        } else {
            current_term.push(c);
        }
    }

    if !current_term.is_empty() {
        terms.push(current_term.to_string());
    }

    terms
}

fn split_term(signed_term: &str) -> Result<(usize, FixedPoint), String> {
    let mut term: &str = signed_term;

    let sign: i64 = if let Some(x) = get_coeff_sign(signed_term) {
        term = &term[1..];
        x
    } else {
        1
    };

    if let Some((coefficient, power))= term
        .split_once('*') {
        let coefficient: FixedPoint = extract_coefficient(coefficient, sign)?;
        let power: usize = extract_power(power)?;
        Ok((power, coefficient))
    } else if term.starts_with("X") {
        let power: usize = extract_power(term)?;
        Ok ((power, FixedPoint::new(1, 0, sign)))
    } else {
        let coefficient: FixedPoint = extract_coefficient(term, sign)?;
        Ok((0, coefficient))
    }
}

fn get_coeff_sign(term: &str) -> Option<i64> {
    match term.as_bytes().first() {
        Some(b'-') => Some(-1),
        Some(b'+') => Some(1),
        _ => None
    }
}

fn extract_power(power: &str) -> Result<usize, String> {
    if power.is_empty() {
        Ok(0)
    } else if power == "X" {
        Ok(1)
    } else if let Some(stripped) = power.strip_prefix("X^") {
        stripped
            .parse::<usize>()
            .map_err(|_| format!("{}{}", INVALID_POWER, power))
    } else {
        Err(format!("{}{}", INVALID_POWER, power))
    }
}

fn extract_coefficient(coefficient: &str, sign: i64) -> Result<FixedPoint, String> {
    let fixed_parts: (&str, &str) = coefficient
        .split_once('.')
        .unwrap_or((coefficient, "0"));

    let integer_part: i64 = fixed_parts.0
        .parse::<i64>()
        .map_err(|_| format!("{}{}", INVALID_COEFFICIENT, coefficient))?;

    let decimal_str: &str = fixed_parts.1.trim_end_matches('0');
    let scale: i64 = 10_i64.checked_pow(decimal_str.len() as u32)
        .ok_or_else(|| format!("decimal overflow for value: {}", decimal_str))?;
    let decimal_part: i64 = if !decimal_str.is_empty() {
        decimal_str
            .parse::<i64>()
            .map_err(|_| format!("{}{}", INVALID_COEFFICIENT, coefficient))?
    } else {
        0
    };

    Ok(FixedPoint::new_with_scale(integer_part, decimal_part, sign, scale))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn parse_input_to_vec_f64(input: &str) -> Result<Vec<f64>, String> {
        let nums: Vec<FixedPoint> = parse_input(input)?;
        let mut result: Vec<f64> = Vec::new();

        for num in nums {
            result.push(num.to_f64());
        }

        Ok(result)
    }

    #[test]
    fn test_wrong_input() {
        let inputs= [
            "",
            "eoido",
            "829h + 23 - i3ehbd + 2hr93bdi23 - idb9ub2f + ub2idu2idb",
            "++--+-+-+++++-",
            "=",
            "+=+",
            "X*",
            "*+-+=*-",
            "+=2",
            "3 * X ^ + 4",
            "3X",
            "4*X^6.7",
            "999999999999999999999999999999999999999999999999999999999999",
            "0.0000000000000000000000000000000000000000000000000000000001",
        ];

        for input in inputs {
            let result = parse_input_to_vec_f64(input);
            assert!(
                result.is_err(),
                "Expected error for input '{}', but got: {:?}",
                input,
                result
            );
        }

    }

    #[test]
    fn test_simple_input() {
        let inputs = [
            ("3", vec![3.0]),
            ("3*X", vec![0.0, 3.0]),
            ("3*X^2", vec![0.0, 0.0, 3.0]),
            ("3*X^3", vec![0.0, 0.0, 0.0, 3.0]),
            ("X", vec![0.0, 1.0]),
            ("3 * X ^ 0 + 6*X ^1+   X", vec![3.0, 7.0]),
            (" 0 * X ^ 2", vec![0.0, 0.0, 0.0]),
            (" 0.4 * X ^ 2", vec![0.0, 0.0, 0.4]),
            ("3*X - 3.4321", vec![-3.4321, 3.0]),
            ("-5*X +  4 -0", vec![4.0, -5.0]),
            ("  X^5 - 4.8 * X^2 + 6.6-2-1", vec![3.6, 0.0, -4.8, 0.0, 0.0, 1.0]),
            ("X^1 + 5.2*X^0", vec![5.2, 1.0]),
        ];

        for (input, expected) in inputs {
            match parse_input_to_vec_f64(input) {
                Ok(result) => {
                    assert_eq!(
                        result,
                        expected,
                        "Unexpected result for input {}: {:?} (expected: {:?})",
                        input,
                        result,
                        expected,
                    );
                }
                Err(e) => {
                    panic!(
                        "Unexpected result for input {}: {:?} (expected: {:?})",
                        input,
                        e,
                        expected,
                    );
                }
            }
        }
    }

    #[test]
    fn test_equation_input() {
        let inputs = [
            ("X=X", vec![0.0, 0.0]),
            ("3=3", vec![0.0]),
            ("3.4*X = 3", vec![-3.0, 3.4]),
            ("3*X^2 = 8*X^2", vec![0.0, 0.0, -5.0]),
            ("3*X^3 = 3.333 - 7.56*X", vec![-3.333, 7.56, 0.0, 3.0]),
            ("X = X^2 + 6*X", vec![0.0, -5.0, -1.0]),
            ("3 * X ^ 0 + 6*X ^1+   X= 0", vec![3.0, 7.0]),
            ("3*X - 3 = -9*X^3 + X^2", vec![-3.0, 3.0, -1.0, 9.0]),
            ("  X^5 - 4 * X^2 + 6-2-1 = 36*X", vec![3.0, -36.0, -4.0, 0.0, 0.0, 1.0]),
            ("X^1 + 5*X^0= 7.87 + 0.001", vec![-2.871, 1.0]),
            ("-5*X +  4 -0=9-8*X^2", vec![-5.0, -5.0, 8.0]),
        ];

        for (input, expected) in inputs {
            match parse_input_to_vec_f64(input) {
                Ok(result) => {
                    assert_eq!(
                        result,
                        expected,
                        "Unexpected result for input {}: {:?} (expected: {:?})",
                        input,
                        result,
                        expected,
                    );
                }
                Err(e) => {
                    panic!(
                        "Unexpected result for input {}: {:?} (expected: {:?})",
                        input,
                        e,
                        expected,
                    );
                }
            }
        }
    }
}
