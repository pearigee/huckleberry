use std::collections::BTreeMap;

use ordered_float::OrderedFloat;

use crate::{
    error::error,
    scanner::{scan, Token, TokenType},
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expr {
    List(Vec<Expr>),
    Number(OrderedFloat<f64>),
    Boolean(bool),
    String(String),
    Keyword(String),
    Symbol(String),
    Vector(Vec<Expr>),
    Map(BTreeMap<Expr, Expr>),
    Nil,
}

impl Expr {
    pub fn number(value: f64) -> Expr {
        Expr::Number(OrderedFloat(value))
    }

    pub fn boolean(value: bool) -> Expr {
        Expr::Boolean(value)
    }

    pub fn string(value: &str) -> Expr {
        Expr::String(value.to_string())
    }

    pub fn symbol(value: &str) -> Expr {
        Expr::Symbol(value.to_string())
    }

    pub fn keyword(value: &str) -> Expr {
        Expr::Keyword(value.to_string())
    }

    pub fn vector(exprs: &[Expr]) -> Expr {
        Expr::Vector(exprs.to_vec())
    }

    pub fn list(exprs: &[Expr]) -> Expr {
        Expr::List(exprs.to_vec())
    }

    pub fn nil() -> Expr {
        Expr::Nil
    }

    pub fn map(exprs: &[(Expr, Expr)]) -> Expr {
       let mut map = BTreeMap::new();
       for pair in exprs.iter() {
           map.insert(pair.0.to_owned(), pair.1.to_owned());
       }
       Expr::Map(map)
    }
}

struct Parser {
    tokens: Vec<Token>,
    current: usize,
    debug: bool,
}

pub fn parse(input: &str) -> Vec<Expr> {
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

    fn parse(&mut self) -> Vec<Expr> {
        self.parse_program()
    }

    fn parse_program(&mut self) -> Vec<Expr> {
        let mut program: Vec<Expr> = Vec::new();
        while !self.is_at_end() {
            program.push(self.parse_expression());
        }
        program
    }

    fn parse_expression(&mut self) -> Expr {
        let token = self.peek();

        match token.token_type {
            TokenType::LeftParen => {
                Expr::List(self.parse_vector(TokenType::LeftParen, TokenType::RightParen))
            }
            TokenType::LeftSquare => {
                Expr::Vector(self.parse_vector(TokenType::LeftSquare, TokenType::RightSquare))
            }
            TokenType::LeftCurly => Expr::Map(self.parse_map()),
            TokenType::Number(value) => {
                self.advance();
                Expr::number(value)
            }
            TokenType::Boolean(value) => {
                self.advance();
                Expr::Boolean(value)
            }
            TokenType::String(value) => {
                self.advance();
                Expr::String(value)
            }
            TokenType::Keyword(value) => {
                self.advance();
                Expr::Keyword(value)
            }
            TokenType::Symbol(value) => {
                self.advance();
                Expr::Symbol(value)
            }
            TokenType::Nil => {
                self.advance();
                Expr::Nil
            }
            _ => error(format!(
                "Unexpected token {:?} in expression at line {}",
                token, token.line
            )),
        }
    }

    fn parse_vector(&mut self, open_token: TokenType, close_token: TokenType) -> Vec<Expr> {
        self.match_token(open_token);
        let mut expressions: Vec<Expr> = Vec::new();
        while !self.check(&close_token) {
            expressions.push(self.parse_expression());
        }
        self.match_token(close_token);

        self.debug(format!("VECTOR {:?}", expressions));
        expressions
    }

    fn parse_map(&mut self) -> BTreeMap<Expr, Expr> {
        let initial_token = self.match_token(TokenType::LeftCurly);
        let mut expressions: Vec<Expr> = Vec::new();
        while !self.check(&TokenType::RightCurly) {
            expressions.push(self.parse_expression());
        }
        self.match_token(TokenType::RightCurly);

        if expressions.len() % 2 == 0 {
            let mut map: BTreeMap<Expr, Expr> = BTreeMap::new();
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
            Expr::list(&[
                Expr::symbol("+"),
                Expr::number(1.),
                Expr::number(1.)
            ])
        );
    }

    #[test]
    fn test_parses_nested_expression() {
        let result = parse("(+ 1.25 (/ 3 4))");

        assert_eq!(
            result[0],
            Expr::list(&[
                Expr::symbol("+"),
                Expr::number(1.25),
                Expr::list(&[
                    Expr::symbol("/"),
                    Expr::number(3.),
                    Expr::number(4.)
                ])
            ])
        );
    }

    #[test]
    fn test_parses_vector() {
        let result = parse("[1 true nil]");

        assert_eq!(
            result[0],
            Expr::vector(&[Expr::number(1.), Expr::boolean(true), Expr::nil()])
        );
    }

    #[test]
    fn test_parses_map() {
        let result = parse("{:key \"value\" [true] false}");

        assert_eq!(
            result[0],
            Expr::map(&[
                (
                    Expr::keyword(":key"),
                    Expr::string("value")
                ),
                (
                    Expr::vector(&[Expr::boolean(true)]),
                    Expr::boolean(false)
                ),
            ])
        );
    }
}
