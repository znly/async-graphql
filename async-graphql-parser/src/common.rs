use crate::{Positioned, Value};
use std::fmt;

#[derive(Debug, PartialEq)]
pub enum Type {
    Named(String),
    List(Box<Type>),
    NonNull(Box<Type>),
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Type::Named(name) => write!(f, "{}", name),
            Type::List(ty) => write!(f, "[{}]", ty),
            Type::NonNull(ty) => write!(f, "{}!", ty),
        }
    }
}

#[derive(Debug)]
pub struct Directive {
    pub name: Positioned<String>,
    pub arguments: Vec<(Positioned<String>, Positioned<Value>)>,
}

impl Directive {
    pub fn get_argument(&self, name: &str) -> Option<&Positioned<Value>> {
        self.arguments
            .iter()
            .find(|item| item.0.node == name)
            .map(|item| &item.1)
    }
}
