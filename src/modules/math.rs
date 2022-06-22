use crate::{
    environment::{Environment, EnvironmentRef},
    error::HError,
    expr::{Arity, Expr},
    modules::utils::{resolve_args, check_num},
};

macro_rules! operator {
    ($name:expr, $op:tt) => {
        Expr::native_callable($name, Arity::Range(1, usize::MAX), |args: &[Expr], env: EnvironmentRef| -> Result<Expr, HError> {
            let resolved = resolve_args(args, env)?;
            let mut result = check_num(&resolved[0], $name)?;
            for expr in &resolved[1..] {
                result = result $op check_num(expr, $name)?;
            }
            Ok(Expr::number(result))
        })
    };
}

pub fn math_module() -> Environment {
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
        env.merge(math_module());
        let env_ref = env.into_ref();

        assert_eq!(
            eval("(+ 1 2 3 4 5)", env_ref.clone_ref()),
            Ok(Expr::number(15.))
        );
        // Test that a single argument is parsed correctly.
        assert_eq!(eval("(+ 1)", env_ref), Ok(Expr::number(1.)));
    }

    #[test]
    fn test_sub_op() {
        let mut env = Environment::new();
        env.merge(math_module());

        assert_eq!(
            eval("(- 1 2 3 4 5)", env.into_ref()),
            Ok(Expr::number(-13.))
        );
    }

    #[test]
    fn test_mul_op() {
        let mut env = Environment::new();
        env.merge(math_module());

        assert_eq!(
            eval("(* 1 2 3 4 5)", env.into_ref()),
            Ok(Expr::number(120.))
        );
    }

    #[test]
    fn test_div_op() {
        let mut env = Environment::new();
        env.merge(math_module());

        assert_eq!(eval("(/ 20 2 2)", env.into_ref()), Ok(Expr::number(5.)));
    }
}
