use std::fmt;
use std::fmt::Formatter;
use crate::prelude::*;
use crate::vm::Binding::Unbound;

use std::sync::Arc;

#[derive(Debug)]
enum ValueData {
    Number(f64),
    String(String)
}

struct Value {
    data: Arc<ValueData>
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.data.as_ref() {
            ValueData::Number(value) => write!(f, "{}", value),
            ValueData::String(value) => write!(f, "{}", value),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self.data.as_ref() {
            ValueData::Number(value) => write!(f, "{}", value),
            ValueData::String(value) => write!(f, "{}", value),
        }
    }
}


impl From<usize> for Value {
    fn from(value: usize) -> Self {
        Value { data: Arc::new(ValueData::Number(value as f64)) }
    }
}

impl From<f64> for Value {
    fn from(value: f64) -> Self {
        Value { data: Arc::new(ValueData::Number(value)) }
    }
}

impl From<String> for Value {
    fn from(value: String) -> Self {
        Value { data: Arc::new(ValueData::String(value)) }
    }
}

impl From<&str> for Value {
    fn from(value: &str) -> Self {
        Value { data: Arc::new(ValueData::String(String::from(value))) }
    }
}

pub struct Attribute {
    namespace: String,
    name: String,
}

pub struct Datom {
    entity: Option<usize>,
    attribute: Option<Attribute>,
    value: Option<Value>,
    transaction: Option<usize>,
    fact: bool,
}

#[derive(Debug)]
pub enum Binding {
    Unbound,
    Data(Value)
}

pub struct Variable {
    label: String,
    binding: Binding,
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:", self.label)?;
        match &self.binding {
            Unbound => write!(f, "_]"),
            Binding::Data(value) => write!(f, "{}]", value)
        }
    }
}

impl fmt::Debug for Variable {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "[{}:", self.label)?;
        match &self.binding {
            Unbound => write!(f, "_]"),
            Binding::Data(value) => write!(f, "{}]", value)
        }
    }
}

impl From<(&str)> for Variable {
    fn from((label): (&str)) -> Self {
        Variable {
            label: String::from(label),
            binding: Unbound,
        }
    }
}

impl<Type> From<(&str, Type)> for Variable
where Type: Into<Value>
{
    fn from((label, value): (&str, Type)) -> Self {
        Variable {
            label: String::from(label),
            binding: Binding::Data(value.into()),
        }
    }
}


pub struct Pattern {
    pub entity: Option<Variable>,
    pub attribute: Option<Variable>,
    pub value: Option<Variable>,
    pub transaction: Option<Variable>,
    pub fact: Option<Variable>,
}

impl fmt::Debug for Pattern {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if let Some(v) = &self.entity {
            write!(f, "e{}\n", v)?;
        }
        if let Some(v) = &self.attribute {
            write!(f, "a{}\n", v)?;
        }
        if let Some(v) = &self.value {
            write!(f, "v{}\n", v)?;
        }
        if let Some(v) = &self.transaction {
            write!(f, "t{}\n", v)?;
        }
        if let Some(v) = &self.fact {
            write!(f, "f{}\n", v)?;
        }
        Ok(())
    }
}

impl Default for Pattern {
    fn default() -> Self {
        Pattern {
            entity: None,
            attribute: None,
            value: None,
            transaction: None,
            fact: None,
        }
    }
}

#[derive(Debug)]
pub struct Memory {
}

#[derive(Debug)]
pub struct Machine {
    memory: Memory
}