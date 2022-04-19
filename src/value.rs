use std::fmt::Display;

use crate::{ast::Symbol, Id};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
// FIXME this shouldn't be pub
pub struct Value(pub ValueInner);

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone)]
pub enum ValueInner {
    Bool(bool),
    Id(Id),
    I64(i64),
    String(Symbol),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            ValueInner::Bool(b) => b.fmt(f),
            ValueInner::Id(id) => id.fmt(f),
            ValueInner::I64(i) => i.fmt(f),
            ValueInner::String(s) => write!(f, "\"{s}\""),
        }
    }
}

impl Value {
    pub fn fake() -> Self {
        Value(ValueInner::Id(Id::fake()))
    }
}

macro_rules! impl_from {
    ($ctor:ident($t:ty)) => {
        impl From<Value> for $t {
            fn from(value: Value) -> Self {
                match value.0 {
                    ValueInner::$ctor(t) => t,
                    _ => panic!("Expected {}, got {value:?}", stringify!($ctor)),
                }
            }
        }

        impl From<$t> for Value {
            fn from(t: $t) -> Self {
                Value(ValueInner::$ctor(t))
            }
        }
    };
}

impl_from!(Id(Id));
impl_from!(I64(i64));
impl_from!(Bool(bool));
impl_from!(String(Symbol));