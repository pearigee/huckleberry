use crate::{
    env::{Env, EnvRef},
    error::HError,
    expr::{Arity, Expr},
    interpreter::resolve_args,
};

pub fn data_module() -> Env {
    let mut env = Env::new();

    env.define(
        "get",
        Expr::native_fn(
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
        ),
    );

    env
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{env::Env, interpreter::eval};

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
}
