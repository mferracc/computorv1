use std::{env, process};
use computorv1::Polynomial;

fn main() {
    let args: Vec<String> = env::args().collect();
    let polynomial: Polynomial = Polynomial::build(&args).unwrap_or_else(|err: &str| {
        println!("Error: {err}.");
        process::exit(1);
    });

    // polynomial::solve;
    // polynomial::display_result;
}