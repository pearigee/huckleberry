use crate::{environment::Environment, modules::{special_forms::special_forms_module, math::math_module}, interpreter::eval};

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
