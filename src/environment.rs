use std::collections::BTreeMap;

use crate::expr::Expr;

pub struct Environment<'a, T> {
    vars: BTreeMap<String, T>,
    enclosing: Option<&'a Environment<'a, T>>,
}

impl<'a, T> Environment<'a, T> {
    pub fn new() -> Environment<'a, T> {
        Environment {
            vars: BTreeMap::new(),
            enclosing: None,
        }
    }

    pub fn extend(environment: &'a Environment<T>) -> Environment<'a, T> {
        Environment {
            vars: BTreeMap::new(),
            enclosing: Some(environment),
        }
    }

    pub fn define(&mut self, key: &str, value: T) {
        self.vars.insert(key.to_string(), value);
    }

    pub fn get(&self, key: &str) -> Option<&T> {
        let result = self.vars.get(key);
        if result.is_none() && self.enclosing.is_some() {
            return self.enclosing.unwrap().get(key);
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_can_define_variables() {
        let mut env = Environment::new();
        env.define("key", Expr::string("value"));

        assert_eq!(env.get("key").unwrap(), &Expr::string("value"));
    }

    #[test]
    fn test_can_overwrite_vars() {
        let mut env = Environment::new();
        env.define("key", Expr::string("value"));
        env.define("key", Expr::number(1.));

        assert_eq!(env.get("key").unwrap(), &Expr::number(1.));
    }

    #[test]
    fn test_can_extend_an_environment() {
        let mut env = Environment::new();
        env.define("a", Expr::string("a"));
        env.define("b", Expr::string("b"));

        {
            let mut extended_env = Environment::extend(&env);
            extended_env.define("a", Expr::string("a_shadow"));

            assert_eq!(extended_env.get("a").unwrap(), &Expr::string("a_shadow"));
            assert_eq!(extended_env.get("b").unwrap(), &Expr::string("b"));
        }

        assert_eq!(env.get("a").unwrap(), &Expr::string("a"));
    }
}
