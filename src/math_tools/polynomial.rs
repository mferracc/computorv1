use crate::math_tools::basic;
use crate::parser;
use crate::solvers::linear::solve_linear;
use crate::solvers::quadratic::solve_quadratic;

pub struct Polynomial {
    pub degree: usize,
    pub coefficients: Vec<f64>,
    pub solutions: Option<Vec<f64>>,
}

impl Polynomial {
    pub fn new(equation: &str) -> Result<Self, String> {
        let mut coefficients: Vec<f64> = parser::parse_input(equation)?;
        let degree: usize = Self::get_polynomial_degree(&mut coefficients);

        Ok(Polynomial {
            degree,
            coefficients,
            solutions: None,
        })
    }

    pub fn get_polynomial_degree(coefficients: &mut Vec<f64>) -> usize {
        let mut degree: usize = coefficients.len() - 1;

        while degree > 0 {
            if coefficients[degree] == 0.0 {
                degree -= 1;
                coefficients.pop();
            } else {
                return degree;
            }
        }
        degree
    }

    pub fn solve(&mut self) {
        self.display_reduced_form();
        self.display_degree();

        self.solutions = match self.degree {
            0 => None,
            1 => solve_linear(&self.coefficients),
            2 => solve_quadratic(&self.coefficients),
            _ => {
                println!("Degree > 2 not supported");
                None
            }
        };
        self.display_solutions()
    }

    /// Private part
    fn display_reduced_form(&self) {
        let mut terms: Vec<String> = Vec::new();

        for (index, coefficient) in self.coefficients.iter().enumerate() {
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

    fn display_degree(&self) {
        println!("Polynomial degree: {}", self.degree);
    }

    fn display_solutions(&self) {
        print!("Solutions:");
        match &self.solutions {
            Some(solutions) if solutions.is_empty() => println!("no solutions."),
            Some(solutions) => {
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
            None => {
                if self.degree == 1 {
                    println!(" all real numbers are solutions.")
                } else {
                    println!(" None")
                }
            }
        }
    }
}