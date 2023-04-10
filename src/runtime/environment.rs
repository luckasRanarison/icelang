use super::value::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    pub values: HashMap<String, Value>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new(),
        }
    }

    pub fn from(environment: Environment) -> Self {
        Self {
            enclosing: Some(Box::new(environment)),
            values: HashMap::new(),
        }
    }

    pub fn new_enclosing(&mut self, enclosing: Box<Environment>) {
        self.enclosing = Some(enclosing);
    }

    pub fn store(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: String) -> Option<Value> {
        self.values
            .get(&name)
            .cloned()
            .or_else(|| self.enclosing.as_ref().and_then(|e| e.get(name)))
    }

    pub fn contains(&self, name: &String) -> bool {
        self.values.contains_key(name)
    }
}
