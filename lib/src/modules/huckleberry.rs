use crate::{env::EnvRef, evaluator::eval};

const CODE: &str = "
(defm number? [to: max] (range this max))
(defm number? [to: max do: f] (for-each i (range this max) (f i)))
(defm number? [less-than: n] (lt this n))
(defm number? [less-than-eq: n] (lte this n))
(defm number? [greater-than: n] (gt this n))
(defm number? [greater-than-eq: n] (gte this n))
(defm true [=: e] (= this e))
(defm true [!=: e] (!= this e))
(defm number? [+: n] (+ this n))
(defm number? [-: n] (- this n))
(defm number? [/: n] (/ this n))
(defm number? [*: n] (* this n))
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
    fn test_number_boolean_methods() {
        let env = Env::with_core_module().into_ref();

        assert_eq!(
            eval("<1 less-than: 5>", env.clone_ref()).unwrap(),
            Expr::boolean(true)
        );
        assert_eq!(
            eval("<5 less-than: 5>", env.clone_ref()).unwrap(),
            Expr::boolean(false)
        );
        assert_eq!(
            eval("<5 less-than-eq: 5>", env.clone_ref()).unwrap(),
            Expr::boolean(true)
        );

        assert_eq!(
            eval("<5 greater-than: 1>", env.clone_ref()).unwrap(),
            Expr::boolean(true)
        );
        assert_eq!(
            eval("<5 greater-than: 5>", env.clone_ref()).unwrap(),
            Expr::boolean(false)
        );
        assert_eq!(
            eval("<5 greater-than-eq: 5>", env.clone_ref()).unwrap(),
            Expr::boolean(true)
        );
    }

    #[test]
    fn test_eq_methods() {
        let env = Env::with_core_module().into_ref();

        assert_eq!(
            eval("<[1 2 3] = [1 2 3]>", env.clone_ref()).unwrap(),
            Expr::boolean(true)
        );

        assert_eq!(
            eval("<[1 2 3] = [1 3 2]>", env.clone_ref()).unwrap(),
            Expr::boolean(false)
        );

        assert_eq!(
            eval("<[1 2 3] != [1 2 3]>", env.clone_ref()).unwrap(),
            Expr::boolean(false)
        );

        assert_eq!(
            eval("<[1 2 3] != [1 3 2]>", env.clone_ref()).unwrap(),
            Expr::boolean(true)
        );
    }

    #[test]
    fn test_math_methods() {
        let env = Env::with_core_module().into_ref();

        assert_eq!(eval("<1 + 2>", env.clone_ref()).unwrap(), Expr::number(3.));
        assert_eq!(eval("<1 - 2>", env.clone_ref()).unwrap(), Expr::number(-1.));
        assert_eq!(eval("<1 / 2>", env.clone_ref()).unwrap(), Expr::number(0.5));
        assert_eq!(eval("<1 * 2>", env.clone_ref()).unwrap(), Expr::number(2.));
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
