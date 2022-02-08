use std::{borrow::BorrowMut, cell::RefCell, collections::HashMap, rc::Rc};

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

    pub fn get_method(&self, prop: &String) -> Option<&Function> {
        self.methods.get(prop)
    }
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub class: Rc<Class>,
    pub fields: Rc<RefCell<HashMap<String, Value>>>,
}

impl Instance {
    pub fn new(class: Rc<Class>) -> Self {
        Self {
            class: Rc::clone(&class),
            fields: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn get(&self, prop: &String) -> Option<Value> {
        if let Some(v) = self.fields.borrow().get(prop) {
            return Some(v.clone());
        };
        if let Some(method) = self.class.get_method(prop) {
            return Some(Value::Function(method.clone()));
        }
        None
    }

    pub fn set(&mut self, prop: &String, value: Value) {
        (*self.fields).borrow_mut().insert(prop.clone(), value);
    }
}

impl Callable for Class {
    fn arity(&self) -> usize {
        unimplemented!()
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> EvalResult<Value> {
        let mut instance = Instance::new(Rc::new(self.clone()));
        // instance.set(&"init".to_string(), Value::String("init".to_string()));
        Ok(Value::Instance(instance))
    }
}
