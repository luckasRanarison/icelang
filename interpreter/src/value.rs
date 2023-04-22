use super::{environment::RefEnv, error::RuntimeError};
use lexer::tokens::Token;
use parser::ast::{Expression, FunctionDeclaration};

use std::{
    borrow::Borrow,
    cell::RefCell,
    collections::HashMap,
    fmt,
    ops::{self, Add, Div, Mul, Rem, Sub},
    rc::Rc,
};

pub type RefVal = Rc<RefCell<Value>>;

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Array(Vec<RefVal>),
    Object(Object),
    Function(Function),
    Builtin(Builtin),
    Range(Range),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub declaration: FunctionDeclaration,
}

impl PartialEq for Function {
    fn eq(&self, other: &Self) -> bool {
        self.declaration.token == other.declaration.token
    }
}

impl PartialOrd for Function {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Less)
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct Object {
    pub values: HashMap<String, RefVal>,
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        if self.values.len() > other.values.len() {
            Some(std::cmp::Ordering::Greater)
        } else {
            Some(std::cmp::Ordering::Less)
        }
    }
}

#[derive(Clone)]
pub struct Builtin {
    pub name: &'static str,
    pub args: usize,
    pub function: fn(&RefEnv, token: &Token, &Vec<Expression>) -> Result<Value, RuntimeError>,
}

impl fmt::Debug for Builtin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Builtin")
            .field("name", &self.name)
            .field("args", &self.args)
            .field("function", &"<native function>")
            .finish()
    }
}

impl PartialEq for Builtin {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl PartialOrd for Builtin {
    fn partial_cmp(&self, _other: &Self) -> Option<std::cmp::Ordering> {
        Some(std::cmp::Ordering::Less)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Range {
    NumberRange(ops::Range<i64>),
    CharRange(ops::Range<char>),
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Range::NumberRange(a), Range::NumberRange(b)) => {
                if a.end > b.end {
                    Some(std::cmp::Ordering::Greater)
                } else {
                    Some(std::cmp::Ordering::Less)
                }
            }
            (Range::CharRange(a), Range::CharRange(b)) => {
                if a.end > b.end {
                    Some(std::cmp::Ordering::Greater)
                } else {
                    Some(std::cmp::Ordering::Less)
                }
            }
            _ => None,
        }
    }
}

impl Value {
    pub fn get_type(&self) -> String {
        let value_type = match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Boolean(_) => "boolean",
            Value::Null => "null",
            Value::Array(_) => "array",
            Value::Object(_) => "object",
            Value::Function(_) | Value::Builtin(_) => "function",
            Value::Range(_) => "range",
        };

        value_type.to_string()
    }

    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
        }
    }

    pub fn is_iterable(&self) -> bool {
        match self {
            Value::String(_)
            | Value::Array(_)
            | Value::Object(_)
            | Value::Range(_)
            | Value::Number(_) => true,
            _ => false,
        }
    }

    pub fn iter(&self) -> Box<dyn Iterator<Item = (Value, Value)> + '_> {
        match self {
            Value::Number(value) => {
                let range = 0..*value as i64;
                Box::new(
                    range.into_iter().enumerate().map(|(key, value)| {
                        (Value::Number(key as f64), Value::Number(value as f64))
                    }),
                )
            }
            Value::Range(range) => match range {
                Range::NumberRange(value) => {
                    Box::new(value.clone().into_iter().enumerate().map(|(key, value)| {
                        (Value::Number(key as f64), Value::Number(value as f64))
                    }))
                }
                Range::CharRange(value) => {
                    Box::new(value.clone().into_iter().enumerate().map(|(key, value)| {
                        (Value::Number(key as f64), Value::String(value.into()))
                    }))
                }
            },
            Value::String(string) => {
                Box::new(string.chars().enumerate().map(|(index, value)| {
                    (Value::Number(index as f64), Value::String(value.into()))
                }))
            }
            Value::Array(array) => Box::new(array.iter().enumerate().map(|(index, value)| {
                (Value::Number(index as f64), value.as_ref().borrow().clone())
            })),
            Value::Object(object) => {
                Box::new(object.values.borrow().iter().map(|(key, value)| {
                    (Value::String(key.clone()), value.as_ref().borrow().clone())
                }))
            }
            _ => Box::new(std::iter::empty()),
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(value) => {
                let mut s = value.to_string();

                if s.ends_with(".0") {
                    s.truncate(s.len() - 2);
                }

                write!(f, "{s}")
            }
            Value::String(value) => write!(f, "{}", value),
            Value::Boolean(value) => write!(f, "{:?}", value),
            Value::Null => write!(f, "null"),
            Value::Array(items) => {
                let mut s = String::new();
                let mut iter = items.iter();
                if let Some(item) = iter.next() {
                    s.push_str(&format!("{}", item.as_ref().borrow()));
                    for item in iter {
                        s.push_str(&format!(", {}", item.as_ref().borrow()));
                    }
                }
                write!(f, "[{}]", s)
            }
            Value::Function(function) => {
                let name = match &function.declaration.token {
                    Some(token) => &token.lexeme,
                    None => "anonymous",
                };
                write!(f, "[Function {}]", name)
            }
            Value::Builtin(builtin) => {
                write!(f, "[Function {}]", builtin.name)
            }
            Value::Object(object) => {
                let mut s = String::new();
                let mut iter = object.values.iter();
                if let Some((key, value)) = iter.next() {
                    s.push_str(&format!("{}: {}", key, value.as_ref().borrow()));
                    for (key, value) in iter {
                        s.push_str(&format!(", {}: {}", key, value.as_ref().borrow()));
                    }
                }
                write!(f, "{{ {} }}", s)
            }
            Value::Range(range) => {
                let (start, end) = match range {
                    Range::NumberRange(value) => (
                        Value::Number(value.start as f64),
                        Value::Number(value.end as f64),
                    ),
                    Range::CharRange(value) => (
                        Value::String(value.start.to_string()),
                        Value::String(value.end.to_string()),
                    ),
                };
                write!(f, "{}..{}", start, end)
            }
        }
    }
}

impl Mul for Value {
    type Output = Option<Value>;

    fn mul(self, rhs: Value) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a * b)),
            _ => None,
        }
    }
}

impl Div for Value {
    type Output = Option<Value>;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a / b)),
            _ => None,
        }
    }
}

impl Sub for Value {
    type Output = Option<Value>;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a - b)),
            _ => None,
        }
    }
}

impl Add for Value {
    type Output = Option<Value>;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a + b)),
            (Value::String(a), Value::Number(b)) => Some(Value::String(format!("{}{}", a, b))),
            (Value::Number(a), Value::String(b)) => Some(Value::String(format!("{}{}", a, b))),
            (Value::String(a), Value::String(b)) => Some(Value::String(format!("{}{}", a, b))),
            _ => None,
        }
    }
}

impl Rem for Value {
    type Output = Option<Value>;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Value::Number(a), Value::Number(b)) => Some(Value::Number(a % b)),
            _ => None,
        }
    }
}
