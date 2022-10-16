use crate::env::{Env, EnvRef};

use self::{
    data::data_module, huckleberry::add_eval_definitions, io::io_module, logic::logic_module,
    math::math_module, special_forms::special_forms_module,
};

pub mod data;
pub mod huckleberry;
pub mod io;
pub mod logic;
pub mod math;
pub mod special_forms;
pub mod utils;

pub fn core_module() -> EnvRef {
    let env = Env::new().into_ref();
    env.merge(math_module()).unwrap();
    env.merge(special_forms_module()).unwrap();
    env.merge(logic_module()).unwrap();
    env.merge(io_module()).unwrap();
    env.merge(data_module()).unwrap();
    add_eval_definitions(env.clone_ref());
    env
}
