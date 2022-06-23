use crate::{
    environment::{Environment, EnvironmentRef},
    error::HError,
    expr::{Arity, Expr},
    interpreter::resolve_args,
};

fn print_resolved_exprs(exprs: &[Expr], env: EnvironmentRef) -> Result<Expr, HError> {
    for expr in resolve_args(exprs, env)? {
        print!("{}", expr);
    }
    Ok(Expr::Nil)
}

pub fn io_module() -> Environment {
    let mut env = Environment::new();

    env.define(
        "print",
        Expr::native_fn(
            "print",
            Arity::Range(0, usize::MAX),
            |args: &[Expr], env: EnvironmentRef| -> Result<Expr, HError> {
                print_resolved_exprs(args, env)?;
                Ok(Expr::Nil)
            },
        ),
    );

    env.define(
        "println",
        Expr::native_fn(
            "println",
            Arity::Range(0, usize::MAX),
            |args: &[Expr], env: EnvironmentRef| -> Result<Expr, HError> {
                print_resolved_exprs(args, env)?;
                println!();
                Ok(Expr::Nil)
            },
        ),
    );

    env
}
