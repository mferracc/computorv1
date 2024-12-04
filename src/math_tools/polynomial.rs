use crate::parser;
use crate::solvers::linear::solve_linear;
use crate::solvers::quadratic::solve_quadratic;

pub struct Polynomial {
    pub degree: usize,
    pub coefficients: Vec<f64>,
    pub solutions: Option<Vec<f64>>,
    pub discriminant: f64,
}

impl Polynomial {
    pub fn new(equation: &str) -> Result<Self, String> {
        let mut coefficients: Vec<f64> = parser::parse_input(equation)?;
        let degree: usize = Self::get_polynomial_degree(&mut coefficients);

        Ok(Polynomial {
            degree,
            coefficients,
            solutions: None,
            discriminant: 0.0,
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
        self.solutions = match self.degree {
            0 => None,
            1 => solve_linear(&self.coefficients),
            2 => solve_quadratic(&self.coefficients),
            _ => {
                println!("Degree > 2 not supported");
                None
            }
        };
    }
}
