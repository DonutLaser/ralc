use super::tokenizer::Token;

struct Tokens {
    tokens: Vec<Token>,
    current_token: usize,
}

impl Tokens {
    pub fn new(data: Vec<Token>) -> Tokens {
        Tokens {
            tokens: data,
            current_token: 0,
        }
    }

    pub fn eat(&mut self) {
        self.current_token += 1;
    }

    pub fn peek(&self) -> Token {
        if self.current_token >= self.tokens.len() {
            Token::EOF
        } else {
            self.tokens[self.current_token]
        }
    }
}

fn is_float(value: f64) -> bool {
    value.to_string().contains('.')
}

fn expr(tokens: &mut Tokens) -> f64 {
    let mut value = modulo(tokens);

    let mut token = tokens.peek();
    while token == Token::PLUS || token == Token::MINUS {
        match token {
            Token::PLUS => {
                tokens.eat();
                value += modulo(tokens);
            }
            Token::MINUS => {
                tokens.eat();
                value -= modulo(tokens);
            }
            _ => panic!("Expected + or -, got {:?}", token),
        }

        token = tokens.peek();
    }

    value
}

fn modulo(tokens: &mut Tokens) -> f64 {
    let mut value = term(tokens);

    let mut token = tokens.peek();
    while token == Token::MOD {
        match token {
            Token::MOD => {
                tokens.eat();
                value %= term(tokens);
            }
            _ => panic!("Expected %, got {:?}", token),
        }

        token = tokens.peek();
    }

    value
}

fn term(tokens: &mut Tokens) -> f64 {
    let mut value = power(tokens);

    let mut token = tokens.peek();
    while token == Token::MUL || token == Token::DIV {
        match token {
            Token::MUL => {
                tokens.eat();
                value *= power(tokens);
            }
            Token::DIV => {
                tokens.eat();
                value /= power(tokens);
            }
            _ => panic!("Expected * or /, got {:?}", token),
        }

        token = tokens.peek();
    }

    value
}

fn power(tokens: &mut Tokens) -> f64 {
    let mut value = factor(tokens);

    let mut token = tokens.peek();
    while token == Token::POW {
        match token {
            Token::POW => {
                tokens.eat();
                value = value.powf(factor(tokens));
            }
            _ => panic!("Expected ^, got {:?}", token),
        }

        token = tokens.peek();
    }

    value
}

fn factor(tokens: &mut Tokens) -> f64 {
    let token = tokens.peek();
    match token {
        Token::NUM(n) => {
            tokens.eat();
            n
        }
        Token::MINUS => {
            tokens.eat();
            let value = factor(tokens);

            -value
        }
        Token::LPAREN => {
            tokens.eat();
            let value = expr(tokens);
            let token = tokens.peek();
            if token != Token::RPAREN {
                panic!("Expected ), got {:?}", token);
            }

            tokens.eat();
            value
        }
        Token::SQRT => {
            tokens.eat();
            let token = tokens.peek();
            if token != Token::LPAREN {
                panic!("Expected (, got {:?}", token);
            }
            let value = factor(tokens);

            (value).sqrt()
        }
        Token::ABS => {
            tokens.eat();
            let token = tokens.peek();
            if token != Token::LPAREN {
                panic!("Expected (, got {:?}", token);
            }
            let value = factor(tokens);

            value.abs()
        }
        Token::FAC => {
            tokens.eat();
            let token = tokens.peek();
            if token != Token::LPAREN {
                panic!("Expected (, got {:?}", token);
            }
            let value = factor(tokens);

            if is_float(value) {
                panic!("Factorial for a decimal value is not supported");
            }

            (2..=value as i32).product::<i32>() as f64
        }
        _ => panic!("Did not expect {:?}", token),
    }
}

pub fn parse(tokens: Vec<Token>) -> f64 {
    let mut token_list = Tokens::new(tokens);
    expr(&mut token_list)
}
