use crate::math_tools::fixed_point::fixed_point::FixedPoint;
use crate::parser;
use crate::solvers::bigger_degree::solve_bigger_degree;
use crate::solvers::cubic::solve_cubic;
use crate::solvers::linear::solve_linear;
use crate::solvers::quadratic::solve_quadratic;
use crate::solvers::quartic::solve_quartic;

pub struct Polynomial {
    pub degree: usize,
    pub coefficients: Vec<FixedPoint>,
    pub discriminant: Option<FixedPoint>,
    pub solutions: Option<Vec<FixedPoint>>,
}

impl Polynomial {
    pub fn new(equation: &str) -> Result<Self, String> {
        let coefficients: Vec<FixedPoint> = parser::parse_input(equation)?;
        let degree: usize = coefficients.len() - 1;

        Ok(Polynomial {
            degree,
            coefficients,
            discriminant: None,
            solutions: None,
        })
    }

    pub fn solve(&mut self) {
        self.solutions = match self.degree {
            0 => None,
            1 => solve_linear(&self.coefficients),
            2 => solve_quadratic(self),
            3 => solve_cubic(self),
            4 => solve_quartic(self),
            _ => solve_bigger_degree(self),
        };
    }

    pub fn display_result(&self) {
        self.display_reduced_form();
        self.display_degree();
        self.display_discriminant();
        self.display_solutions();
    }

    /// Private part

    fn display_reduced_form(&self) {
        let mut terms: Vec<String> = Vec::new();

        for (index, coefficient) in self.coefficients.iter().enumerate() {
            let value: f64 = coefficient.to_f64();
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

    fn display_discriminant(&self) {
        if !self.discriminant.is_none() {
            print!(
                "Discriminant is {:?}",
                self.discriminant
            );
            if !self.solutions.is_none() {
                println!(", the two solutions are:");
            } else {
                println!(", no solutions.");
            }
        }
    }

    fn display_solutions(&self) {
        print!("Solutions: ");
        match &self.solutions {
            Some(solutions) if solutions.is_empty() => println!("no solutions."),
            Some(solutions) => {
                for solution in solutions {
                    println!("  {}", solution);
                }
            }
            None => println!("all real numbers are solutions."),
        }
    }
}