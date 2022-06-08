use std::collections::BTreeMap;

use ordered_float::OrderedFloat;

use crate::{
    error::error,
    scanner::{scan, Token, TokenType},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expression {
    List(Vec<Expression>),
    Number(OrderedFloat<f64>),
    Boolean(bool),
    String(String),
    Keyword(String),
    Symbol(String),
    Vector(Vec<Expression>),
    Map(BTreeMap<Expression, Expression>),
}

impl Expression {
    fn from_f64(number: f64) -> Expression {
        Expression::Number(OrderedFloat(number))
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
            TokenType::LeftParen => {
                Expression::List(self.parse_vector(TokenType::LeftParen, TokenType::RightParen))
            }
            TokenType::LeftSquare => {
                Expression::Vector(self.parse_vector(TokenType::LeftSquare, TokenType::RightSquare))
            }
            TokenType::LeftCurly => Expression::Map(self.parse_map()),
            TokenType::Number(value) => {
                self.advance();
                Expression::from_f64(value)
            }
            TokenType::Boolean(value) => {
                self.advance();
                Expression::Boolean(value)
            }
            TokenType::String(value) => {
                self.advance();
                Expression::String(value)
            }
            TokenType::Keyword(value) => {
                self.advance();
                Expression::Keyword(value)
            }
            TokenType::Symbol(value) => {
                self.advance();
                Expression::Symbol(value)
            }
            _ => error(format!(
                "Unexpected token {:?} in expression at line {}",
                token, token.line
            )),
        }
    }

    fn parse_vector(&mut self, open_token: TokenType, close_token: TokenType) -> Vec<Expression> {
        self.match_token(open_token);
        let mut expressions: Vec<Expression> = Vec::new();
        while !self.check(&close_token) {
            expressions.push(self.parse_expression());
        }
        self.match_token(close_token);

        self.debug(format!("VECTOR {:?}", expressions));
        expressions
    }

    fn parse_map(&mut self) -> BTreeMap<Expression, Expression> {
        let initial_token = self.match_token(TokenType::LeftCurly);
        let mut expressions: Vec<Expression> = Vec::new();
        while !self.check(&TokenType::RightCurly) {
            expressions.push(self.parse_expression());
        }
        self.match_token(TokenType::RightCurly);

        if expressions.len() % 2 == 0 {
            let mut map: BTreeMap<Expression, Expression> = BTreeMap::new();
            for pair in expressions.chunks(2) {
                map.insert(pair[0].to_owned(), pair[1].to_owned());
            }
            map
        } else {
            error(format!(
                "A map must have an even number of elements! Map opened at line {}.",
                initial_token.line
            ));
        }
    }

    fn match_token(&mut self, token_type: TokenType) -> Token {
        if self.check(&token_type) {
            return self.advance();
        }
        error(format!(
            "Expected {:?} at line {:?}",
            &token_type,
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
            Expression::List(vec![
                Expression::Symbol("+".to_string()),
                Expression::from_f64(1.),
                Expression::from_f64(1.)
            ])
        );
    }

    #[test]
    fn test_parses_nested_expression() {
        let result = parse("(+ 1.25 (/ 3 4))");

        assert_eq!(
            result[0],
            Expression::List(vec![
                Expression::Symbol("+".to_string()),
                Expression::from_f64(1.25),
                Expression::List(vec![
                    Expression::Symbol("/".to_string()),
                    Expression::from_f64(3.),
                    Expression::from_f64(4.)
                ])
            ])
        );
    }

    #[test]
    fn test_parses_vector() {
        let result = parse("[1 true]");

        assert_eq!(
            result[0],
            Expression::Vector(vec![Expression::from_f64(1.), Expression::Boolean(true)])
        );
    }

    #[test]
    fn test_parses_map() {
        let result = parse("{:key \"value\" [true] false}");

        assert_eq!(
            result[0],
            Expression::Map(BTreeMap::from([
                (
                    Expression::Keyword(":key".to_string()),
                    Expression::String("value".to_string())
                ),
                (
                    Expression::Vector(vec![Expression::Boolean(true)]),
                    Expression::Boolean(false)
                ),
            ]))
        );
    }
}
