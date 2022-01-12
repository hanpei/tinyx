use std::{
    borrow::{Borrow, BorrowMut},
    cell::RefCell,
    collections::HashMap,
    rc::Rc,
};

use crate::value::Value;

pub struct Environment {
    store: HashMap<String, Value>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            parent: None,
        }
    }

    pub fn define(&mut self, key: String, value: Value) -> Option<Value> {
        self.store.insert(key, value)
    }

    pub fn lookup(&mut self, key: &String) -> Option<&Value> {
        self.store.get(key)
    }

    pub fn assign(&mut self, key: &String, value: Value) {
        if self.store.contains_key(key) {
            self.store.insert(key.into(), value);
        } else {
            if let Some(ref outer) = self.parent {
                outer.as_ref().borrow_mut().assign(key, value)
            }
        }
    }

    pub fn extend(env: Rc<RefCell<Environment>>) -> Self {
        Environment {
            store: HashMap::new(),
            parent: Some(env),
        }
    }
}
