use crate::{error::HError, expr::Expr};

pub fn check_num(expr: &Expr, fun_name: &str) -> Result<f64, HError> {
    match expr {
        Expr::Number(val) => Ok(**val),
        _ => Err(HError::InvalidType(fun_name.to_string(), expr.clone())),
    }
}

pub fn is_truthy(expr: &Expr) -> bool {
    match expr {
        Expr::Nil | Expr::Boolean(false) => false,
        _ => true,
    }
}
