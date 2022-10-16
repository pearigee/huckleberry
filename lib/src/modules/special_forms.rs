use crate::{
    env::{Env, EnvRef},
    error::HError,
    evaluator::eval_expr,
    expr::{Arity, Expr, Fn, Method},
};

use super::utils::{is_truthy, method_args, method_id};

pub fn special_forms_module() -> Env {
    let mut env = Env::new();

    env.defn(
        "var",
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
        |args: &[Expr], env: EnvRef| function(args, env),
    );

    env.defn(
        "defn",
        Arity::Range(2, usize::MAX),
        |args: &[Expr], env: EnvRef| -> Result<Expr, HError> {
            let name_expr = &args[0];
            let name = match name_expr {
                Expr::Symbol(value) => value,
                invalid => {
                    return Err(HError::UnexpectedForm(
                        "\"defn\" requires a symbol for name".to_string(),
                        invalid.clone(),
                    ))
                }
            };

            let fun_expr = function(&args[1..], env.clone_ref())?;
            env.def(name, fun_expr);

            Ok(Expr::nil())
        },
    );

    env.defn(
        "defm",
        Arity::Range(2, usize::MAX),
        |args: &[Expr], env: EnvRef| -> Result<Expr, HError> {
            let selector: Expr = eval_expr(&args[0], env.clone_ref())?;

            let raw_args = match &args[1] {
                Expr::Vector(values) => values,
                value => {
                    return Err(HError::UnexpectedForm(
                        "Expected an argument vector".to_string(),
                        value.clone(),
                    ))
                }
            };

            let arg_len = raw_args.len();
            if arg_len == 0 {
                return Err(HError::UnexpectedForm(
                    "Expected at least one argument".to_string(),
                    args[1].clone(),
                ));
            }

            if arg_len > 1 && arg_len % 2 == 1 {
                return Err(HError::UnexpectedForm(
                    "Expected an even number of arguments".to_string(),
                    args[1].clone(),
                ));
            }

            let name = method_id(raw_args);
            let filtered_args: Vec<Expr> = method_args(raw_args);
            let arity = Arity::Count(filtered_args.len());

            let mut code: &[Expr] = &[Expr::Nil];
            if args.len() > 2 {
                code = &args[2..];
            }
            env.defm(
                &name,
                Method {
                    id: name.to_string(),
                    selector: Box::new(selector.clone()),
                    arity,
                    args: filtered_args.clone(),
                    function: code.into(),
                    closure: env.clone_ref(),
                },
            );

            Ok(Expr::nil())
        },
    );

    env
}

fn function(args: &[Expr], env: EnvRef) -> Result<Expr, HError> {
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
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        env::Env,
        evaluator::{eval, get_first_method_matching},
    };

    #[test]
    fn test_defm() {
        let env = Env::with_core_module().into_ref();

        eval(
            "(defm (fn [i] i) [to: n do: f] (println num))",
            env.clone_ref(),
        )
        .unwrap();

        let method = get_first_method_matching("to do", &Expr::boolean(true), env.clone_ref())
            .unwrap()
            .unwrap();

        assert_eq!(method.id, "to do");
        assert_eq!(method.arity, Arity::Count(2));
        assert_eq!(
            method.args,
            vec![Expr::Symbol("n".to_string()), Expr::Symbol("f".to_string())]
        );
        assert_eq!(
            method.selector,
            Box::new(Expr::Fn(Fn {
                id: "[Symbol(\"i\")]_[Symbol(\"i\")]".to_string(),
                arity: Arity::Count(1),
                args: vec![Expr::Symbol("i".to_string())],
                function: vec![Expr::Symbol("i".to_string())].into(),
                closure: env.clone_ref(),
            }))
        );
        assert_eq!(
            method.function,
            vec![Expr::list(&[
                Expr::Symbol("println".to_string()),
                Expr::Symbol("num".to_string())
            ])]
        );
    }

    #[test]
    fn test_defn() {
        let env = Env::with_core_module().into_ref();

        eval("(defn add [a b] (+ a b))", env.clone_ref()).unwrap();

        match env.get("add").unwrap() {
            Expr::Fn(fun) => {
                assert_eq!(fun.arity, Arity::Count(2));
                assert_eq!(
                    fun.args,
                    vec![Expr::Symbol("a".to_string()), Expr::Symbol("b".to_string())]
                );
                assert_eq!(
                    fun.function,
                    vec![Expr::list(&[
                        Expr::Symbol("+".to_string()),
                        Expr::Symbol("a".to_string()),
                        Expr::Symbol("b".to_string())
                    ])]
                );
            }
            _ => panic!("Expected a function"),
        }
    }

    #[test]
    fn test_def() {
        let env = Env::with_core_module().into_ref();

        let result = eval("(var a 2) (+ a 1)", env);

        assert_eq!(result, Ok(Expr::number(3.)));
    }

    #[test]
    fn test_def_overwrite() {
        let env = Env::with_core_module().into_ref();

        eval("(var a 2) (var a (+ a 1))", env.clone_ref()).unwrap();

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

        eval("(var a 2) (set! a 1) a", env.clone_ref()).unwrap();

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

        eval("(var f (fn [a b] (+ a b)))", env.clone_ref()).unwrap();

        assert_eq!(eval("(f 1 2)", env.clone_ref()), Ok(Expr::number(3.)));
    }

    #[test]
    fn test_creates_variadic_lambdas() {
        let env = Env::with_core_module().into_ref();

        eval("(var f (fn [a &b] (println a b)))", env.clone_ref()).unwrap();

        match env.get("f").unwrap() {
            Expr::Fn(f) => {
                assert_eq!(f.arity, Arity::Range(1, usize::MAX));
            }
            _ => panic!("Expected a function"),
        }

        eval("(var f (fn [a b &c] (println a b)))", env.clone_ref()).unwrap();

        match env.get("f").unwrap() {
            Expr::Fn(f) => {
                assert_eq!(f.arity, Arity::Range(2, usize::MAX));
            }
            _ => panic!("Expected a function"),
        }

        eval("(var f (fn [&c] (println a b)))", env.clone_ref()).unwrap();

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

        eval("(var f (fn []))", env.clone_ref()).unwrap();

        assert_eq!(eval("(f)", env.clone_ref()), Ok(Expr::Nil));
    }
}
