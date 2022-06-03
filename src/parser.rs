use crate::{
    error::error,
    scanner::{scan, Token, TokenType},
};

/// program => list*
/// expression => atom | list
/// list => "( expression* )"
/// atom => number | string | symbol

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    List(Box<Cons>),
    Number(f64),
    String(String),
    Symbol(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cons {
    first: Expression,
    rest: Vec<Expression>,
}

impl Cons {
    pub fn new(first: Expression, rest: Vec<Expression>) -> Cons {
        Cons { first, rest }
    }
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
    debug: bool,
}

pub fn parse(input: &str) -> Vec<Expression> {
    let tokens = scan(input);
    Parser::new(tokens).parse()
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens.to_owned(),
            current: 0,
            debug: true,
        }
    }

    fn parse(&mut self) -> Vec<Expression> {
        self.parse_program()
    }

    fn parse_program(&mut self) -> Vec<Expression> {
        let mut program: Vec<Expression> = Vec::new();
        while !self.is_at_end() {
            program.push(self.parse_expression());
        }
        program
    }

    fn parse_expression(&mut self) -> Expression {
        let token = self.peek();

        match token.token_type {
            TokenType::LeftParen => Expression::List(Box::new(self.parse_list())),
            TokenType::Number(value) => {
                self.advance();
                Expression::Number(value)
            }
            TokenType::String(value) => {
                self.advance();
                Expression::String(value.to_owned())
            }
            TokenType::Symbol(value) => {
                self.advance();
                Expression::Symbol(value.to_owned())
            }
            _ => error(format!(
                "Unexpected token {:?} in expression at line {}",
                token, token.line
            )),
        }
    }

    fn parse_list(&mut self) -> Cons {
        let opening_paren = self.match_token(&TokenType::LeftParen);
        let mut expressions: Vec<Expression> = Vec::new();
        while !self.check(&TokenType::RightParen) {
            expressions.push(self.parse_expression());
        }
        self.match_token(&TokenType::RightParen);

        self.debug(format!("LIST {:?}", expressions));
        if let Some((first, rest)) = expressions.split_first() {
            Cons {
                first: first.to_owned(),
                rest: rest.to_owned(),
            }
        } else {
            error(format!("Empty list at {}", opening_paren.line))
        }
    }

    fn match_token(&mut self, token_type: &TokenType) -> Token {
        if self.check(token_type) {
            return self.advance();
        }
        error(format!(
            "Expected {:?} at line {:?}",
            token_type,
            self.peek().line
        ))
    }

    fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        return self.peek().token_type == *token_type;
    }

    fn peek(&self) -> Token {
        return self.tokens[self.current].to_owned();
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current = self.current + 1;
        }
        let consumed = self.previous();
        self.debug(format!("CONSUMED {:?}", consumed));
        consumed
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1].to_owned()
    }

    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::EndOfFile
    }

    fn debug(&self, print: String) {
        if self.debug {
            println!("{}", print);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parses_expression() {
        let result = parse("(+ 1 1)");

        assert_eq!(
            result[0],
            Expression::List(Box::new(Cons::new(
                Expression::Symbol("+".to_string()),
                vec![Expression::Number(1.), Expression::Number(1.)]
            )))
        );
    }
}
