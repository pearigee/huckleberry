use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

use crate::{error::HError, expr::Expr, modules::core_module};

pub struct EnvRef(Rc<RefCell<Option<Env>>>);

pub struct Env {
    vars: BTreeMap<String, Expr>,
    enclosing: EnvRef,
}

fn new_rc_ref_cell<T>(x: T) -> Rc<RefCell<T>> {
    Rc::new(RefCell::new(x))
}

impl EnvRef {
    pub fn nil() -> EnvRef {
        EnvRef(new_rc_ref_cell(None))
    }

    pub fn new(env: Env) -> EnvRef {
        EnvRef(new_rc_ref_cell(Some(env)))
    }

    pub fn is_some(&self) -> bool {
        self.0.borrow().as_ref().is_some()
    }

    pub fn clone_ref(&self) -> EnvRef {
        EnvRef(Rc::clone(&self.0))
    }

    pub fn get(&self, id: &str) -> Result<Expr, HError> {
        self.0
            .borrow()
            .as_ref()
            .ok_or_else(|| HError::EnvironmentNotFound)?
            .get(id)
    }

    pub fn set(&self, key: &str, value: Expr) -> Result<Expr, HError> {
        self.0
            .borrow_mut()
            .as_mut()
            .ok_or_else(|| HError::EnvironmentNotFound)?
            .set(key, value)
    }

    pub fn define(&self, key: &str, value: Expr) {
        self.0
            .borrow_mut()
            .as_mut()
            .expect("Environment not found")
            .define(key, value);
    }
}

impl Env {
    pub fn new() -> Env {
        Env {
            vars: BTreeMap::new(),
            enclosing: EnvRef::nil(),
        }
    }

    pub fn with_core_module() -> Env {
        let mut env = Env::new();
        env.merge(core_module());
        env
    }

    pub fn extend(env_ref: EnvRef) -> Env {
        Env {
            vars: BTreeMap::new(),
            enclosing: env_ref,
        }
    }

    pub fn define(&mut self, key: &str, value: Expr) {
        self.vars.insert(key.to_string(), value);
    }

    pub fn merge(&mut self, env: Env) {
        self.vars.extend(env.vars.clone())
    }

    pub fn get(&self, key: &str) -> Result<Expr, HError> {
        let result = self.vars.get(key);
        if result.is_none() && self.enclosing.is_some() {
            return self.enclosing.get(key);
        }
        match result {
            Some(value) => Ok(value.to_owned()),
            _ => Err(HError::UnboundVar(key.to_string())),
        }
    }

    pub fn set(&mut self, key: &str, value: Expr) -> Result<Expr, HError> {
        if self.vars.contains_key(key) {
            self.vars
                .insert(key.to_string(), value)
                .ok_or_else(|| HError::UnboundVar(key.to_string()))
        } else if self.enclosing.is_some() {
            self.enclosing.set(key, value)
        } else {
            Err(HError::SetUninitializedVar(key.to_string()))
        }
    }

    pub fn into_ref(self) -> EnvRef {
        EnvRef::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_define_variables() {
        let mut env = Env::new();
        env.define("key", Expr::string("value"));

        assert_eq!(env.get("key").unwrap(), Expr::string("value"));
    }

    #[test]
    fn test_can_overwrite_vars() {
        let mut env = Env::new();
        env.define("key", Expr::string("value"));
        env.define("key", Expr::number(1.));

        assert_eq!(env.get("key").unwrap(), Expr::number(1.));
    }

    #[test]
    fn test_can_set_vars() {
        let mut env = Env::new();
        env.define("key", Expr::string("value"));

        let env_ref = env.into_ref();

        let mut nested_env = Env::extend(env_ref.clone_ref());
        nested_env.set("key", Expr::number(1.)).unwrap();

        assert_eq!(env_ref.get("key").unwrap(), Expr::number(1.));
    }

    #[test]
    fn test_can_extend_an_environment() {
        let mut env = Env::new();
        env.define("a", Expr::string("a"));
        env.define("b", Expr::string("b"));

        let env_ref = env.into_ref();

        {
            let mut extended_env = Env::extend(env_ref.clone_ref());
            extended_env.define("a", Expr::string("a_shadow"));

            assert_eq!(extended_env.get("a").unwrap(), Expr::string("a_shadow"));
            assert_eq!(extended_env.get("b").unwrap(), Expr::string("b"));
        }

        assert_eq!(env_ref.get("a").unwrap(), Expr::string("a"));
    }
}