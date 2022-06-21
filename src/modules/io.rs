use crate::{
    environment::{Environment, EnvironmentRef},
    error::HError,
    expr::{Arity, Expr},
    interpreter::eval_expr,
};

fn print_resolved_exprs(exprs: &[Expr], env: EnvironmentRef) -> Result<Expr, HError> {
    let resolved_args = exprs.iter()
                    .map(|expr| eval_expr(expr, env.clone_ref()));
    for expr in resolved_args {
        print!("{}", expr?);
    }
    Ok(Expr::Nil)
}

pub fn io_module() -> Environment {
    let mut env = Environment::new();

    env.define(
        "print",
        Expr::native_callable(
            "print",
            Arity::Range(0,usize::MAX),
            |args: &[Expr], env: EnvironmentRef| -> Result<Expr, HError> {
                print_resolved_exprs(args, env)?;
                Ok(Expr::Nil)
            },
        ),
    );

    env.define(
        "println",
        Expr::native_callable(
            "println",
            Arity::Range(0,usize::MAX),
            |args: &[Expr], env: EnvironmentRef| -> Result<Expr, HError> {
                print_resolved_exprs(args, env)?;
                println!();
                Ok(Expr::Nil)
            },
        ),
    );

    env
}

