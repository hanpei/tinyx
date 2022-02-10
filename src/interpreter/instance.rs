use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::value::Value;

use super::class::Class;

#[derive(Debug, Clone)]
pub struct Instance {
    pub class: Rc<Class>,
    pub fields: Rc<RefCell<HashMap<String, Value>>>,
}

impl Instance {
    pub fn new(class: &Class) -> Self {
        Self {
            class: Rc::new(class.clone()),
            fields: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn get(&self, prop: &str) -> Option<Value> {
        if let Some(v) = self.fields.borrow().get(prop) {
            return Some(v.clone());
        };
        if let Some(method) = self.class.get_method(prop) {
            let m = method.bind(self);
            return Some(Value::Function(m));
        }
        None
    }

    pub fn set(&mut self, prop: &str, value: Value) {
        (*self.fields).borrow_mut().insert(prop.to_string(), value);
    }
}
