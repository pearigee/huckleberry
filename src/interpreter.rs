use crate::{
    environment::{Environment, EnvironmentRef},
    error::{error, HError},
    expr::{Arity, CodeCallable, Expr, NativeCallable},
    parser::parse,
};

trait Callable {
    fn call(&self, args: &[Expr], env: EnvironmentRef) -> Result<Expr, HError>;
    fn arity(&self) -> &Arity;
}

pub fn eval(input: &str, env: EnvironmentRef) -> Result<Expr, HError> {
    let exprs = parse(input);
    eval_exprs(&exprs, env)
}

pub fn eval_exprs(exprs: &Vec<Expr>, env: EnvironmentRef) -> Result<Expr, HError> {
    let mut result = Expr::Nil;
    for expr in exprs {
        result = eval_expr(expr, env.clone_ref())?;
    }
    Ok(result)
}

pub fn eval_expr(expr: &Expr, env: EnvironmentRef) -> Result<Expr, HError> {
    match expr {
        Expr::List(list) => {
            if list.is_empty() {
                error("Invalid empty list")
            }
            let (f, args) = list.split_first().unwrap();
            let function = resolve(f, env.clone_ref());
            match function {
                Ok(Expr::NativeCallable(callable)) => {
                    callable.arity.check(&callable.id, args)?;
                    callable.call(args, env)
                }
                Ok(Expr::CodeCallable(callable)) => {
                    callable.arity.check(&callable.id, args)?;
                    // TODO: Support variadic functions
                    let resolved_args: Vec<Result<Expr, HError>> = args
                        .iter()
                        .map(|arg| eval_expr(arg, env.clone_ref()))
                        .collect();
                    let mut arg_env = Environment::extend(env.clone_ref());
                    for i in 0..callable.args.len() {
                        arg_env.define(&callable.args[i].id(), resolved_args[i].clone()?);
                    }
                    callable.call(args, arg_env.as_ref())
                }
                _ => error(format!("{:?} is not callable", f)),
            }
        }
        Expr::Symbol(value) => match env.get(value) {
            Ok(expr) => Ok(expr.to_owned()),
            _ => error(format!("{:?} is not bound to anything", value)),
        },
        _ => Ok(expr.to_owned()),
    }
}

pub fn resolve(expr: &Expr, env: EnvironmentRef) -> Result<Expr, HError> {
    match expr {
        Expr::Symbol(id) => env.get(&id),
        _ => Err(HError::UnexpectedForm(expr.to_owned())),
    }
}

impl Callable for NativeCallable {
    fn arity(&self) -> &Arity {
        &self.arity
    }

    fn call(&self, args: &[Expr], env: EnvironmentRef) -> Result<Expr, HError> {
        (self.function)(args, env)
    }
}

impl Callable for CodeCallable {
    fn arity(&self) -> &Arity {
        &self.arity
    }

    fn call(&self, _args: &[Expr], _env: EnvironmentRef) -> Result<Expr, HError> {
        Ok(Expr::Nil)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::{core_module};

    #[test]
    fn test_calls_native_callable() {
        let mut env = Environment::new();
        env.merge(core_module());

        assert_eq!(
            eval("(+ 1 (/ (* 3 (- 5 2)) 3))", env.as_ref()).unwrap(),
            Expr::number(4.)
        );
    }

    #[test]
    fn test_checks_arity() {
        let mut env = Environment::new();
        env.merge(core_module());

        let env_ref = env.as_ref();
        assert_eq!(
            eval("(def a 3 4)", env_ref.clone_ref()),
            Err(HError::InvalidArity("def".to_string(), Arity::Range(1,2)))
        );

        assert_eq!(
            eval("(set! a)", env_ref.clone_ref()),
            Err(HError::InvalidArity("set!".to_string(), Arity::Count(2)))
        );
    }
}
