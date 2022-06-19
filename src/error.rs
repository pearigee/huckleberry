use crate::expr::Expr;

pub fn error<S: Into<String>>(message: S) -> ! {
    panic!("{}", message.into());
}

#[derive(Debug, PartialEq)]
pub enum HError {
    UnboundVar(String),
    UnexpectedForm(Expr),
    EnvironmentNotFound,
}
