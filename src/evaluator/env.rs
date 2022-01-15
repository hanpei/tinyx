use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::EvalError, value::Value};

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

    pub fn extends(outer: &Rc<RefCell<Environment>>) -> Self {
        Environment {
            store: HashMap::new(),
            outer: Some(Rc::clone(outer)),
        }
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

    pub fn define(&mut self, name: &str, value: Value) {
        self.store.insert(name.into(), value);
    }

    pub fn assign(&mut self, name: &str, value: Value) {
        if self.store.contains_key(name.into()) {
            self.store.insert(name.into(), value);
        } else {
            match &self.outer {
                Some(outer) => outer.as_ref().borrow_mut().assign(name, value),
                None => unreachable!(),
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

        first.borrow_mut().define("foo", Value::Int(42));

        let a = first.borrow().lookup("foo").unwrap();
        let b = second.lookup("foo").unwrap();

        assert_eq!(a, Value::Int(42));
        assert_eq!(b, Value::Int(42));
        assert_eq!(a, b);
    }

    #[test]
    fn env_assign() {
        let first = Environment::default();
        let mut second = Environment::extends(&first);

        first.borrow_mut().define("foo", Value::Int(42));
        second.assign("foo", Value::Int(1));

        assert_eq!(
            second.lookup("foo").unwrap(),
            first.borrow().lookup("foo").unwrap()
        );
    }
}
