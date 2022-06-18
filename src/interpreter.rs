use crate::{
    environment::Environment,
    error::error,
    expr::{Arity, CodeCallable, Expr, NativeCallable},
    parser::parse,
};

trait Callable {
    fn call(&self, args: &[Expr], env: &mut Environment) -> Expr;
    fn arity(&self) -> &Arity;
}

pub fn eval(input: &str, env: &mut Environment) -> Expr {
    let exprs = parse(input);
    eval_exprs(&exprs, env)
}

pub fn eval_exprs(exprs: &Vec<Expr>, env: &mut Environment) -> Expr {
    let mut result = Expr::Nil;
    for expr in exprs {
        result = eval_expr(expr, env);
    }
    result
}

pub fn eval_expr(expr: &Expr, env: &mut Environment) -> Expr {
    match expr {
        Expr::List(list) => {
            if list.is_empty() {
                error("Invalid empty list")
            }
            let (f, args) = list.split_first().unwrap();
            let function = resolve(f, env);
            match function {
                Some(Expr::NativeCallable(callable)) => {
                    if let Arity::Count(value) = callable.arity {
                        if value != args.len() {
                            invalid_arity_error(f);
                        }
                    }
                    callable.call(args, env)
                }
                Some(Expr::CodeCallable(callable)) => {
                    if let Arity::Count(value) = callable.arity {
                        if value != args.len() {
                            invalid_arity_error(f);
                        }
                    }
                    // TODO: Support variadic functions
                    let resolved_args: Vec<Expr> =
                        args.iter().map(|arg| eval_expr(arg, env)).collect();
                    let mut arg_env = Environment::extend(&env);
                    for i in 0..callable.args.len() {
                        arg_env.define(&callable.args[i].id(), resolved_args[i].to_owned());
                    }
                    callable.call(args, &mut arg_env)
                }
                _ => error(format!("{:?} is not callable", f)),
            }
        }
        Expr::Symbol(value) => match env.get(value) {
            Some(expr) => expr.to_owned(),
            _ => error(format!("{:?} is not bound to anything", value)),
        },
        _ => expr.to_owned(),
    }
}

pub fn resolve<'a>(expr: &'a Expr, env: &'a Environment) -> Option<Expr> {
    match expr {
        Expr::Symbol(id) => env.get(id),
        _ => None,
    }
}

fn invalid_arity_error(callable: &Expr) {
    error(format!("Invalid arity for {:?}", callable));
}

impl Callable for NativeCallable {
    fn arity(&self) -> &Arity {
        &self.arity
    }

    fn call(&self, args: &[Expr], env: &mut Environment) -> Expr {
        (self.function)(args, env)
    }
}

impl Callable for CodeCallable {
    fn arity(&self) -> &Arity {
        &self.arity
    }

    fn call(&self, _args: &[Expr], _env: &mut Environment) -> Expr {
        Expr::Nil
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::modules::math::math_module;

    #[test]
    fn test_calls_native_callable() {
        let mut env = Environment::new();
        env.merge(&math_module());

        assert_eq!(
            eval("(+ 1 (/ (* 3 (- 5 2)) 3))", &mut env),
            Expr::number(4.)
        );
    }
}
