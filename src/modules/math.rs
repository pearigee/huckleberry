use crate::{
    environment::Environment,
    error::error,
    expr::{Arity, Expr},
    interpreter::eval_expr,
};

macro_rules! operator {
    ($name:expr, $op:tt) => {
        Expr::native_callable($name, Arity::Variadic, |args, env| -> Expr {
            args.iter()
            .map(|expr| eval_expr(expr, env))
            .reduce(|a, b| {
                if let (Expr::Number(a), Expr::Number(b)) = (a,b) {
                       Expr::number(*a $op *b)
                   } else {
                       error(format!("{} can only be applied to numbers.", $name))
                   }
            }).unwrap()
        })
    };
}

pub fn math_module() -> Environment<'static> {
    let mut env = Environment::new();

    env.define("+", operator!("+", +));
    env.define("-", operator!("-", -));
    env.define("*", operator!("*", *));
    env.define("/", operator!("/", /));

    env
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{environment::Environment, interpreter::eval, modules::math::math_module};

    #[test]
    fn test_add_op() {
        let mut env = Environment::new();
        env.merge(&math_module());

        assert_eq!(eval("(+ 1 2 3 4 5)", &mut env), Expr::number(15.));
    }

    #[test]
    fn test_sub_op() {
        let mut env = Environment::new();
        env.merge(&math_module());

        assert_eq!(eval("(- 1 2 3 4 5)", &mut env), Expr::number(-13.));
    }

    #[test]
    fn test_mul_op() {
        let mut env = Environment::new();
        env.merge(&math_module());

        assert_eq!(eval("(* 1 2 3 4 5)", &mut env), Expr::number(120.));
    }

    #[test]
    fn test_div_op() {
        let mut env = Environment::new();
        env.merge(&math_module());

        assert_eq!(eval("(/ 20 2 2)", &mut env), Expr::number(5.));
    }
}
