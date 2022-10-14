use crate::{
    env::{Env, EnvRef},
    error::HError,
    expr::{Arity, Expr},
    evaluator::resolve_args,
    modules::utils::check_num,
};

macro_rules! num_operator {
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

pub fn logic_module() -> Env {
    let mut env = Env::new();

    env.def("lt", num_operator!("lt", <));
    env.def("lte", num_operator!("lte", <=));
    env.def("gt", num_operator!("gt", >));
    env.def("gte", num_operator!("gte", >=));

    env.def("=", generic_operator!("=", ==));
    env.def("!=", generic_operator!("!=", !=));

    env
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{env::Env, evaluator::eval};

    #[test]
    fn test_eq() {
        let mut env = Env::new();
        env.merge(logic_module());
        let env_ref = env.into_ref();

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
        let mut env = Env::new();
        env.merge(logic_module());
        let env_ref = env.into_ref();

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
        let mut env = Env::new();
        env.merge(logic_module());
        let env_ref = env.into_ref();

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
        let mut env = Env::new();
        env.merge(logic_module());
        let env_ref = env.into_ref();

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
        let mut env = Env::new();
        env.merge(logic_module());
        let env_ref = env.into_ref();

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
        let mut env = Env::new();
        env.merge(logic_module());
        let env_ref = env.into_ref();

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
}
