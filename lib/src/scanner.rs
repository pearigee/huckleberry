use crate::error::HError;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    LeftCurly,
    RightCurly,
    LeftSquare,
    RightSquare,
    LeftAngle,
    RightAngle,
    String(String),
    Number(f64),
    Symbol(String),
    Keyword(String),
    Boolean(bool),
    Ampersand,
    Nil,
    EndOfFile,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub line: i32,
}

#[derive(Debug)]
struct Scanner {
    start: usize,
    current: usize,
    line: i32,
    tokens: Vec<Token>,
    source: String,
    source_length: usize,
}

pub fn scan(input: &str) -> Result<Vec<Token>, HError> {
    Ok(Scanner::new(input).scan_tokens()?)
}

impl Scanner {
    pub fn new(input: &str) -> Scanner {
        Scanner {
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
            source: input.to_string(),
            source_length: input.chars().count(),
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, HError> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens.push(Token {
            token_type: TokenType::EndOfFile,
            line: self.line,
        });

        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<(), HError> {
        let c = self.advance();

        match c {
            Some('(') => self.add_token(TokenType::LeftParen),
            Some(')') => self.add_token(TokenType::RightParen),
            Some('<') => self.add_token(TokenType::LeftAngle),
            Some('>') => self.add_token(TokenType::RightAngle),
            Some('{') => self.add_token(TokenType::LeftCurly),
            Some('}') => self.add_token(TokenType::RightCurly),
            Some('[') => self.add_token(TokenType::LeftSquare),
            Some(']') => self.add_token(TokenType::RightSquare),
            Some('&') => self.add_token(TokenType::Ampersand),
            Some('"') => self.string()?,
            Some(':') => self.keyword(),
            Some(' ') | Some('\r') | Some('\t') => (),
            Some('\n') => self.line = self.line + 1,
            _ => {
                if Scanner::is_digit(c) {
                    self.number()?;
                } else if Scanner::is_alpha(c) {
                    // This includes booleans and nil.
                    self.symbol();
                } else {
                    return Err(HError::ScannerError(format!(
                        "Unexpected character {:?}",
                        c
                    )));
                }
            }
        }

        Ok(())
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token {
            token_type: token_type,
            line: self.line,
        });
    }

    fn advance(&mut self) -> Option<char> {
        let c = self.source.chars().nth(self.current as usize);
        self.current = self.current + 1;
        c
    }

    fn peek(&self) -> Option<char> {
        if self.is_at_end() {
            return None;
        }
        self.source.chars().nth(self.current)
    }

    fn peek_next(&self) -> Option<char> {
        let next = self.current + 1;
        if next >= self.source_length {
            return None;
        }
        self.source.chars().nth(next)
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source_length
    }

    fn string(&mut self) -> Result<(), HError> {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line = self.line + 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(HError::ScannerError("Unterminated string".to_string()));
        }

        self.advance();

        let value: String = self.source[self.start + 1..self.current - 1].to_string();
        self.add_token(TokenType::String(value));
        Ok(())
    }

    fn number(&mut self) -> Result<(), HError> {
        while Scanner::is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == Some('.') && Scanner::is_digit(self.peek_next()) {
            // Consume the '.'
            self.advance();

            while Scanner::is_digit(self.peek()) {
                self.advance();
            }
        }

        let value = self.source[self.start..self.current].parse::<f64>();
        match value {
            Ok(v) => self.add_token(TokenType::Number(v)),
            Err(_) => return Err(HError::ScannerError("Invalid number".to_string())),
        };
        Ok(())
    }

    fn symbol(&mut self) {
        while Scanner::is_alpha_numeric(self.peek()) {
            self.advance();
        }
        // Allow symbols to end in optional : for method arguments.
        if self.peek() == Some(':') {
            self.advance();
        }

        let result = self.source[self.start..self.current].to_string();

        if result == "true" {
            self.add_token(TokenType::Boolean(true))
        } else if result == "false" {
            self.add_token(TokenType::Boolean(false))
        } else if result == "nil" {
            self.add_token(TokenType::Nil)
        } else {
            // Purge optional : to simplify argument handling.
            self.add_token(TokenType::Symbol(if result.ends_with(":") {
                self.source[self.start..self.current - 1].to_string()
            } else {
                self.source[self.start..self.current].to_string()
            }))
        }
    }

    fn keyword(&mut self) {
        while Scanner::is_alpha_numeric(self.peek()) {
            self.advance();
        }

        self.add_token(TokenType::Keyword(
            self.source[self.start..self.current].to_string(),
        ))
    }

    fn is_digit(c: Option<char>) -> bool {
        match c {
            Some(value) => value >= '0' && value <= '9',
            _ => false,
        }
    }

    /// Checks if the character could create a valid symbol.
    /// This includes all basic math operators and _, !, and ?.
    fn is_alpha(c: Option<char>) -> bool {
        match c {
            Some(value) => {
                (value >= 'a' && value <= 'z')
                    || (value >= 'A' && value <= 'Z')
                    || value == '_'
                    || value == '-'
                    || value == '*'
                    || value == '+'
                    || value == '!'
                    || value == '?'
                    || value == '/'
                    || value == '='
            }
            _ => false,
        }
    }

