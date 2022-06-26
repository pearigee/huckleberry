use crate::env::Env;

use self::{
    data::data_module, io::io_module, logic::logic_module, math::math_module,
    special_forms::special_forms_module,
};

pub mod data;
pub mod io;
pub mod logic;
pub mod math;
pub mod special_forms;
pub mod utils;

pub fn core_module() -> Env {
    let mut env = Env::new();
    env.merge(math_module());
    env.merge(special_forms_module());
    env.merge(logic_module());
    env.merge(io_module());
    env.merge(data_module());
    env
}
