use crate::{
    env::{Env, EnvRef},
    error::HError,
    expr::{Arity, Expr, Fn},
    evaluator::eval_expr,
};

use super::utils::is_truthy;

pub fn special_forms_module() -> Env {
    let mut env = Env::new();

    env.defn(
        "def",
        Arity::Range(1, 2),
        |args: &[Expr], env: EnvRef| -> Result<Expr, HError> {
            match &args[0] {
                Expr::Symbol(value) => {
                    env.def(&value, eval_expr(&args[1], env.clone_ref())?);
                    Ok(Expr::Nil)
                }
                invalid => Err(HError::UnexpectedForm(
                    "Only symbols can be defined".to_string(),
                    invalid.clone(),
                )),
            }
        },
    );

    env.defn(
        "if",
        Arity::Range(2, 3),
        |args: &[Expr], env: EnvRef| -> Result<Expr, HError> {
            let condition = eval_expr(&args[0], env.clone_ref())?;
            if is_truthy(&condition) {
                Ok(eval_expr(&args[1], env.clone_ref())?)
            } else if args.len() == 3 {
                Ok(eval_expr(&args[2], env.clone_ref())?)
            } else {
                Ok(Expr::Nil)
            }
        },
    );

    env.defn(
        "set!",
        Arity::Count(2),
        |args: &[Expr], env: EnvRef| -> Result<Expr, HError> {
            match &args[0] {
                Expr::Symbol(value) => {
                    env.set(&value, eval_expr(&args[1], env.clone_ref())?)?;
                    Ok(Expr::Nil)
                }
                invalid => Err(HError::UnexpectedForm(
                    "Only symbols can be set".to_string(),
                    invalid.clone(),
                )),
            }
        },
    );

    env.defn(
        "fn",
        Arity::Range(1, usize::MAX),
        |args: &[Expr], env: EnvRef| -> Result<Expr, HError> {
            let fn_args = match &args[0] {
                Expr::Vector(values) => values,
                value => {
                    return Err(HError::UnexpectedForm(
                        "Expected an argument vector".to_string(),
                        value.clone(),
                    ))
                }
            };

            let mut arity = Arity::Count(fn_args.len());
            for (i, arg) in fn_args.iter().enumerate() {
                match arg {
                    Expr::Ampersand => {
                        arity = Arity::Range(i, usize::MAX);
                        if fn_args.len() != i + 2 {
                            return Err(HError::UnexpectedForm(
                                "In fn declaration, & can only proceed one symbol".to_string(),
                                arg.clone(),
                            ));
                        }
                        break;
                    }
                    _ => (),
                }
            }

            let mut code: &[Expr] = &[Expr::Nil];
            if args.len() > 1 {
                code = &args[1..];
            }

            Ok(Expr::Fn(Fn {
                id: format!("{:?}_{:?}", fn_args, code),
                arity,
                args: fn_args.clone(),
                function: code.into(),
                closure: env.clone_ref(),
            }))
        },
    );

    env
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{env::Env, evaluator::eval};

    #[test]
    fn test_def() {
        let env = Env::with_core_module().into_ref();

        let result = eval("(def a 2) (+ a 1)", env);

        assert_eq!(result, Ok(Expr::number(3.)));
    }

    #[test]
    fn test_def_overwrite() {
        let env = Env::with_core_module().into_ref();

        eval("(def a 2) (def a (+ a 1))", env.clone_ref()).unwrap();

        assert_eq!(env.get("a"), Ok(Expr::number(3.)));
    }

    #[test]
    fn test_if() {
        let env = Env::with_core_module().into_ref();

        assert_eq!(
            eval("(if (lt 1 2) 1 2)", env.clone_ref()),
            Ok(Expr::number(1.))
        );
        assert_eq!(
            eval("(if (gt 1 2) 1 2)", env.clone_ref()),
            Ok(Expr::number(2.))
        );
        assert_eq!(
            eval("(if false 1 2)", env.clone_ref()),
            Ok(Expr::number(2.))
        );
        assert_eq!(eval("(if nil 1 2)", env.clone_ref()), Ok(Expr::number(2.)));
        assert_eq!(
            eval("(if \"hello\" 1 2)", env.clone_ref()),
            Ok(Expr::number(1.))
        );
    }

    #[test]
    fn test_set() {
        let env = Env::with_core_module().into_ref();

        eval("(def a 2) (set! a 1) a", env.clone_ref()).unwrap();

        assert_eq!(env.get("a"), Ok(Expr::number(1.)));
    }

    #[test]
    fn test_set_error_on_unset() {
        let env = Env::with_core_module().into_ref();

        let result = eval("(set! a 1)", env.clone_ref());

        assert_eq!(result, Err(HError::SetUninitializedVar("a".to_string())));
    }

    #[test]
    fn test_creates_lambdas() {
        let env = Env::with_core_module().into_ref();

        eval("(def f (fn [a b] (+ a b)))", env.clone_ref()).unwrap();

        assert_eq!(eval("(f 1 2)", env.clone_ref()), Ok(Expr::number(3.)));
    }

    #[test]
    fn test_creates_variadic_lambdas() {
        let env = Env::with_core_module().into_ref();

        eval("(def f (fn [a &b] (println a b)))", env.clone_ref()).unwrap();

        match env.get("f").unwrap() {
            Expr::Fn(f) => {
                assert_eq!(f.arity, Arity::Range(1, usize::MAX));
            }
            _ => panic!("Expected a function"),
        }

        eval("(def f (fn [a b &c] (println a b)))", env.clone_ref()).unwrap();

        match env.get("f").unwrap() {
            Expr::Fn(f) => {
                assert_eq!(f.arity, Arity::Range(2, usize::MAX));
            }
            _ => panic!("Expected a function"),
        }

        eval("(def f (fn [&c] (println a b)))", env.clone_ref()).unwrap();

        match env.get("f").unwrap() {
            Expr::Fn(f) => {
                assert_eq!(f.arity, Arity::Range(0, usize::MAX));
            }
            _ => panic!("Expected a function"),
        }
    }

    #[test]
    fn test_empty_lambda_returns_nil() {
        let env = Env::with_core_module().into_ref();

        eval("(def f (fn []))", env.clone_ref()).unwrap();

        assert_eq!(eval("(f)", env.clone_ref()), Ok(Expr::Nil));
    }
}
