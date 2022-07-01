use crate::{
    env::{Env, EnvRef},
    error::HError,
    expr::{Arity, Expr},
    interpreter::resolve_args,
};

fn print_resolved_exprs(exprs: &[Expr], env: EnvRef) -> Result<Expr, HError> {
    for expr in resolve_args(exprs, env)? {
        print!("{}", expr);
    }
    Ok(Expr::Nil)
}

pub fn io_module() -> Env {
    let mut env = Env::new();

    env.defn(
        "print",
        Arity::Range(0, usize::MAX),
        |args: &[Expr], env: EnvRef| -> Result<Expr, HError> {
            print_resolved_exprs(args, env)?;
            Ok(Expr::Nil)
        },
    );

    env.defn(
        "println",
        Arity::Range(0, usize::MAX),
        |args: &[Expr], env: EnvRef| -> Result<Expr, HError> {
            print_resolved_exprs(args, env)?;
            println!();
            Ok(Expr::Nil)
        },
    );

    env
}
