use super::value::{RefVal, Value};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub type RefEnv = Rc<RefCell<Environment>>;

#[derive(Debug)]
pub struct Environment {
    values: HashMap<String, RefVal>,
    parent: Option<RefEnv>,
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
            Some(value.borrow().clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow().get(name)
        } else {
            None
        }
    }

    pub fn get_ref(&self, name: &str) -> Option<RefVal> {
        if let Some(value) = self.values.get(name) {
            Some(value.clone())
        } else if let Some(parent) = &self.parent {
            parent.borrow().get_ref(name)
        } else {
            None
        }
    }

    pub fn set(&mut self, name: &str, value: Value) {
        let value = Rc::new(RefCell::new(value));
        self.values.insert(name.to_owned(), value);
    }

    pub fn assign(&mut self, name: &str, value: Value) {
        if let Some(prev_value) = self.values.get(name) {
            *prev_value.borrow_mut() = value;
        } else if let Some(parent) = &self.parent {
            parent.borrow_mut().assign(name, value)
        }
    }

    pub fn global_contains(&self, name: &str) -> bool {
        if self.values.contains_key(name) {
            true
        } else if let Some(parent) = &self.parent {
            parent.borrow().global_contains(name)
        } else {
            false
        }
    }

    pub fn contains(&self, name: &str) -> bool {
        self.values.contains_key(name)
    }
}
