use crate::math_tools::polynomial::Polynomial;

pub fn solve_linear(polynomial: &mut Polynomial) {
    let a: f64 = polynomial.coefficients[1].to_f64();
    let b: f64 = polynomial.coefficients[0].to_f64();
    let _solution: Option<f64> = if a == 0.0 {
        None
    } else {
        Some(-b/a)
    };
    // store the solution inside the polynomial
    todo!()
}


