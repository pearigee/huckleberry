use std::collections::BTreeMap;

use ordered_float::OrderedFloat;

use crate::{environment::EnvironmentRef, error::HError};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Arity {
    Count(usize),
    Range(usize, usize),
}

pub struct NativeCallable {
    pub id: String,
    pub arity: Arity,
    pub function: fn(args: &[Expr], env: EnvironmentRef) -> Result<Expr, HError>,
}

pub struct CodeCallable {
    pub id: String,
    pub arity: Arity,
    pub args: Vec<Expr>,
    pub function: Vec<Expr>,
    pub closure: EnvironmentRef,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Expr {
    List(Vec<Expr>),
    Number(OrderedFloat<f64>),
    Boolean(bool),
    String(String),
    Keyword(String),
    Symbol(String),
    Vector(Vec<Expr>),
    Map(BTreeMap<Expr, Expr>),
    NativeCallable(NativeCallable),
    CodeCallable(CodeCallable),
    Nil,
}

impl Expr {
    pub fn number(value: f64) -> Expr {
        Expr::Number(OrderedFloat(value))
    }

    pub fn boolean(value: bool) -> Expr {
        Expr::Boolean(value)
    }

    pub fn string(value: &str) -> Expr {
        Expr::String(value.to_string())
    }

    pub fn symbol(value: &str) -> Expr {
        Expr::Symbol(value.to_string())
    }

    pub fn keyword(value: &str) -> Expr {
        Expr::Keyword(value.to_string())
    }

    pub fn vector(exprs: &[Expr]) -> Expr {
        Expr::Vector(exprs.to_vec())
    }

    pub fn list(exprs: &[Expr]) -> Expr {
        Expr::List(exprs.to_vec())
    }

    pub fn nil() -> Expr {
        Expr::Nil
    }

    pub fn map(exprs: &[(Expr, Expr)]) -> Expr {
        let mut map = BTreeMap::new();
        for pair in exprs.iter() {
            map.insert(pair.0.to_owned(), pair.1.to_owned());
        }
        Expr::Map(map)
    }

    pub fn native_callable(
        name: &str,
        arity: Arity,
        function: fn(args: &[Expr], env: EnvironmentRef) -> Result<Expr, HError>,
    ) -> Expr {
        Expr::NativeCallable(NativeCallable {
            id: name.to_string(),
            arity,
            function,
        })
    }

    pub fn id(&self) -> String {
        match self {
            Expr::Symbol(name) => name.to_string(),
            _ => format!("{:?}", self),
        }
    }
}

impl Arity {
    pub fn check(&self, name: &str, args: &[Expr]) -> Result<(), HError> {
        let matches = match self {
            Arity::Count(num) => args.len() == *num,
            Arity::Range(min, max) => *min <= args.len() && args.len() <= *max,
        };

        if matches {
            Ok(())
        } else {
            Err(HError::InvalidArity(name.to_string(), self.clone()))
        }
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Number(OrderedFloat(value)) => write!(f, "{}", value),
            Expr::Symbol(value) => write!(f, "{}", value),
            Expr::Keyword(value) => write!(f, "{}", value),
            Expr::String(value) => write!(f, "{}", value),
            Expr::Nil => write!(f, "nil"),
            val => write!(f, "{:?}", val),
        }
    }
}

impl PartialEq for NativeCallable {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for NativeCallable {}

impl Ord for NativeCallable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for NativeCallable {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl std::fmt::Debug for NativeCallable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeCallable")
            .field("id", &self.id)
            .finish()
    }
}

impl Clone for NativeCallable {
    fn clone(&self) -> Self {
        NativeCallable {
            id: self.id.to_string(),
            arity: self.arity.to_owned(),
            function: self.function,
        }
    }
}

impl PartialEq for CodeCallable {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for CodeCallable {}

impl Ord for CodeCallable {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for CodeCallable {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl std::fmt::Debug for CodeCallable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Fn").field("id", &self.id).finish()
    }
}

impl Clone for CodeCallable {
    fn clone(&self) -> Self {
        CodeCallable {
            id: self.id.to_string(),
            args: self.args.clone(),
            arity: self.arity.to_owned(),
            closure: self.closure.clone_ref(),
            function: self.function.clone(),
        }
    }
}
