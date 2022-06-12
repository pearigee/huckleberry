use std::collections::BTreeMap;

use ordered_float::OrderedFloat;

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
}