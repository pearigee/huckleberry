use crate::{environment::EnvironmentRef, error::HError, expr::Expr, interpreter::eval_expr};

pub fn resolve_args(args: &[Expr], env: EnvironmentRef) -> Result<Vec<Expr>, HError> {
    let mut result = Vec::new();
    for expr in args {
        result.push(eval_expr(expr, env.clone_ref())?);
    }
    Ok(result)
}

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