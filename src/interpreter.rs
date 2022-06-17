use crate::{
    environment::Environment,
    error::error,
    expr::{CodeCallable, Expr, NativeCallable},
    parser::parse,
};

trait Callable {
    fn call(&self, args: &[Expr], env: &Environment<Expr>) -> Expr;
    fn arity(&self) -> usize;
}

pub fn eval(input: &str, env: &Environment<Expr>) -> Expr {
    let exprs = parse(input);
    eval_exprs(&exprs, env)
}

pub fn eval_exprs(exprs: &Vec<Expr>, env: &Environment<Expr>) -> Expr {
    let mut result = Expr::Nil;
    for expr in exprs {
        result = eval_expr(expr, env);
    }
    result
}

pub fn eval_expr(expr: &Expr, env: &Environment<Expr>) -> Expr {
    match expr {
        Expr::List(list) => {
            if list.is_empty() {
                error("Invalid empty list")
            }
            let (f, args) = list.split_first().unwrap();
            let function = resolve(f, env);
            match function {
                Some(Expr::NativeCallable(callable)) => {
                    if callable.arity() != args.len() {
                        invalid_arity_error(f);
                    }
                    callable.call(args, env)
                }
                Some(Expr::CodeCallable(callable)) => {
                    if callable.arity() != args.len() {
                        invalid_arity_error(f);
                    }
                    let mut arg_env = Environment::extend(env);
                    for i in 0..callable.arity() {
                        arg_env.define(&callable.args[i].id(), eval_expr(&args[i], env))
                    }
                    callable.call(args, &arg_env)
                }
                _ => error(format!("{:?} is not callable", f)),
            }
        }
        _ => expr.to_owned(),
    }
}

pub fn resolve<'a>(expr: &'a Expr, env: &'a Environment<Expr>) -> Option<&'a Expr> {
    match expr {
        Expr::Symbol(id) => env.get(id),
        _ => None,
    }
}

fn invalid_arity_error(callable: &Expr) {
    error(format!("Invalid arity for {:?}", callable));
}

impl Callable for NativeCallable {
    fn arity(&self) -> usize {
        self.arity
    }

    fn call(&self, args: &[Expr], env: &Environment<Expr>) -> Expr {
        (self.function)(args, env)
    }
}

impl Callable for CodeCallable {
    fn arity(&self) -> usize {
        self.args.len()
    }

    fn call(&self, args: &[Expr], env: &Environment<Expr>) -> Expr {
        Expr::Nil
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{modules::math::math_module};

    #[test]
    fn test_calls_native_callable() {
        let mut env = Environment::new();
        env.merge(&math_module());

        assert_eq!(eval("(+ 1 (/ (* 3 (- 5 2)) 3))", &env), Expr::number(4.));
    }
}
