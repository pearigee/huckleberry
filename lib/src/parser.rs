use std::collections::BTreeMap;

use crate::{
    error::HError,
    expr::Expr,
    scanner::{scan, Token, TokenType},
};

struct Parser {
    tokens: Vec<Token>,
    current: usize,
    debug: bool,
}

pub fn parse(input: &str) -> Result<Vec<Expr>, HError> {
    let tokens = scan(input)?;
    Parser::new(tokens).parse()
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens.to_owned(),
            current: 0,
            debug: false,
        }
    }

    fn parse(&mut self) -> Result<Vec<Expr>, HError> {
        let mut program: Vec<Expr> = Vec::new();
        while !self.is_at_end() {
            program.push(self.parse_expression()?);
        }
        Ok(program)
    }

    fn parse_expression(&mut self) -> Result<Expr, HError> {
        let token = self.peek();

        match token.token_type {
            TokenType::LeftParen => Ok(Expr::List(
                self.parse_vector(TokenType::LeftParen, TokenType::RightParen)?,
            )),
            TokenType::LeftAngle => Ok(Expr::MethodList(
                self.parse_vector(TokenType::LeftAngle, TokenType::RightAngle)?,
            )),
            TokenType::LeftSquare => Ok(Expr::Vector(
                self.parse_vector(TokenType::LeftSquare, TokenType::RightSquare)?,
            )),
            TokenType::LeftCurly => Ok(Expr::Map(self.parse_map()?)),
            TokenType::Number(value) => {
                self.advance();
                Ok(Expr::number(value))
            }
            TokenType::Boolean(value) => {
                self.advance();
                Ok(Expr::boolean(value))
            }
            TokenType::String(value) => {
                self.advance();
                Ok(Expr::string(&value))
            }
            TokenType::Keyword(value) => {
                self.advance();
                Ok(Expr::keyword(&value))
            }
            TokenType::Symbol(value) => {
                self.advance();
                Ok(Expr::symbol(&value))
            }
            TokenType::Ampersand => {
                self.advance();
                Ok(Expr::ampersand())
            }
            TokenType::Nil => {
                self.advance();
                Ok(Expr::nil())
            }
            token => Err(HError::ParseError(format!("Unexpected token {:?}", token))),
        }
    }

    fn parse_vector(
        &mut self,
        open_token: TokenType,
        close_token: TokenType,
    ) -> Result<Vec<Expr>, HError> {
        self.match_token(open_token)?;
        let mut expressions: Vec<Expr> = Vec::new();
        while !self.check(&close_token) {
            expressions.push(self.parse_expression()?);
        }
        self.match_token(close_token)?;

        self.debug(format!("VECTOR {:?}", expressions));
        Ok(expressions)
    }

    fn parse_map(&mut self) -> Result<BTreeMap<Expr, Expr>, HError> {
        let initial_token = self.match_token(TokenType::LeftCurly)?;
        let mut expressions: Vec<Expr> = Vec::new();
        while !self.check(&TokenType::RightCurly) {
            expressions.push(self.parse_expression()?);
        }
        self.match_token(TokenType::RightCurly)?;

        if expressions.len() % 2 == 0 {
            let mut map: BTreeMap<Expr, Expr> = BTreeMap::new();
            for pair in expressions.chunks(2) {
                map.insert(pair[0].to_owned(), pair[1].to_owned());
            }
            Ok(map)
        } else {
            Err(HError::ParseError(format!(
                "A map must have an even number of elements! Map opened at line {}.",
                initial_token.line
            )))
        }
    }

    fn match_token(&mut self, token_type: TokenType) -> Result<Token, HError> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(HError::ParseError(format!(
                "Expected {:?} at line {:?}",
                &token_type,
                self.peek().line
            )))
        }
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
        let result = parse("(+ 1 1)").unwrap();

        assert_eq!(
            result[0],
            Expr::list(&[Expr::symbol("+"), Expr::number(1.), Expr::number(1.)])
        );
    }

    #[test]
    fn test_parses_nested_expression() {
        let result = parse("(+ 1.25 <1 * 2>)").unwrap();

        assert_eq!(
            result[0],
            Expr::list(&[
                Expr::symbol("+"),
                Expr::number(1.25),
                Expr::method_list(&[Expr::number(1.), Expr::symbol("*"), Expr::number(2.)])
            ])
        );
    }

    #[test]
    fn test_parses_method_list() {
        let result = parse("<1 + 2>").unwrap();

        assert_eq!(
            result[0],
            Expr::method_list(&[
                Expr::number(1.),
                Expr::symbol("+"),
                Expr::number(2.)
            ])
        );
    }

    #[test]
    fn test_parses_vector() {
        let result = parse("[1 true & nil]").unwrap();

        assert_eq!(
            result[0],
            Expr::vector(&[
                Expr::number(1.),
                Expr::boolean(true),
                Expr::Ampersand,
                Expr::nil()
            ])
        );
    }

    #[test]
    fn test_parses_map() {
        let result = parse("{:key \"value\" [true] false}").unwrap();

        assert_eq!(
            result[0],
            Expr::map(&[
                (Expr::keyword(":key"), Expr::string("value")),
                (Expr::vector(&[Expr::boolean(true)]), Expr::boolean(false)),
            ])
        );
    }
}