    fn is_alpha_numeric(c: Option<char>) -> bool {
        Scanner::is_alpha(c) || Scanner::is_digit(c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizes_fn_call() {
        let result = scan("(+ 1 2)").unwrap();

        assert_eq!(result[0].token_type, TokenType::LeftParen);
        assert_eq!(result[1].token_type, TokenType::Symbol("+".to_string()));
        assert_eq!(result[2].token_type, TokenType::Number(1.));
        assert_eq!(result[3].token_type, TokenType::Number(2.));
        assert_eq!(result[4].token_type, TokenType::RightParen);
    }

    #[test]
    fn test_tokenizes_values_without_whitespace() {
        let result = scan("(+ abc\":hello\":hello)").unwrap();

        assert_eq!(result[0].token_type, TokenType::LeftParen);
        assert_eq!(result[1].token_type, TokenType::Symbol("+".to_string()));
        assert_eq!(result[2].token_type, TokenType::Symbol("abc".to_string()));
        assert_eq!(
            result[3].token_type,
            TokenType::String(":hello".to_string())
        );
        assert_eq!(
            result[4].token_type,
            TokenType::Keyword(":hello".to_string())
        );
        assert_eq!(result[5].token_type, TokenType::RightParen);
    }

    #[test]
    fn test_tokenizes_parens() {
        let result = scan("()").unwrap();

        assert_eq!(result[0].token_type, TokenType::LeftParen);
        assert_eq!(result[1].token_type, TokenType::RightParen);
        assert_eq!(result[2].token_type, TokenType::EndOfFile);
    }

    #[test]
    fn test_tokenizes_curly_braces() {
        let result = scan("{}").unwrap();

        assert_eq!(result[0].token_type, TokenType::LeftCurly);
        assert_eq!(result[1].token_type, TokenType::RightCurly);
        assert_eq!(result[2].token_type, TokenType::EndOfFile);
    }

    #[test]
    fn test_tokenizes_square_braces() {
        let result = scan("[]").unwrap();

        assert_eq!(result[0].token_type, TokenType::LeftSquare);
        assert_eq!(result[1].token_type, TokenType::RightSquare);
        assert_eq!(result[2].token_type, TokenType::EndOfFile);
    }

    #[test]
    fn test_tokenizes_booleans() {
        let result = scan("true false").unwrap();

        assert_eq!(result[0].token_type, TokenType::Boolean(true));
        assert_eq!(result[1].token_type, TokenType::Boolean(false));
        assert_eq!(result[2].token_type, TokenType::EndOfFile);
    }

    #[test]
    fn test_tokenizes_keywords() {
        let result = scan(":hello :world").unwrap();

        assert_eq!(
            result[0].token_type,
            TokenType::Keyword(":hello".to_string())
        );
        assert_eq!(
            result[1].token_type,
            TokenType::Keyword(":world".to_string())
        );
    }

    #[test]
    fn test_tokenizes_nil() {
        let result = scan("nil").unwrap();

        assert_eq!(result[0].token_type, TokenType::Nil);
    }

    #[test]
    fn test_tokenizes_ampersand() {
        let result = scan("&").unwrap();

        assert_eq!(result[0].token_type, TokenType::Ampersand);
    }

    #[test]
    fn test_tokenizes_angle_brackets() {
        let result = scan("<>").unwrap();

        assert_eq!(result[0].token_type, TokenType::LeftAngle);
        assert_eq!(result[1].token_type, TokenType::RightAngle);
    }

    #[test]
    fn test_tokenizes_strings() {
        let result = scan("\"Test string 1\" \"Test\nstring 2\"").unwrap();

        assert_eq!(
            result[0].token_type,
            TokenType::String("Test string 1".to_string())
        );
        assert_eq!(
            result[1].token_type,
            TokenType::String("Test\nstring 2".to_string())
        );
    }

    #[test]
    fn test_tokenizes_numbers() {
        let result = scan("1 2.34 56.78").unwrap();

        assert_eq!(result[0].token_type, TokenType::Number(1.));
        assert_eq!(result[1].token_type, TokenType::Number(2.34));
        assert_eq!(result[2].token_type, TokenType::Number(56.78));
    }

    #[test]
    fn test_tokenizes_symbols() {
        let input = "+ - / * = ? ! is_symbol? set! hello";
        let tokens = scan(input).unwrap();
        let names = input.split(' ').collect::<Vec<&str>>();

        for i in 0..tokens.len() - 1 {
            assert_eq!(
                tokens[i].token_type,
                TokenType::Symbol(names[i].to_string())
            )
        }
    }

    #[test]
    fn test_tokenizes_method_args() {
        let input = "hello:";
        let tokens = scan(input).unwrap();

        assert_eq!(tokens[0].token_type, TokenType::Symbol("hello".to_string()));
    }

    #[test]
    fn test_ignores_whitespace() {
        let result = scan("( \r \t \n )").unwrap();

        assert_eq!(result[0].token_type, TokenType::LeftParen);
        assert_eq!(result[1].token_type, TokenType::RightParen);
        assert_eq!(result[2].token_type, TokenType::EndOfFile);
    }

    #[test]
    fn test_tracks_line() {
        let result = scan("(\n \"\n\")").unwrap();

        assert_eq!(result[0].line, 1);
        assert_eq!(result[2].line, 3);
    }
}
