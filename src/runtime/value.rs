use std::{
    fmt,
    ops::{Add, Div, Mul, Rem, Sub},
};

#[derive(Debug, PartialEq, PartialOrd, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Boolean(bool),
    Null,
    Array(Vec<Value>),
}

impl Value {
    pub fn get_type(&self) -> String {
        let value_type = match self {
            Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Boolean(_) => "boolean",
            Value::Null => "null",
            Value::Array(_) => "array",
        };

        value_type.to_string()
    }

    pub fn is_number(&self) -> bool {
        match self {
            Value::Number(_) => true,
            _ => false,
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
            Value::String(value) => write!(f, "\"{value}\""),
            Value::Boolean(value) => write!(f, "{:?}", value),
            Value::Null => write!(f, "null"),
            Value::Array(items) => {
                let mut s = String::new();
                let mut iter = items.iter();
                if let Some(item) = iter.next() {
                    s.push_str(&format!("{}", item));
                    for item in iter {
                        s.push_str(&format!(", {}", item));
                    }
                }
                write!(f, "[{}]", s)
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
