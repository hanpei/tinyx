use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::value::Value;

use super::{callable::Callable, function::Function, EvalResult, Interpreter};

#[derive(Debug, Clone)]
pub struct Class {
    pub id: String,
    pub methods: HashMap<String, Function>,
}

impl Class {
    pub fn new(id: String, methods: HashMap<String, Function>) -> Self {
        Self { id, methods }
    }

    pub fn get_method(&self, prop: &str) -> Option<&Function> {
        self.methods.get(prop)
    }
}

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
            let m = method.clone().bind(self);
            return Some(Value::Function(m));
        }
        None
    }

    pub fn set(&mut self, prop: &str, value: Value) {
        (*self.fields).borrow_mut().insert(prop.to_string(), value);
    }
}

impl Callable for Class {
    fn arity(&self) -> usize {
        unimplemented!()
    }

    fn call(&self, _interpreter: &mut Interpreter, _arguments: Vec<Value>) -> EvalResult<Value> {
        let instance = Instance::new(self);

        Ok(Value::Instance(instance))
    }
}
