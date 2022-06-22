use crate::{
    environment::{Environment, EnvironmentRef},
    error::HError,
    expr::{Arity, CodeCallable, Expr},
    interpreter::eval_expr,
};

use super::utils::is_truthy;

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
        "if",
        Expr::native_callable(
            "if",
            Arity::Range(2,3),
            |args: &[Expr], env: EnvironmentRef| -> Result<Expr, HError> {
                let condition = eval_expr(&args[0], env.clone_ref())?;
                if is_truthy(&condition) {
                    Ok(eval_expr(&args[1], env.clone_ref())?)
                } else if args.len() == 3 {
                    Ok(eval_expr(&args[2], env.clone_ref())?)
                } else {
                    Ok(Expr::Nil)
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
                    }
                    invalid => Err(HError::UnexpectedForm(invalid.clone())),
                }
            },
        ),
    );

    env.define(
        "fn",
        Expr::native_callable(
            "fn",
            Arity::Range(1, usize::MAX),
            |args: &[Expr], env: EnvironmentRef| -> Result<Expr, HError> {
                let fn_args = match &args[0] {
                    Expr::Vector(values) => values,
                    value => return Err(HError::UnexpectedForm(value.clone())),
                };
                let mut code: &[Expr] = &[Expr::Nil];
                if args.len() > 1 {
                    code = &args[1..];
                }

                Ok(Expr::CodeCallable(CodeCallable {
                    id: format!("{:?}_{:?}", fn_args, code),
                    arity: Arity::Count(fn_args.len()),
                    args: fn_args.clone(),
                    function: code.into(),
                    closure: env.clone_ref(),
                }))
            },
        ),
    );

    env
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{environment::Environment, interpreter::eval};

    #[test]
    fn test_def() {
        let env = Environment::with_core_module().into_ref();

        let result = eval("(def a 2) (+ a 1)", env);

        assert_eq!(result, Ok(Expr::number(3.)));
    }

    #[test]
    fn test_def_overwrite() {
        let env = Environment::with_core_module().into_ref();

        eval("(def a 2) (def a (+ a 1))", env.clone_ref()).unwrap();

        assert_eq!(env.get("a"), Ok(Expr::number(3.)));
    }

    #[test]
    fn test_if() {
        let env = Environment::with_core_module().into_ref();

        assert_eq!(eval("(if (< 1 2) 1 2)", env.clone_ref()), Ok(Expr::number(1.)));
        assert_eq!(eval("(if (> 1 2) 1 2)", env.clone_ref()), Ok(Expr::number(2.)));
        assert_eq!(eval("(if false 1 2)", env.clone_ref()), Ok(Expr::number(2.)));
        assert_eq!(eval("(if nil 1 2)", env.clone_ref()), Ok(Expr::number(2.)));
        assert_eq!(eval("(if \"hello\" 1 2)", env.clone_ref()), Ok(Expr::number(1.)));
    }

    #[test]
    fn test_set() {
        let env = Environment::with_core_module().into_ref();

        eval("(def a 2) (set! a 1) a", env.clone_ref()).unwrap();

        assert_eq!(env.get("a"), Ok(Expr::number(1.)));
    }

    #[test]
    fn test_set_error_on_unset() {
        let env = Environment::with_core_module().into_ref();

        let result = eval("(set! a 1)", env.clone_ref());

        assert_eq!(result, Err(HError::SetUninitializedVar("a".to_string())));
    }

    #[test]
    fn test_creates_lambdas() {
        let env = Environment::with_core_module().into_ref();

        eval("(def f (fn [a b] (+ a b)))", env.clone_ref()).unwrap();

        assert_eq!(eval("(f 1 2)", env.clone_ref()), Ok(Expr::number(3.)));
    }

    #[test]
    fn test_empty_lambda_returns_nil() {
        let env = Environment::with_core_module().into_ref();

        eval("(def f (fn []))", env.clone_ref()).unwrap();

        assert_eq!(eval("(f)", env.clone_ref()), Ok(Expr::Nil));
    }
}
