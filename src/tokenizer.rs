#![allow(unused)]
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    NUM(f64),
    PLUS,   // +
    MINUS,  // -
    MUL,    // *
    DIV,    // /
    LPAREN, // (
    RPAREN, // )
    MOD,    // %
    POW,    // ^
    SQRT,   // sqrt
    ABS,    // abs
    FAC,    // fact
    EOF,
}

pub fn tokenize(expr: &str) -> Vec<Token> {
    let mut result: Vec<Token> = Vec::new();
    let mut iter = expr.chars().filter(|c| !c.is_whitespace());

    let mut next = iter.next();
    while next.is_some() {
        let mut c = next.unwrap();
        if c.is_digit(10) {
            let mut n: String = String::new();
            while c.is_digit(10) || c == '.' {
                n.push(c);
                next = iter.next();

                if next.is_none() {
                    break;
                }

                c = next.unwrap();
            }

            result.push(Token::NUM(n.parse::<f64>().unwrap()));
            continue;
        } else if c.is_alphabetic() {
            let mut n: String = String::new();
            while c.is_alphabetic() {
                n.push(c);
                next = iter.next();

                if next.is_none() {
                    break;
                }

                c = next.unwrap();
            }

            match n.as_str() {
                "sqrt" => result.push(Token::SQRT),
                "abs" => result.push(Token::ABS),
                "fac" => result.push(Token::FAC),
                _ => panic!("Unknown operation"),
            }

            continue;
        } else {
            let token = match c {
                '+' => Token::PLUS,
                '-' => Token::MINUS,
                '*' => Token::MUL,
                '/' => Token::DIV,
                '(' => Token::LPAREN,
                ')' => Token::RPAREN,
                '%' => Token::MOD,
                '^' => Token::POW,
                _ => Token::EOF,
            };

            result.push(token);
            next = iter.next();
            continue;
        }
    }

    result
}
