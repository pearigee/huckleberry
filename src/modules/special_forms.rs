use crate::{
    environment::Environment,
    error::error,
    expr::{Arity, Expr}, interpreter::eval_expr,
};

pub fn special_forms_module() -> Environment {
    let mut env = Environment::new();

    env.define(
        "def",
        Expr::native_callable("def", Arity::Count(2), |args, env| -> Expr {
            if args.len() != 2 {
                error("`def` takes two arguments: (def var value)")
            }
            match &args[0] {
                Expr::Symbol(value) => env.define(&value, eval_expr(&args[1], env.clone_ref())),
                invalid => error(format!(
                    "`def` can only assign to a symbol, {:?} is not a symbol",
                    invalid
                )),
            };
            Expr::Nil
        }),
    );

    env
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{environment::Environment, interpreter::eval, modules::math::math_module};

    #[test]
    fn test_def() {
        let mut env = Environment::new();
        env.merge(math_module());
        env.merge(special_forms_module());
        let result = eval("(def a 2) (+ a 1)", env.as_ref());

        assert_eq!(result, Expr::number(3.));
    }

    #[test]
    fn test_def_overwrite() {
        let mut env = Environment::new();
        env.merge(math_module());
        env.merge(special_forms_module());
        let env_ref = env.as_ref();
        eval("(def a 2) (def a (+ a 1))", env_ref.clone_ref());

        assert_eq!(env_ref.get("a"), Ok(Expr::number(3.)));
    }
}
