use crate::{
    env::{Env, EnvRef},
    error::HError,
    evaluator::resolve_args,
    expr::{Arity, Expr},
};

pub fn data_module() -> Env {
    let mut env = Env::new();

    env.defn(
        "get",
        Arity::Count(2),
        |args: &[Expr], env: EnvRef| -> Result<Expr, HError> {
            let resolved = resolve_args(args, env)?;
            match &resolved[0] {
                Expr::Map(map) => match map.get(&resolved[1]) {
                    Some(value) => Ok(value.clone()),
                    None => Ok(Expr::Nil),
                },
                Expr::Vector(vec) => {
                    let index = match &resolved[1] {
                        Expr::Number(value) => ((**value) as usize),
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

    env
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{env::Env, evaluator::eval};

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
