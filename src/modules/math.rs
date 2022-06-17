use crate::{environment::Environment, error::error, expr::Expr, interpreter::eval_expr};

macro_rules! operator {
    ($name:expr, $arity:expr, $op:tt) => {
        Expr::native_callable($name, $arity, |args, env| -> Expr {
            let resolved: Vec<Expr> = args.iter().map(|expr| eval_expr(expr, env)).collect();
            match resolved[..] {
                [Expr::Number(a), Expr::Number(b)] => Expr::number(*a $op *b),
                _ => error(format!("Cannot call {} on non-number types", $name)),
            }
        })
    };
}

pub fn math_module() -> Environment<'static, Expr> {
    let mut env = Environment::new();

    env.define("+", operator!("+", 2, +));
    env.define("-", operator!("+", 2, -));
    env.define("*", operator!("+", 2, *));
    env.define("/", operator!("+", 2, /));

    env
}
