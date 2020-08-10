mod parser;
mod tokenizer;

use std::env;
use std::io::{self, Write};

fn eval(expression: &str) -> f64 {
    let tokens = tokenizer::tokenize(expression);
    parser::parse(tokens)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        println!("{}", eval(&args[1]));
    } else {
        let mut running = true;

        while running {
            let mut expr = String::new();
            print!(">> ");
            io::stdout().flush().unwrap();
            io::stdin().read_line(&mut expr).unwrap();

            if expr.trim() == "quit" {
                running = false;
                continue;
            }

            println!("{}", eval(&expr));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::eval;
    use assert_approx_eq::assert_approx_eq;

    #[test]
    fn basic_addition() {
        assert_approx_eq!(eval("2 + 2"), 4.0);
    }

    #[test]
    fn chained_additions() {
        assert_approx_eq!(eval("1 + 2 + 3 + 4"), 10.0);
    }

    #[test]
    fn basic_subtraction() {
        assert_approx_eq!(eval("5 - 5"), 0.0);
    }

    #[test]
    fn chained_subtractions() {
        assert_approx_eq!(eval("5 - 4 - 3 - 2"), -4.0);
    }

    #[test]
    fn chained_addition_and_subtraction() {
        assert_approx_eq!(eval(" 1 + 2 - 3 + 4 - 5 + 6 - 7 - 8 + 9 + 10"), 9.0);
    }

    #[test]
    fn basic_multiplication() {
        assert_approx_eq!(eval("5 * 6"), 30.0);
    }

    #[test]
    fn chained_multiplications() {
        assert_approx_eq!(eval("1 * 2 * 3"), 6.0);
    }

    #[test]
    fn basic_division() {
        assert_approx_eq!(eval("18 / 3"), 6.0);
    }

    #[test]
    fn chained_divisions() {
        assert_approx_eq!(eval("10 / 5 / 2"), 1.0);
    }

    #[test]
    fn chained_multiplication_and_division() {
        assert_approx_eq!(eval("1 * 2 * 3 / 2 * 3 * 4 / 6"), 6.0);
    }

    #[test]
    fn basic_power() {
        assert_approx_eq!(eval("2 ^ 3"), 8.0);
    }

    #[test]
    fn basic_factorial() {
        assert_approx_eq!(eval("fac(5)"), 120.0);
    }

    #[test]
    fn factorial_of_factorial() {
        assert_approx_eq!(eval("fac(fac(3))"), 720.0);
    }

    #[test]
    fn basic_modulo() {
        assert_approx_eq!(eval("5 % 2"), 1.0);
    }

    #[test]
    fn modulo_precedence_with_plusminus() {
        assert_approx_eq!(eval("1 + 5 % 2"), 2.0);
    }

    #[test]
    fn modulo_precedence_with_muldiv() {
        assert_approx_eq!(eval("2 * 3 % 4"), 2.0);
    }

    #[test]
    fn operator_precedence_with_muldiv_and_plusminus() {
        assert_approx_eq!(eval("5 + 3 * 2"), 11.0);
    }

    #[test]
    fn parentheses() {
        assert_approx_eq!(eval("3 - (2 - 1)"), 2.0);
    }

    #[test]
    fn nested_parentheses() {
        assert_approx_eq!(eval("2 * (1 - (8 - 6 + (5 - 3) * 2))"), -10.0);
    }

    #[test]
    fn number_negation() {
        assert_approx_eq!(eval("-4"), -4.0);
    }

    #[test]
    fn expression_negation() {
        assert_approx_eq!(eval("-(1 + 3)"), -4.0);
    }

    #[test]
    fn absolute_value() {
        assert_approx_eq!(eval("abs(-5)"), 5.0);
    }

    #[test]
    fn nested_absolutes() {
        assert_approx_eq!(eval("abs(1 - abs(5 - 10))"), 4.0);
    }

    #[test]
    fn operator_precedence_with_muldiv_and_factorial() {
        assert_approx_eq!(eval("fac(3) * 8"), 48.0);
    }

    #[test]
    fn parentheses_override_precedence() {
        assert_approx_eq!(eval("(10 - 5) * 6"), 30.0);
    }

    #[test]
    fn complex_expression() {
        assert_approx_eq!(
            eval("sqrt(1) * ((fac(3) + (15 * (abs(8^2 - fac(2))) + 3^2) - ((1 + 2) * (3 + 4) / (1 + 2))) - (-5 + 2))"),
            941.0
        );
    }

    #[test]
    fn operations_with_floats() {
        assert_approx_eq!(eval("1 + 2.4 * 5 - 1.1 ^ 2"), 11.79);
    }

    #[test]
    #[should_panic]
    fn unsupported_factorial_with_floats() {
        eval("fac(3.1)");
    }
}
