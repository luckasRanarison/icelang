use super::value::Value;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Environment {
    pub enclosing: Option<Box<Environment>>,
    pub values: HashMap<String, Value>,
    pub breakpoint: bool,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            enclosing: None,
            values: HashMap::new(),
            breakpoint: false,
        }
    }

    pub fn from(environment: Environment) -> Self {
        Self {
            enclosing: Some(Box::new(environment)),
            values: HashMap::new(),
            breakpoint: false,
        }
    }

    pub fn new_enclosing(&mut self, enclosing: Box<Environment>) {
        self.enclosing = Some(enclosing);
    }

    pub fn store(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }

    pub fn assign(&mut self, name: String, value: Value) {
        if self.values.contains_key(&name) {
            self.store(name, value);
        } else if let Some(enclosing) = &mut self.enclosing {
            enclosing.assign(name, value);
        }
    }

    pub fn get(&self, name: String) -> Option<Value> {
        self.values
            .get(&name)
            .cloned()
            .or_else(|| self.enclosing.as_ref().and_then(|e| e.get(name)))
    }

    pub fn local_contains(&self, name: &String) -> bool {
        self.values.contains_key(name)
    }

    pub fn global_contains(&self, name: &String) -> bool {
        self.values.contains_key(name)
            || self
                .enclosing
                .as_ref()
                .map_or(false, |e| e.global_contains(name))
    }

    pub fn return_breakpoint(&mut self) -> Environment {
        let enclosing = self.enclosing.as_mut().unwrap();
        if enclosing.breakpoint {
            enclosing.as_ref().clone()
        } else {
            enclosing.return_breakpoint()
        }
    }
}
