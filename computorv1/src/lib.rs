pub mod solvers;
pub mod math_tools;

pub struct Polynomial {
    degree: u8,
    coefficients: Vec<f64>,
    discriminant: Some(f64),
    solutions: Vec<f64>,
}

impl Polynomial {
    pub fn build(args: &[String]) -> Result<Polynomial, &'static str> {
        if args.len() < 2 {
            return Err("wrong usage");
        }

        let degree: u8 = 0;
        let coefficients: Vec<f64> = Vec::new();
        let discriminant: Some(f64) = None;
        let solutions: Vec<f64> = Vec::new();

        Ok(Polynomial {
            degree,
            coefficients,
            discriminant, // calc later
            solutions, // calc later
        })
    }

    pub fn solve() {
        unimplemented!();
        // calc ∆ and solutions
    }

    pub fn display_result(self) {
        println!("Reduced form: ");
        //println!("Polynomial degree: {}", self::degree)
        println!("Discriminant is ...blablabla..., the two solutions are:");
        println!("les solutions");
    }
}