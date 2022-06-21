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
            let function = resolve(f, env.clone_ref())?;
            match function {
                Expr::NativeCallable(callable) => {
                    callable.call(args, env)
                }
                Expr::CodeCallable(callable) => {
                    callable.call(args, env)
                }
                value => Err(HError::NotAFunction(format!("{}", value))),
            }
        }
        Expr::Symbol(value) => match env.get(value) {
            Ok(expr) => Ok(expr.to_owned()),
            _ => Err(HError::UnboundVar(value.to_string())),
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

pub fn resolve_args(args: &[Expr], env: EnvironmentRef) -> Result<Vec<Expr>, HError> {
    let mut result = Vec::new();
    for expr in args.iter() {
        result.push(eval_expr(expr, env.clone_ref())?);
    }
    Ok(result)
}

impl Callable for NativeCallable {
    fn arity(&self) -> &Arity {
        &self.arity
    }

    fn call(&self, args: &[Expr], env: EnvironmentRef) -> Result<Expr, HError> {
        self.arity.check(&self.id, args)?;
        (self.function)(args, env)
    }
}

impl Callable for CodeCallable {
    fn arity(&self) -> &Arity {
        &self.arity
    }

    fn call(&self, args: &[Expr], env: EnvironmentRef) -> Result<Expr, HError> {
        self.arity.check(&self.id, args)?;
        // TODO: Support variadic functions
        let mut arg_env = Environment::extend(env.clone_ref());
        let mut arg_index: usize = 0;
        for expr in resolve_args(args, env.clone_ref())? {
            arg_env.define(&self.args[arg_index].id(), expr);
            arg_index += 1;
        }
        eval_exprs(&self.function, arg_env.into_ref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calls_native_callable() {
        let env = Environment::with_core_module().into_ref();

        assert_eq!(
            eval("(+ 1 (/ (* 3 (- 5 2)) 3))", env).unwrap(),
            Expr::number(4.)
        );
    }

    #[test]
    fn test_calls_fn() {
        let env = Environment::with_core_module().into_ref();

        eval("(def f (fn [a b] (+ a b)))", env.clone_ref()).unwrap();

        assert_eq!(
            eval("(f 1 2)", env.clone_ref()),
            Ok(Expr::number(3.))
        );
    }

    #[test]
    fn test_checks_arity() {
        let env = Environment::with_core_module().into_ref();

        assert_eq!(
            eval("(def a 3 4)", env.clone_ref()),
            Err(HError::InvalidArity("def".to_string(), Arity::Range(1,2)))
        );

        assert_eq!(
            eval("(set! a)", env.clone_ref()),
            Err(HError::InvalidArity("set!".to_string(), Arity::Count(2)))
        );
    }
}
