mod parser;
mod tokenizer;

use std::env;
use std::io::{self, Write};

fn eval(expression: &str) -> i32 {
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

    #[test]
    fn basic_addition() {
        assert_eq!(eval("2 + 2"), 4);
    }

    #[test]
    fn chained_additions() {
        assert_eq!(eval("1 + 2 + 3 + 4"), 10);
    }

    #[test]
    fn basic_subtraction() {
        assert_eq!(eval("5 - 5"), 0);
    }

    #[test]
    fn chained_subtractions() {
        assert_eq!(eval("5 - 4 - 3 - 2"), -4);
    }

    #[test]
    fn chained_addition_and_subtraction() {
        assert_eq!(eval(" 1 + 2 - 3 + 4 - 5 + 6 - 7 - 8 + 9 + 10"), 9);
    }

    #[test]
    fn basic_multiplication() {
        assert_eq!(eval("5 * 6"), 30);
    }

    #[test]
    fn chained_multiplications() {
        assert_eq!(eval("1 * 2 * 3"), 6);
    }

    #[test]
    fn basic_division() {
        assert_eq!(eval("18 / 3"), 6);
    }

    #[test]
    fn chained_divisions() {
        assert_eq!(eval("10 / 5 / 2"), 1);
    }

    #[test]
    fn chained_multiplication_and_division() {
        assert_eq!(eval("1 * 2 * 3 / 2 * 3 * 4 / 6"), 6);
    }

    #[test]
    fn basic_power() {
        assert_eq!(eval("2 ^ 3"), 8);
    }

    #[test]
    fn basic_factorial() {
        assert_eq!(eval("fac(5)"), 120);
    }

    #[test]
    fn factorial_of_factorial() {
        assert_eq!(eval("fac(fac(3))"), 720);
    }

    #[test]
    fn basic_modulo() {
        assert_eq!(eval("5 % 2"), 1);
    }

    #[test]
    fn modulo_precedence_with_plusminus() {
        assert_eq!(eval("1 + 5 % 2"), 2);
    }

    #[test]
    fn modulo_precedence_with_muldiv() {
        assert_eq!(eval("2 * 3 % 4"), 2);
    }

    #[test]
    fn operator_precedence_with_muldiv_and_plusminus() {
        assert_eq!(eval("5 + 3 * 2"), 11);
    }

    #[test]
    fn parentheses() {
        assert_eq!(eval("3 - (2 - 1)"), 2);
    }

    #[test]
    fn nested_parentheses() {
        assert_eq!(eval("2 * (1 - (8 - 6 + (5 - 3) * 2))"), -10);
    }

    #[test]
    fn number_negation() {
        assert_eq!(eval("-4"), -4);
    }

    #[test]
    fn expression_negation() {
        assert_eq!(eval("-(1 + 3)"), -4);
    }

    #[test]
    fn absolute_value() {
        assert_eq!(eval("abs(-5)"), 5);
    }

    #[test]
    fn nested_absolutes() {
        assert_eq!(eval("abs(1 - abs(5 - 10))"), 4);
    }

    #[test]
    fn operator_precedence_with_muldiv_and_factorial() {
        assert_eq!(eval("fac(3) * 8"), 48);
    }

    #[test]
    fn parentheses_override_precedence() {
        assert_eq!(eval("(10 - 5) * 6"), 30);
    }

    #[test]
    fn complex_expression() {
        assert_eq!(
            eval("sqrt(1) * ((fac(3) + (15 * (abs(8^2 - fac(2))) + 3^2) - ((1 + 2) * (3 + 4) / (1 + 2))) - (-5 + 2))"),
            941
        );
    }
}
