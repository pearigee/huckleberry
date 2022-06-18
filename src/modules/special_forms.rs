use crate::{
    environment::Environment,
    error::error,
    expr::{Arity, Expr},
};

pub fn special_forms_module() -> Environment<'static> {
    let mut env = Environment::new();

    env.define(
        "def",
        Expr::native_callable("def", Arity::Count(2), |args, env| -> Expr {
            if args.len() != 2 {
                error("`def` takes two arguments: (def var value)")
            }
            match &args[0] {
                Expr::Symbol(value) => env.eval_define(&value, args[1].to_owned()),
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
        env.merge(&math_module());
        env.merge(&special_forms_module());
        let result = eval("(def a 2) (+ a 1)", &mut env);

        assert_eq!(result, Expr::number(3.));
    }

    #[test]
    fn test_def_overwrite() {
        let mut env = Environment::new();
        env.merge(&math_module());
        env.merge(&special_forms_module());
        eval("(def a 2) (def a (+ a 1))", &mut env);

        assert_eq!(env.get("a"), Some(Expr::number(3.)));
    }
}
