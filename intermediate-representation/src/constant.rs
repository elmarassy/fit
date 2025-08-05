use std::hash::{Hash, Hasher};

use crate::expression::Number;

#[derive(Debug, Clone, PartialEq)]
pub enum Constant {
    Float(Number),
    Integer(i32),
}

impl Hash for Constant {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Float(number) => number.to_string().hash(state),
            Self::Integer(number) => number.hash(state),
        }
    }
}

impl Eq for Constant {}

impl Constant {
    pub fn new_float(number: Number) -> Self {
        Self::Float(number)
    }
    pub fn new_int(number: i32) -> Self {
        Self::Integer(number)
    }
    pub fn negate(&self) -> Self {
        match &self {
            Self::Float(num) => Self::Float(-num),
            Self::Integer(num) => Self::Integer(-num),
        }
    }
}
