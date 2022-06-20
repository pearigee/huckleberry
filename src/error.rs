use crate::expr::{Arity, Expr};

pub fn error<S: Into<String>>(message: S) -> ! {
    panic!("{}", message.into());
}

#[derive(Debug, PartialEq, Clone)]
pub enum HError {
    UnboundVar(String),
    UnexpectedForm(Expr),
    NotAFunction(String),
    SetUninitializedVar(String),
    InvalidArity(String, Arity),
    InvalidType(String, Expr), // Fn being called, violating Expr
    EnvironmentNotFound,
}
