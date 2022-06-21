use crate::environment::Environment;

use self::{math::math_module, special_forms::special_forms_module, io::io_module};

pub mod math;
pub mod special_forms;
pub mod io;

pub fn core_module() -> Environment {
    let mut env = Environment::new();
    env.merge(math_module());
    env.merge(special_forms_module());
    env.merge(io_module());
    env
}