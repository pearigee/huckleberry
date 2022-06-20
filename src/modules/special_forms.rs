use crate::{
    environment::{Environment, EnvironmentRef},
    error::HError,
    expr::{Arity, Expr},
    interpreter::eval_expr,
};

pub fn special_forms_module() -> Environment {
    let mut env = Environment::new();

    env.define(
        "def",
        Expr::native_callable(
            "def",
            Arity::Range(1, 2),
            |args: &[Expr], env: EnvironmentRef| -> Result<Expr, HError> {
                match &args[0] {
                    Expr::Symbol(value) => {
                        env.define(&value, eval_expr(&args[1], env.clone_ref())?);
                        Ok(Expr::Nil)
                    }
                    invalid => Err(HError::UnexpectedForm(invalid.clone())),
                }
            },
        ),
    );

    env.define(
        "set!",
        Expr::native_callable(
            "set!",
            Arity::Count(2),
            |args: &[Expr], env: EnvironmentRef| -> Result<Expr, HError> {
                match &args[0] {
                    Expr::Symbol(value) => {
                        env.set(&value, eval_expr(&args[1], env.clone_ref())?)?;
                        Ok(Expr::Nil)
                    },
                    invalid => Err(HError::UnexpectedForm(invalid.clone())),
                }
            },
        ),
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
        let result = eval("(def a 2) (+ a 1)", env.into_ref());

        assert_eq!(result, Ok(Expr::number(3.)));
    }

    #[test]
    fn test_def_overwrite() {
        let mut env = Environment::new();
        env.merge(math_module());
        env.merge(special_forms_module());
        let env_ref = env.into_ref();
        eval("(def a 2) (def a (+ a 1))", env_ref.clone_ref()).unwrap();

        assert_eq!(env_ref.get("a"), Ok(Expr::number(3.)));
    }

    #[test]
    fn test_set() {
        let mut env = Environment::new();
        env.merge(math_module());
        env.merge(special_forms_module());
        let env_ref = env.into_ref();

        eval("(def a 2) (set! a 1) a", env_ref.clone_ref()).unwrap();

        assert_eq!(env_ref.get("a"), Ok(Expr::number(1.)));
    }

    #[test]
    fn test_set_error_on_unset() {
        let mut env = Environment::new();
        env.merge(math_module());
        env.merge(special_forms_module());
        let env_ref = env.into_ref();

        let result = eval("(set! a 1)", env_ref.clone_ref());

        assert_eq!(result, Err(HError::SetUninitializedVar("a".to_string())));
    }
}
