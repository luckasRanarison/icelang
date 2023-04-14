use std::{cell::RefCell, collections::HashMap, rc::Rc};

use super::value::Value;

pub struct Environment {
    values: HashMap<String, Value>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
            parent: None,
        }
    }

    pub fn from(environment: Rc<RefCell<Environment>>) -> Self {
        Self {
            values: HashMap::new(),
            parent: Some(environment),
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        if let Some(value) = self.values.get(name) {
            Some(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.as_ref().borrow().get(name)
        } else {
            None
        }
    }

    pub fn set(&mut self, name: &str, value: Value) {
        self.values.insert(name.to_owned(), value);
    }

    pub fn assign(&mut self, name: &str, value: Value) -> bool {
        if self.values.contains_key(name) {
            self.set(name, value);
            true
        } else if let Some(parent) = &self.parent {
            parent.as_ref().borrow_mut().assign(name, value)
        } else {
            false
        }
    }

    pub fn local_contains(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }
}
