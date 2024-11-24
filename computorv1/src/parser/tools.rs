use std::collections::HashMap;
use crate::constants::math_tools_constants::POSITIVE;
use crate::constants::parsing_constants::{EMPTY_INPUT, INVALID_COEFFICIENT, INVALID_POWER, OPERATORS};
use crate::math_tools::fixed_point::fixed_point::FixedPoint;

pub fn split_input(input: &str) -> Result<(String, String), String> {
    let cleaned_input: String = input.replace(' ', "");

    let (left, right) = cleaned_input
        .split_once('=')
        .unwrap_or((&cleaned_input, ""));

    if cleaned_input.is_empty() || left.is_empty() {
        return Err(EMPTY_INPUT.to_string());
    }

    Ok((left.to_string(), right.to_string()))
}

pub fn sum_coefficients(
    left: HashMap<usize, FixedPoint>,
    right: HashMap<usize, FixedPoint>,
) -> Vec<FixedPoint> {
    let degree = left.keys().chain(right.keys()).copied().max().unwrap_or(0);
    let mut coefficients: Vec<FixedPoint> = vec![FixedPoint::new(0, 0, POSITIVE); degree + 1];

    for (&power, coeff) in &left {
        coefficients[power] = coeff.clone();
    }
    for (&power, coeff) in &right {
        coefficients[power] -= coeff.clone();
    }

    coefficients
}

pub fn parse_equation(input: &str) -> Result<HashMap<usize, FixedPoint>, String> {
    let terms: Vec<String> = split_inclusive(input);
    let mut coefficients: HashMap<usize, FixedPoint> = HashMap::new();

    for term in terms {
        let (power, coefficient): (usize, FixedPoint) = split_term(&term)?;
        coefficients
            .entry(power)
            .and_modify(|c| *c += coefficient.clone())
            .or_insert_with(|| coefficient.clone());
    }

    Ok(coefficients)
}

pub fn split_inclusive(input: &str) -> Vec<String> {
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

pub fn split_term(signed_term: &str) -> Result<(usize, FixedPoint), String> {
    let mut term: &str = signed_term;

    let sign: i64 = if let Some(x) = get_coeff_sign(signed_term) {
        term = &term[1..];
        x
    } else {
        1
    };

    if let Some((coefficient, power)) = term.split_once('*') {
        let coefficient: FixedPoint = extract_coefficient(coefficient, sign)?;
        let power: usize = extract_power(power)?;
        Ok((power, coefficient))
    } else if term.starts_with('X') {
        let power: usize = extract_power(term)?;
        Ok((power, FixedPoint::new(1, 0, sign)))
    } else {
        let coefficient: FixedPoint = extract_coefficient(term, sign)?;
        Ok((0, coefficient))
    }
}

pub fn get_coeff_sign(term: &str) -> Option<i64> {
    match term.as_bytes().first() {
        Some(b'-') => Some(-1),
        Some(b'+') => Some(1),
        _ => None,
    }
}

pub fn extract_power(power: &str) -> Result<usize, String> {
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

pub fn extract_coefficient(coefficient: &str, sign: i64) -> Result<FixedPoint, String> {
    let fixed_parts: (&str, &str) = coefficient.split_once('.').unwrap_or((coefficient, "0"));

    let integer_part: i64 = fixed_parts
        .0
        .parse::<i64>()
        .map_err(|_| format!("{}{}", INVALID_COEFFICIENT, coefficient))?;

    let decimal_str: &str = fixed_parts.1.trim_end_matches('0');
    let scale: i64 = 10_i64
        .checked_pow(decimal_str.len() as u32)
        .ok_or_else(|| format!("decimal overflow for value: {}", decimal_str))?;
    let decimal_part: i64 = if !decimal_str.is_empty() {
        decimal_str
            .parse::<i64>()
            .map_err(|_| format!("{}{}", INVALID_COEFFICIENT, coefficient))?
    } else {
        0
    };

    Ok(FixedPoint::new_with_scale(
        integer_part,
        decimal_part,
        sign,
        scale,
    ))
}