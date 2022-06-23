use crate::expr::{Arity, Expr};

#[derive(Debug, PartialEq, Clone)]
pub enum HError {
    UnboundVar(String),
    UnexpectedForm(Expr),
    NotAFunction(String),
    InvalidEmptyList(String),
    SetUninitializedVar(String),
    InvalidArity(String, Arity),
    InvalidType(String, Expr), // Fn being called, violating Expr
    ParseError(String),
    ScannerError(String),
    EnvironmentNotFound,
}
