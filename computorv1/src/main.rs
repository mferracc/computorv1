use std::{env, process};
use computorv1::Polynomial;

fn main() {
    let args: Vec<String> = env::args().collect();
    let polynomial: Polynomial = Polynomial::new(&args).unwrap_or_else(|err: String| {
        println!("Error: {err}.");
        process::exit(1);
    });

    dbg!(args);

    // polynomial::solve;
    // polynomial::display_result;
}
