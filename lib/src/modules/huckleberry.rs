use crate::{env::EnvRef, evaluator::eval};

const CODE: &str = "
(defm number? [to: max] (range this max))
(defm number? [to: max do: f] (for-each i (range this max) (f i)))
";

pub fn add_eval_definitions(env: EnvRef) {
    eval(CODE, env.clone_ref()).unwrap();
}

#[cfg(test)]
mod tests {
    use crate::{env::Env, evaluator::eval, expr::Expr};

    #[test]
    fn test_number_to() {
        let env = Env::with_core_module().into_ref();

        assert_eq!(
            eval("<1 to: 5>", env.clone_ref()),
            Ok(Expr::vector(&[
                Expr::number(1.),
                Expr::number(2.),
                Expr::number(3.),
                Expr::number(4.)
            ]))
        );
    }

    #[test]
    fn test_number_to_do() {
        let env = Env::with_core_module().into_ref();
        eval(
            "
            (def a 1)
            <1 to: 6 do: (fn [i] (set! a (+ a i)))>",
            env.clone_ref(),
        )
        .unwrap();
        assert_eq!(env.get("a"), Ok(Expr::number(16.)));
    }
}
