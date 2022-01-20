use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::value::Value;

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

    pub fn extends(outer: &Rc<RefCell<Environment>>) -> Rc<RefCell<Environment>> {
        let env = Environment {
            store: HashMap::new(),
            outer: Some(Rc::clone(outer)),
        };
        Rc::new(RefCell::new(env))
    }

    pub fn define(&mut self, name: String, value: Value) {
        self.store.insert(name, value);
    }

    pub fn lookup(&self, name: &str) -> Option<Value> {
        match self.store.get(name.into()) {
            Some(value) => Some(value.clone()),
            None => match &self.outer {
                Some(outer) => outer.borrow().lookup(name),
                None => None,
            },
        }
    }

    pub fn assign(&mut self, name: &str, value: Value) -> Option<Value> {
        if self.store.contains_key(name.into()) {
            self.store.insert(name.into(), value.clone());
            Some(value)
        } else {
            match &self.outer {
                Some(outer) => {
                    outer.as_ref().borrow_mut().assign(name, value.clone());
                    Some(value)
                }
                None => None,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn env_lookup() {
        let first = Environment::default();
        let second = Environment::extends(&first);

        first
            .borrow_mut()
            .define("foo".to_string(), Value::Number(42.0));

        let a = first.borrow().lookup("foo").unwrap();
        let b = second.borrow().lookup("foo").unwrap();

        assert_eq!(a, Value::Number(42.0));
        assert_eq!(b, Value::Number(42.0));
        assert_eq!(a, b);
    }

    #[test]
    fn env_assign() {
        let first = Environment::default();
        let second = Environment::extends(&first);

        first
            .borrow_mut()
            .define("foo".to_string(), Value::Number(42.0));
        second.borrow_mut().assign("foo", Value::Number(1.0));

        assert_eq!(
            second.borrow_mut().lookup("foo").unwrap(),
            first.borrow().lookup("foo").unwrap()
        );
    }
}
