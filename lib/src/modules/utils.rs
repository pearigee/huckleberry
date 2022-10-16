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

pub fn method_id(args: &[Expr]) -> String {
    let id: Vec<&Expr> = args.iter().step_by(2).collect();

    id.iter()
        .map(|e| format!("{}", e))
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn method_args(args: &[Expr]) -> Vec<Expr> {
    args.iter().skip(1).step_by(2).map(|a| a.clone()).collect()
}
