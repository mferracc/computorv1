use crate::math_tools::basic;
use crate::math_tools::polynomial::Polynomial;

pub fn display_solved_polynomial(polynomial: &Polynomial) {
    display_reduced_form(polynomial);
    display_degree(polynomial);
    display_solutions(polynomial);
}

/// Private part
fn display_reduced_form(polynomial: &Polynomial) {
    let mut terms: Vec<String> = Vec::new();

    for (index, coefficient) in polynomial.coefficients.iter().enumerate() {
        let value: f64 = *coefficient;
        if value != 0.0 {
            let mut term: String = format!("{}", value);
            if index > 0 {
                term.push_str("*X");
                if index > 1 {
                    term.push('^');
                    term.push_str(&index.to_string());
                }
            }
            terms.push(term);
        }
    }
    let reduced_form = terms.join(" + ").replace("+ -", "- ");
    println!("Reduced form: {}", reduced_form);
}

fn display_degree(polynomial: &Polynomial) {
    println!("Polynomial degree: {}", polynomial.degree);
}

fn display_solutions(polynomial: &Polynomial) {
    print!("Solutions:");
    match &polynomial.solutions {
        Some(solutions) if solutions.is_empty() => println!("no solutions."),
        Some(solutions) => {
            if polynomial.degree == 2 && polynomial.discriminant < 0.0 {
                display_complex_solutions(solutions);
            } else {
                display_real_solutions(solutions);
            }
        }
        None => {
            if polynomial.degree == 1 {
                println!(" all real numbers are solutions.")
            } else {
                println!(" None")
            }
        }
    }
}

fn display_complex_solutions(solutions: &[f64]) {
    let real_part: f64 = solutions[0];
    let imaginary_part: f64 = solutions[1];

    println!("{} + i * {} and {} - i * {}", real_part, imaginary_part, real_part, imaginary_part);
}

fn display_real_solutions(solutions: &[f64]) {

    for solution in solutions {
        print!(" {} ", solution);
        if let Some((numerator, denominator)) = basic::convert_to_irreducible(*solution) {
            if denominator != 1 {
                print!("({}/{})", numerator, denominator);
            }
        }
    }
    println!();
}