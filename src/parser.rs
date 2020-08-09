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

fn expr(tokens: &mut Tokens) -> i32 {
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

fn modulo(tokens: &mut Tokens) -> i32 {
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

fn term(tokens: &mut Tokens) -> i32 {
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

fn power(tokens: &mut Tokens) -> i32 {
    let mut value = factor(tokens);

    let mut token = tokens.peek();
    while token == Token::POW {
        match token {
            Token::POW => {
                tokens.eat();
                value = value.pow(factor(tokens) as u32);
            }
            _ => panic!("Expected ^, got {:?}", token),
        }

        token = tokens.peek();
    }

    value
}

fn factor(tokens: &mut Tokens) -> i32 {
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

            (value as f64).sqrt() as i32
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

            (2..=value).product()
        }
        _ => panic!("Did not expect {:?}", token),
    }
}

pub fn parse(tokens: Vec<Token>) -> i32 {
    let mut token_list = Tokens::new(tokens);
    expr(&mut token_list)
}
