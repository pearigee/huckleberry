use std::collections::BTreeMap;

use crate::{
    env::{Env, EnvRef},
    error::HError,
    expr::{Arity, Expr, Fn, NativeFn},
    parser::parse,
};

trait Callable {
    fn call(&self, args: &[Expr], env: EnvRef) -> Result<Expr, HError>;
    fn arity(&self) -> &Arity;
}

pub fn eval(input: &str, env: EnvRef) -> Result<Expr, HError> {
    let exprs = parse(input)?;
    eval_exprs(&exprs, env)
}

pub fn eval_exprs(exprs: &Vec<Expr>, env: EnvRef) -> Result<Expr, HError> {
    let mut result = Expr::Nil;
    for expr in exprs {
        result = eval_expr(expr, env.clone_ref())?;
    }
    Ok(result)
}

pub fn eval_expr(expr: &Expr, env: EnvRef) -> Result<Expr, HError> {
    match expr {
        Expr::List(list) => {
            if list.is_empty() {
                return Err(HError::InvalidEmptyList(
                    "An empty list is invalid. Should this be a function call?".to_string(),
                ));
            }
            let (f, args) = list.split_first().unwrap();
            let function = resolve(f, env.clone_ref())?;
            match function {
                Expr::NativeFn(callable) => callable.call(args, env),
                Expr::Fn(callable) => callable.call(args, env),
                value => Err(HError::NotAFunction(format!("{}", value))),
            }
        }
        Expr::Symbol(value) => match env.get(value) {
            Ok(expr) => Ok(expr.to_owned()),
            _ => Err(HError::UnboundVar(value.to_string())),
        },
        Expr::Map(map) => {
            let resolved_map: BTreeMap<Expr, Expr> = map
                .into_iter()
                .map(|(key, value)| {
                    Ok((
                        eval_expr(&key, env.clone_ref())?,
                        eval_expr(&value, env.clone_ref())?,
                    ))
                })
                .collect::<Result<_, _>>()?;
            Ok(Expr::Map(resolved_map))
        }
        Expr::Vector(vector) => {
            let resolved_vector: Vec<Expr> = vector
                .into_iter()
                .map(|value| eval_expr(&value, env.clone_ref()))
                .collect::<Result<_, _>>()?;
            Ok(Expr::Vector(resolved_vector))
        }
        _ => Ok(expr.to_owned()),
    }
}

pub fn resolve(expr: &Expr, env: EnvRef) -> Result<Expr, HError> {
    match expr {
        Expr::Symbol(id) => env.get(&id),
        _ => Err(HError::UnexpectedForm(
            "Only symbols are resolvable".to_string(),
            expr.to_owned(),
        )),
    }
}

pub fn resolve_args(args: &[Expr], env: EnvRef) -> Result<Vec<Expr>, HError> {
    let mut result = Vec::new();
    for expr in args.iter() {
        result.push(eval_expr(expr, env.clone_ref())?);
    }
    Ok(result)
}

impl Callable for NativeFn {
    fn arity(&self) -> &Arity {
        &self.arity
    }

    fn call(&self, args: &[Expr], env: EnvRef) -> Result<Expr, HError> {
        self.arity.check(&self.id, args)?;
        (self.function)(args, env)
    }
}

impl Callable for Fn {
    fn arity(&self) -> &Arity {
        &self.arity
    }

    fn call(&self, args: &[Expr], env: EnvRef) -> Result<Expr, HError> {
        self.arity.check(&self.id, args)?;
        let mut arg_env = Env::extend(env.clone_ref());
        for (i, binding) in self.args.iter().enumerate() {
            match binding {
                Expr::Symbol(ref name) => {
                    arg_env.def(name, eval_expr(&args[i], env.clone_ref())?.clone())
                }
                Expr::Ampersand => {
                    arg_env.def(&self.args[i + 1].id(), Expr::Vector(args[i..].to_vec()));
                    break;
                }
                _ => {
                    return Err(HError::UnexpectedForm(
                        "Expected a symbol argument".to_string(),
                        self.args[i].clone(),
                    ))
                }
            }
        }
        eval_exprs(&self.function, arg_env.into_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calls_native_callable() {
        let env = Env::with_core_module().into_ref();

        assert_eq!(
            eval("(+ 1 (/ (* 3 (- 5 2)) 3))", env).unwrap(),
            Expr::number(4.)
        );
    }

    #[test]
    fn test_calls_fn() {
        let env = Env::with_core_module().into_ref();

        eval("(def f (fn [a b] (+ a b)))", env.clone_ref()).unwrap();

        assert_eq!(eval("(f 1 2)", env.clone_ref()), Ok(Expr::number(3.)));
    }

    #[test]
    fn test_calls_variadic_fn() {
        let env = Env::with_core_module().into_ref();

        eval("(def f (fn [a &b] [a b]))", env.clone_ref()).unwrap();

        assert_eq!(
            eval("(f 1 2 3 4)", env.clone_ref()),
            Ok(Expr::vector(&[
                Expr::number(1.),
                Expr::vector(&[Expr::number(2.), Expr::number(3.), Expr::number(4.),])
            ]))
        );

        eval("(def f (fn [&a] a))", env.clone_ref()).unwrap();

        assert_eq!(
            eval("(f 1 2 3 4)", env.clone_ref()),
            Ok(Expr::vector(&[
                Expr::number(1.),
                Expr::number(2.),
                Expr::number(3.),
                Expr::number(4.),
            ]))
        );
    }

    #[test]
    fn test_checks_arity() {
        let env = Env::with_core_module().into_ref();

        assert_eq!(
            eval("(def a 3 4)", env.clone_ref()),
            Err(HError::InvalidArity("def".to_string(), Arity::Range(1, 2)))
        );

        assert_eq!(
            eval("(set! a)", env.clone_ref()),
            Err(HError::InvalidArity("set!".to_string(), Arity::Count(2)))
        );
    }

    #[test]
    fn test_resolves_map() {
        let env = Env::with_core_module().into_ref();

        assert_eq!(
            eval("{:a (+ 1 2)}", env.clone_ref()),
            Ok(Expr::map(&[(Expr::keyword(":a"), Expr::number(3.)),]))
        );
    }

    #[test]
    fn test_resolves_vector() {
        let env = Env::with_core_module().into_ref();

        assert_eq!(
            eval("[1 (+ 1 2)]", env.clone_ref()),
            Ok(Expr::vector(&[Expr::number(1.), Expr::number(3.),]))
        );
    }
}
