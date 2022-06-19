use crate::{
    environment::Environment,
    interpreter::eval,
    modules::{math::math_module, special_forms::special_forms_module},
};

mod environment;
mod error;
mod expr;
mod interpreter;
mod modules;
mod parser;
mod scanner;

fn main() {
    let mut env = Environment::new();
    env.merge(math_module());
    env.merge(special_forms_module());
    println!("{:?}", eval("(def a 2) (+ a 2)", env.as_ref()));
}
