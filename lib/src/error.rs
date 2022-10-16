use crate::expr::{Arity, Expr};

#[derive(Debug, PartialEq, Clone)]
pub enum HError {
    UnboundVar(String),
    UnboundMethod(String),
    UnexpectedForm(String, Expr),
    NotAFunction(String),
    NotAMethod(String),
    InvalidEmptyList(String),
    SetUninitializedVar(String),
    InvalidArity(String, Arity),
    InvalidType(String, Expr), // Fn being called, violating Expr
    ParseError(String),
    ScannerError(String),
    EnvironmentNotFound,
}
