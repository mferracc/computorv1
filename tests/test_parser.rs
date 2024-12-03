#[cfg(test)]
mod tests {
    use computorv1::parser;

    #[test]
    fn test_wrong_input() {
        let inputs = [
            "",
            "eoido",
            "829h + 23 - i3ehbd + 2hr93bdi23 - idb9ub2f + ub2idu2idb",
            "++--+-+-+++++-",
            "=",
            "+=+",
            "X*",
            "*+-+=*-",
            "+=2",
            "3 * X ^ + 4",
            "3X",
            "4*X^6.7",
        ];

        for input in inputs {
            let result = parser::parse_input(input);
            assert!(
                result.is_err(),
                "Expected error for input '{}', but got: {:?}",
                input,
                result
            );
        }
    }

    fn run_parse_tests(inputs: &[(&str, Vec<f64>)]) {
        for (input, expected) in inputs {
            match parser::parse_input(input) {
                Ok(result) => {
                    assert_eq!(
                        result, *expected,
                        "Unexpected result for input {}: {:?} (expected: {:?})",
                        input, result, expected,
                    );
                }
                Err(e) => {
                    panic!(
                        "Unexpected result for input {}: {:?} (expected: {:?})",
                        input, e, expected,
                    );
                }
            }
        }
    }

    #[test]
    fn test_simple_input() {
        let inputs = [
            ("3", vec![3.0]),
            ("3*X", vec![0.0, 3.0]),
            ("3*X^2", vec![0.0, 0.0, 3.0]),
            ("3*X^3", vec![0.0, 0.0, 0.0, 3.0]),
            ("X", vec![0.0, 1.0]),
            ("3 * X ^ 0 + 6*X ^1+   X", vec![3.0, 7.0]),
            (" 0 * X ^ 2", vec![0.0, 0.0, 0.0]),
            (" 0.4 * X ^ 2", vec![0.0, 0.0, 0.4]),
            ("3*X - 3.4321", vec![-3.4321, 3.0]),
            ("-5*X +  4 -0", vec![4.0, -5.0]),
            (
                "  X^5 - 4.8 * X^2 + 6.6-2-1",
                vec![3.6, 0.0, -4.8, 0.0, 0.0, 1.0],
            ),
            ("X^1 + 5.2*X^0", vec![5.2, 1.0]),
        ];
        run_parse_tests(&inputs);
    }

    #[test]
    fn test_equation_input() {
        let inputs = [
            ("X=X", vec![0.0, 0.0]),
            ("3=3", vec![0.0]),
            ("3.4*X = 3", vec![-3.0, 3.4]),
            ("3*X^2 = 8*X^2", vec![0.0, 0.0, -5.0]),
            ("3*X^3 = 3.333 - 7.56*X", vec![-3.333, 7.56, 0.0, 3.0]),
            ("X = X^2 + 6*X", vec![0.0, -5.0, -1.0]),
            ("3 * X ^ 0 + 6*X ^1+   X= 0", vec![3.0, 7.0]),
            ("3*X - 3 = -9*X^3 + X^2", vec![-3.0, 3.0, -1.0, 9.0]),
            (
                "  X^5 - 4 * X^2 + 6-2-1 = 36*X",
                vec![3.0, -36.0, -4.0, 0.0, 0.0, 1.0],
            ),
            ("X^1 + 5*X^0= 7.87 + 0.001", vec![-2.871, 1.0]),
            ("-5*X +  4 -0=9-8*X^2", vec![-5.0, -5.0, 8.0]),
        ];
        run_parse_tests(&inputs);
    }
}