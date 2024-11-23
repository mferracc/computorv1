use crate::constants::parsing_constants::INVALID_ARG_NUMBER;
use crate::math_tools::fixed_point::FixedPoint;
use crate::parser;

pub struct Polynomial {
    degree: usize,
    coefficients: Vec<FixedPoint>,
    discriminant: Option<FixedPoint>,
    solutions: Option<Vec<FixedPoint>>,
}

impl Polynomial {
    pub fn new(args: &[String]) -> Result<Self, String> {
        if args.len() != 2 {
            return Err(INVALID_ARG_NUMBER.to_string());
        }

        let coefficients: Vec<FixedPoint> = parser::parse_input(&args[1])?;
        let degree: usize = coefficients.len() - 1;

        Ok(Polynomial {
            degree,
            coefficients,
            discriminant: None,
            solutions: None,
        })
    }

    pub fn solve(&self) {
        // calc ∆ and solutions
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
                    term.push_str(" * X");
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
        println!(
            "Discriminant is {:?}, the two solutions are:",
            self.discriminant
        );
    }

    fn display_solutions(&self) {
        println!("Solutions: {:?}", self.solutions);
    }
}
