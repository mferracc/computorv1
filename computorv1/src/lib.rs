use crate::constants::INVALID_ARG_NUMBER;
use crate::math_tools::fixed_point::FixedPoint;

pub mod constants;
pub mod math_tools;
pub mod parser;
pub mod solvers;

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
        unimplemented!();
        // calc ∆ and solutions
    }

    pub fn display_result(&self) {
        println!("Reduced form: {:?}", self.coefficients);
        println!("Polynomial degree: {}", self.degree);
        println!(
            "Discriminant is {:?}, the two solutions are:",
            self.discriminant
        );
        println!("Solutions: {:?}", self.solutions);
    }
}
