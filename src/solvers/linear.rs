pub fn solve_linear(coefficients: &[f64]) -> Option<Vec<f64>> {
    if coefficients.len() < 2 {
        return None;
    }

    let a: f64 = coefficients[1];
    let b: f64 = coefficients[0];

    if a == 0.0 {
        println!("Linear equation with no solution (a = 0).");
        None
    } else if b == 0.0 {
        Some(vec![0.0])
    } else {
        let solution: f64 = -b / a;
        dbg!(&solution);
        println!("Linear equation with one solution: -b/a = {:.}/{:.}", -b, a);
        Some(vec![solution])
    }
}
