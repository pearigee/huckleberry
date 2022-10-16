use crate::env::{Env, EnvRef};

use self::{
    huckleberry::add_eval_definitions, native::native_module, special_forms::special_forms_module,
};

pub mod huckleberry;
pub mod native;
pub mod special_forms;
pub mod utils;

pub fn core_module() -> EnvRef {
    let env = Env::new().into_ref();
    env.merge(special_forms_module()).unwrap();
    env.merge(native_module()).unwrap();
    add_eval_definitions(env.clone_ref());
    env
}
