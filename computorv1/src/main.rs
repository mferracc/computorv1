struct Polynomial {
    degree: u32,
    coefficients: Vec<f64>,
    discriminant: f64,
//    reduced_form: String,     -> implements a method instead
    solutions: Vec<String>,     // Are we really keeping the type String for the solutions??
    // solve_equation()     -> method for solving the equation and displaying output at the same time (see subject)
}

fn main() {
    println!("Hello, world!");
}
