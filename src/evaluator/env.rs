use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::value::Value;

pub struct Environment {
    symbol_table: HashMap<String, Value>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            symbol_table: HashMap::new(),
            parent: None,
        }
    }

    pub fn define(&mut self, key: String, value: Value) -> Option<Value> {
        self.symbol_table.insert(key, value)
    }

    pub fn lookup(&mut self, key: &String) -> Option<&Value> {
        self.symbol_table.get(key)
    }
}
