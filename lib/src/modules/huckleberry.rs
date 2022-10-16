use crate::{env::EnvRef, evaluator::eval};

pub fn add_eval_definitions(env: EnvRef) {
    eval("(defm number? [to: max] (range this max))", env.clone_ref()).unwrap();
}
