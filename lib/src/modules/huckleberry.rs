use crate::{env::EnvRef, evaluator::eval};

const CODE: &str = "
(defm number? [to: max] (range this max))
";

pub fn add_eval_definitions(env: EnvRef) {
    eval(CODE, env.clone_ref()).unwrap();
}
