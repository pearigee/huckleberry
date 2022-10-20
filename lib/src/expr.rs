use std::collections::BTreeMap;

use ordered_float::OrderedFloat;

use crate::{env::EnvRef, error::HError};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub enum Arity {
    Count(usize),
    Range(usize, usize),
}

pub struct NativeFn {
    pub id: String,
    pub arity: Arity,
    pub function: fn(args: &[Expr], env: EnvRef) -> Result<Expr, HError>,
}

pub struct Fn {
    pub id: String,
    pub arity: Arity,
    pub args: Vec<Expr>,
    pub function: Vec<Expr>,
    pub closure: EnvRef,
}

pub struct Method {
    pub id: String,
    pub selector: Box<Expr>,
    pub arity: Arity,
    pub args: Vec<Expr>,
    pub function: Vec<Expr>,
    pub closure: EnvRef,
}

pub type HMap = BTreeMap<Expr, Expr>;

// The second HMap defined below for Symbol, Vector, and Map, is for metadata.
// For example: class type, documentation, source location, etc.
#[derive(Debug, Clone, PartialOrd, Ord)]
pub enum Expr {
    List(Vec<Expr>),
    MethodList(Vec<Expr>),
    Number(OrderedFloat<f64>),
    Boolean(bool),
    String(String),
    Keyword(String),
    Symbol(String, HMap),
    Vector(Vec<Expr>, HMap),
    Map(HMap, HMap),
    NativeFn(NativeFn),
    Fn(Fn),
    Method(Method),
    Ampersand,
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
        Expr::Symbol(value.to_string(), HMap::new())
    }

    pub fn keyword(value: &str) -> Expr {
        Expr::Keyword(value.to_string())
    }

    pub fn vector(exprs: &[Expr]) -> Expr {
        Expr::Vector(exprs.to_vec(), HMap::new())
    }

    pub fn list(exprs: &[Expr]) -> Expr {
        Expr::List(exprs.to_vec())
    }

    pub fn method_list(exprs: &[Expr]) -> Expr {
        Expr::MethodList(exprs.to_vec())
    }

    pub fn ampersand() -> Expr {
        Expr::Ampersand
    }

    pub fn nil() -> Expr {
        Expr::Nil
    }

    pub fn map(exprs: &[(Expr, Expr)]) -> Expr {
        let mut map = BTreeMap::new();
        for pair in exprs.iter() {
            map.insert(pair.0.to_owned(), pair.1.to_owned());
        }
        Expr::Map(map, HMap::new())
    }

    pub fn native_fn(
        name: &str,
        arity: Arity,
        function: fn(args: &[Expr], env: EnvRef) -> Result<Expr, HError>,
    ) -> Expr {
        Expr::NativeFn(NativeFn {
            id: name.to_string(),
            arity,
            function,
        })
    }

    pub fn id(&self) -> String {
        match self {
            Expr::Symbol(name, _) => name.to_string(),
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
            Expr::Symbol(value, _) => write!(f, "{}", value),
            Expr::Keyword(value) => write!(f, "{}", value),
            Expr::String(value) => write!(f, "{}", value),
            Expr::Boolean(value) => write!(f, "{}", value),
            Expr::Vector(value, _) => write!(
                f,
                "[{}]",
                value
                    .iter()
                    .map(|e| e.to_string())
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Expr::Map(value, _) => write!(
                f,
                "{{{}}}",
                value
                    .iter()
                    .map(|(k, v)| format!("{} {}", k.to_string(), v.to_string()))
                    .collect::<Vec<String>>()
                    .join(" ")
            ),
            Expr::Nil => write!(f, "nil"),
            val => write!(f, "{:?}", val),
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            // For types with metadata, it should be ignored for equality checks.
            (Expr::Symbol(a, _), Expr::Symbol(b, _)) => a == b,
            (Expr::Map(a, _), Expr::Map(b, _)) => a == b,
            (Expr::Vector(a, _), Expr::Vector(b, _)) => a == b,
            // Types without metadata.
            (Expr::Boolean(a), Expr::Boolean(b)) => a == b,
            (Expr::Number(a), Expr::Number(b)) => a == b,
            (Expr::String(a), Expr::String(b)) => a == b,
            (Expr::Keyword(a), Expr::Keyword(b)) => a == b,
            (Expr::List(a), Expr::List(b)) => a == b,
            (Expr::MethodList(a), Expr::MethodList(b)) => a == b,
            (Expr::NativeFn(a), Expr::NativeFn(b)) => a == b,
            (Expr::Fn(a), Expr::Fn(b)) => a == b,
            (Expr::Method(a), Expr::Method(b)) => a == b,
            (Expr::Ampersand, Expr::Ampersand) => true,
            (Expr::Nil, Expr::Nil) => true,
            _ => false,
        }
    }
}

impl Eq for Expr {}

impl PartialEq for NativeFn {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for NativeFn {}

impl Ord for NativeFn {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for NativeFn {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl std::fmt::Debug for NativeFn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NativeCallable")
            .field("id", &self.id)
            .finish()
    }
}

impl Clone for NativeFn {
    fn clone(&self) -> Self {
        NativeFn {
            id: self.id.to_string(),
            arity: self.arity.to_owned(),
            function: self.function,
        }
    }
}

impl PartialEq for Fn {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Fn {}

impl Ord for Fn {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Fn {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl std::fmt::Debug for Fn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Fn").field("id", &self.id).finish()
    }
}

impl Clone for Fn {
    fn clone(&self) -> Self {
        Fn {
            id: self.id.to_string(),
            args: self.args.clone(),
            arity: self.arity.to_owned(),
            closure: self.closure.clone_ref(),
            function: self.function.clone(),
        }
    }
}

impl PartialEq for Method {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Method {}

impl Ord for Method {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

impl PartialOrd for Method {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.id.cmp(&other.id))
    }
}

impl std::fmt::Debug for Method {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Method").field("id", &self.id).finish()
    }
}

impl Clone for Method {
    fn clone(&self) -> Self {
        Method {
            id: self.id.to_string(),
            selector: self.selector.clone(),
            args: self.args.clone(),
            arity: self.arity.to_owned(),
            closure: self.closure.clone_ref(),
            function: self.function.clone(),
        }
    }
}
