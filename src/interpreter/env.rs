use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::RuntimeError, position::Span, value::Value, EvalResult};

#[derive(PartialEq, Clone, Debug)]
pub struct Environment {
    store: HashMap<String, Value>,
    outer: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Environment {
            store: HashMap::new(),
            outer: None,
        }
    }

    pub fn default() -> Rc<RefCell<Environment>> {
        Rc::new(RefCell::new(Environment::new()))
    }

    pub fn extends(env: &Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
        let env = Environment {
            store: HashMap::new(),
            outer: Some(Rc::clone(env)),
        };
        Rc::new(RefCell::new(env))
    }

    pub fn define(&mut self, name: String, value: Value) {
        // println!("ENV defined:  {:#?}", name);
        self.store.insert(name, value);
    }

    pub fn lookup(&mut self, name: &str) -> Option<Value> {
        // println!("ENV lookup:  {:#?}", name);
        match self.store.get(name.into()) {
            Some(value) => Some(value.clone()),
            None => match &self.outer {
                Some(outer) => outer.borrow_mut().lookup(name),
                None => None,
            },
        }
    }

    pub fn assign(&mut self, name: &str, value: Value) -> bool {
        if self.store.contains_key(name) {
            self.store.insert(name.to_string(), value);
            true
        } else {
            match &self.outer {
                Some(outer) => {
                    outer.borrow_mut().assign(name, value);
                    true
                }
                None => false,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_lookup() {}

    #[test]
    fn env_assign() {}
}
