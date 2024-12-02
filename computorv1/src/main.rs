use computorv1::constants::parsing_constants::INVALID_ARG_NUMBER;
use computorv1::math_tools::polynomial::Polynomial;
use std::{env, process};

fn handle_error(message: &str) -> ! {
    println!("Error: {message}.");
    process::exit(1);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        handle_error(INVALID_ARG_NUMBER);
    }

    let equation: &String = &args[1];
    let mut polynomial: Polynomial =
        Polynomial::new(equation).unwrap_or_else(|err| handle_error(&err));

    polynomial.solve();
}
