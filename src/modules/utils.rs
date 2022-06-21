use crate::{environment::EnvironmentRef, error::HError, expr::Expr, interpreter::eval_expr};

pub fn resolve_args(args: &[Expr], env: EnvironmentRef) -> Result<Vec<Expr>, HError> {
    let mut result = Vec::new();
    for expr in args {
        result.push(eval_expr(expr, env.clone_ref())?);
    }
    Ok(result)
}
