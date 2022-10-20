use crate::{
    env::{Env, EnvRef},
    error::HError,
    evaluator::resolve_args,
    expr::{Arity, Expr, HMap},
    modules::utils::check_num,
};

macro_rules! num_operator {
    ($name:expr, $op:tt) => {
        Expr::native_fn($name, Arity::Range(1, usize::MAX), |args: &[Expr], env: EnvRef| -> Result<Expr, HError> {
            let resolved = resolve_args(args, env)?;
            let mut result = check_num(&resolved[0], $name)?;
            for expr in &resolved[1..] {
                result = result $op check_num(expr, $name)?;
            }
            Ok(Expr::number(result))
        })
    };
}

macro_rules! num_bool_operator {
    ($name:expr, $op:tt) => {
        Expr::native_fn($name, Arity::Range(2, usize::MAX), |args: &[Expr], env: EnvRef| -> Result<Expr, HError> {
            let resolved = resolve_args(args, env)?;
            let first = check_num(&resolved[0], $name)?;
            for expr in &resolved[1..] {
                if !(first $op check_num(expr, $name)?) {
                    return Ok(Expr::boolean(false));
                };
            }
            Ok(Expr::boolean(true))
        })
    };
}

macro_rules! generic_operator {
    ($name:expr, $op:tt) => {
        Expr::native_fn($name, Arity::Range(2, usize::MAX), |args: &[Expr], env: EnvRef| -> Result<Expr, HError> {
            let resolved = resolve_args(args, env)?;
            let first = &resolved[0];
            for expr in &resolved[1..] {
                if !(first $op expr) {
                    return Ok(Expr::boolean(false));
                };
            }
            Ok(Expr::boolean(true))
        })
    };
}

pub fn native_module() -> Env {
    let mut env = Env::new();

    env.def("+", num_operator!("+", +));
    env.def("-", num_operator!("-", -));
    env.def("*", num_operator!("*", *));
    env.def("/", num_operator!("/", /));

    env.def("lt", num_bool_operator!("lt", <));
    env.def("lte", num_bool_operator!("lte", <=));
    env.def("gt", num_bool_operator!("gt", >));
    env.def("gte", num_bool_operator!("gte", >=));

    env.def("=", generic_operator!("=", ==));
    env.def("!=", generic_operator!("!=", !=));

    env.defn(
        "get",
        Arity::Count(2),
        |args: &[Expr], env: EnvRef| -> Result<Expr, HError> {
            let resolved = resolve_args(args, env)?;
            match &resolved[0] {
                Expr::Map(map, _) => match map.get(&resolved[1]) {
                    Some(value) => Ok(value.clone()),
                    None => Ok(Expr::Nil),
                },
                Expr::Vector(vec, _) => {
                    let index = match &resolved[1] {
                        Expr::Number(value) => (**value) as usize,
                        invalid => {
                            return Err(HError::UnexpectedForm(
                                "Invalid vector index".to_string(),
                                invalid.clone(),
                            ))
                        }
                    };
                    match vec.get(index) {
                        Some(value) => Ok(value.clone()),
                        None => Ok(Expr::Nil),
                    }
                }
                _ => Err(HError::UnexpectedForm(
                    "Type does not support `get`".to_string(),
                    resolved[0].clone(),
                )),
            }
        },
    );

    env.defn(
        "range",
        Arity::Count(2),
        |args: &[Expr], env: EnvRef| -> Result<Expr, HError> {
            let resolved = resolve_args(args, env)?;
            match (&resolved[0], &resolved[1]) {
                (Expr::Number(min), Expr::Number(max)) => {
                    let min = **min as i64;
                    let max = **max as i64;
                    Ok(Expr::Vector(
                        (min..max).map(|n| Expr::number(n as f64)).collect(),
                        HMap::new(),
                    ))
                }
                (min, max) => Err(HError::UnexpectedForm(
                    "Invalid range".to_string(),
                    Expr::vector(&[min.clone(), max.clone()]),
                )),
            }
        },
    );

    env.defn(
        "number?",
        Arity::Count(1),
        |args: &[Expr], env: EnvRef| -> Result<Expr, HError> {
            let resolved = resolve_args(args, env)?;
            match resolved[0] {
                Expr::Number(_) => Ok(Expr::boolean(true)),
                _ => Ok(Expr::boolean(false)),
            }
        },
    );

    env.defn(
        "meta",
        Arity::Count(1),
        |args: &[Expr], env: EnvRef| -> Result<Expr, HError> {
            let resolved = resolve_args(args, env)?;
            match &resolved[0] {
                Expr::Symbol(_, metadata) => Ok(Expr::Map(metadata.clone(), HMap::new())),
                Expr::Map(_, metadata) => Ok(Expr::Map(metadata.clone(), HMap::new())),
                Expr::Vector(_, metadata) => Ok(Expr::Map(metadata.clone(), HMap::new())),
                _ => Ok(Expr::nil()),
            }
        },
    );

    env.defn(
        "print",
        Arity::Range(0, usize::MAX),
        |args: &[Expr], env: EnvRef| -> Result<Expr, HError> {
            print_resolved_exprs(args, env)?;
            Ok(Expr::Nil)
        },
    );

    env.defn(
        "println",
        Arity::Range(0, usize::MAX),
        |args: &[Expr], env: EnvRef| -> Result<Expr, HError> {
            print_resolved_exprs(args, env)?;
            println!();
            Ok(Expr::Nil)
        },
    );

    env
}

