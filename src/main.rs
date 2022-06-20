use crate::{
    environment::Environment,
    interpreter::eval,
    modules::{core_module},
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
    env.merge(core_module());
    println!("{:?}", eval("(def a 2) (+ a 2)", env.into_ref()));
}
