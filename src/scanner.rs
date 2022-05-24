use crate::error::error;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    LeftParen,
    RightParen,
    String(String),
    EndOfFile,
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: i32,
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

pub fn scan(input: &str) -> Vec<Token> {
    Scanner::new(input).scan_tokens().to_vec()
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

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::EndOfFile,
            lexeme: String::new(),
            line: self.line,
        });

        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            Some('(') => self.add_token(TokenType::LeftParen),
            Some(')') => self.add_token(TokenType::RightParen),
            Some('"') => self.string(),
            Some(' ') | Some('\r') | Some('\t') => (),
            Some('\n') => self.line = self.line + 1,
            _ => error(format!("Unexpected character {:?}", c)),
        }
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text = &self.source[self.start..self.current];
        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text.to_string(),
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

    fn is_at_end(&self) -> bool {
        self.current >= self.source_length
    }

    fn string(&mut self) {
        while self.peek() != Some('"') && !self.is_at_end() {
            if self.peek() == Some('\n') {
                self.line = self.line + 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            error("Unterminated string");
        }

        self.advance();

        let value: String = self.source[self.start+1..self.current-1].to_string();
        self.add_token(TokenType::String(value));
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_simple_tokenization() {
        let result = scan("()");

        assert_eq!(result[0].token_type, TokenType::LeftParen);
        assert_eq!(result[0].lexeme, "(");
        assert_eq!(result[1].token_type, TokenType::RightParen);
        assert_eq!(result[1].lexeme, ")");
        assert_eq!(result[2].token_type, TokenType::EndOfFile);
    }

    #[test]
    fn test_tokenizes_strings() {
        let result =  scan("\"Test string 1\" \"Test\nstring 2\"");
        
        assert_eq!(result[0].token_type, TokenType::String("Test string 1".to_string()));
        assert_eq!(result[1].token_type, TokenType::String("Test\nstring 2".to_string()));
    }

    #[test]
    fn test_ignores_whitespace() {
        let result = scan("( \r \t \n )");

        assert_eq!(result[0].token_type, TokenType::LeftParen);
        assert_eq!(result[1].token_type, TokenType::RightParen);
        assert_eq!(result[2].token_type, TokenType::EndOfFile);
    }

    #[test]
    fn test_tracks_line() {
        let result = scan("(\n \"\n\")");

        assert_eq!(result[0].line, 1);
        assert_eq!(result[2].line, 3);
    }
}