fn print_resolved_exprs(exprs: &[Expr], env: EnvRef) -> Result<Expr, HError> {
    for expr in resolve_args(exprs, env)? {
        print!("{}", expr);
    }
    Ok(Expr::Nil)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{env::Env, evaluator::eval};

    #[test]
    fn test_add_op() {
        let env = Env::with_core_module().into_ref();

        assert_eq!(
            eval("(+ 1 2 3 4 5)", env.clone_ref()),
            Ok(Expr::number(15.))
        );
        // Test that a single argument is parsed correctly.
        assert_eq!(eval("(+ 1)", env), Ok(Expr::number(1.)));
    }

    #[test]
    fn test_sub_op() {
        let env = Env::with_core_module().into_ref();

        assert_eq!(eval("(- 1 2 3 4 5)", env), Ok(Expr::number(-13.)));
    }

    #[test]
    fn test_mul_op() {
        let env = Env::with_core_module().into_ref();

        assert_eq!(eval("(* 1 2 3 4 5)", env), Ok(Expr::number(120.)));
    }

    #[test]
    fn test_div_op() {
        let env = Env::with_core_module().into_ref();

        assert_eq!(eval("(/ 20 2 2)", env), Ok(Expr::number(5.)));
    }

    #[test]
    fn test_eq() {
        let env_ref = Env::with_core_module().into_ref();

        assert_eq!(
            eval("(= 1 1)", env_ref.clone_ref()),
            Ok(Expr::boolean(true))
        );
        assert_eq!(
            eval("(= \"hello\" \"hello\")", env_ref.clone_ref()),
            Ok(Expr::boolean(true))
        );
        assert_eq!(
            eval("(= false false)", env_ref.clone_ref()),
            Ok(Expr::boolean(true))
        );
        assert_eq!(
            eval("(= :hello :hello)", env_ref.clone_ref()),
            Ok(Expr::boolean(true))
        );
        assert_eq!(
            eval("(= [1 2] [1 2])", env_ref.clone_ref()),
            Ok(Expr::boolean(true))
        );
        assert_eq!(
            eval("(= {\"key\" :value} {\"key\" :value})", env_ref.clone_ref()),
            Ok(Expr::boolean(true))
        );
    }

    #[test]
    fn test_not_eq() {
        let env_ref = Env::with_core_module().into_ref();

        assert_eq!(
            eval("(!= 1 1)", env_ref.clone_ref()),
            Ok(Expr::boolean(false))
        );
        assert_eq!(
            eval("(!= \"hello\" \"hello\")", env_ref.clone_ref()),
            Ok(Expr::boolean(false))
        );
        assert_eq!(
            eval("(!= false false)", env_ref.clone_ref()),
            Ok(Expr::boolean(false))
        );
        assert_eq!(
            eval("(!= :hello :hello)", env_ref.clone_ref()),
            Ok(Expr::boolean(false))
        );
        assert_eq!(
            eval("(!= [1 2] [1 2])", env_ref.clone_ref()),
            Ok(Expr::boolean(false))
        );
        assert_eq!(
            eval(
                "(!= {\"key\" :value} {\"key\" :value})",
                env_ref.clone_ref()
            ),
            Ok(Expr::boolean(false))
        );
    }

    #[test]
    fn test_lt_op() {
        let env_ref = Env::with_core_module().into_ref();

        assert_eq!(
            eval("(lt 1 2 3 4 5)", env_ref.clone_ref()),
            Ok(Expr::boolean(true))
        );
        assert_eq!(
            eval("(lt 5 4)", env_ref.clone_ref()),
            Ok(Expr::boolean(false))
        );
    }

    #[test]
    fn test_lt_eq_op() {
        let env_ref = Env::with_core_module().into_ref();

        assert_eq!(
            eval("(lte 2 2)", env_ref.clone_ref()),
            Ok(Expr::boolean(true))
        );
        assert_eq!(
            eval("(lte 2 3)", env_ref.clone_ref()),
            Ok(Expr::boolean(true))
        );
        assert_eq!(
            eval("(lte 5 4)", env_ref.clone_ref()),
            Ok(Expr::boolean(false))
        );
    }

    #[test]
    fn test_gt_op() {
        let env_ref = Env::with_core_module().into_ref();

        assert_eq!(
            eval("(gt 5 4 3 2 1)", env_ref.clone_ref()),
            Ok(Expr::boolean(true))
        );
        assert_eq!(
            eval("(gt 4 5)", env_ref.clone_ref()),
            Ok(Expr::boolean(false))
        );
    }

    #[test]
    fn test_gt_eq_op() {
        let env_ref = Env::with_core_module().into_ref();

        assert_eq!(
            eval("(gte 2 2)", env_ref.clone_ref()),
            Ok(Expr::boolean(true))
        );
        assert_eq!(
            eval("(gte 3 2)", env_ref.clone_ref()),
            Ok(Expr::boolean(true))
        );
        assert_eq!(
            eval("(gte 4 5)", env_ref.clone_ref()),
            Ok(Expr::boolean(false))
        );
    }

    #[test]
    fn test_get() {
        let env = Env::with_core_module().into_ref();

        assert_eq!(
            eval("(get {:hello 3} :hello)", env.clone_ref()),
            Ok(Expr::number(3.))
        );
        assert_eq!(
            eval("(get {:hello 3} :world)", env.clone_ref()),
            Ok(Expr::nil())
        );

        assert_eq!(
            eval("(get [1 2 3] 2)", env.clone_ref()),
            Ok(Expr::number(3.))
        );
        assert_eq!(eval("(get [1 2 3] 3)", env.clone_ref()), Ok(Expr::nil()));
    }

    #[test]
    fn test_range() {
        let env = Env::with_core_module().into_ref();

        assert_eq!(
            eval("(range 0 3)", env.clone_ref()),
            Ok(Expr::vector(&[
                Expr::number(0.),
                Expr::number(1.),
                Expr::number(2.)
            ]))
        );
    }

    #[test]
    fn test_number_q() {
        let env = Env::with_core_module().into_ref();

        assert_eq!(
            eval("[(number? 3.2) (number? \"a\")]", env.clone_ref()),
            Ok(Expr::vector(&[Expr::boolean(true), Expr::boolean(false)]))
        );
    }
}
