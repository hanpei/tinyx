use std::{collections::HashMap, rc::Rc};

use crate::{
    ast::{FunctionDeclaration, Identifier},
    error::RuntimeError,
    value::Value,
};

use super::{callable::Callable, function::Function, EvalResult, Interpreter};

#[derive(Debug, Clone)]
pub struct Class {
    pub id: Identifier,
    pub methods: HashMap<String, Function>,
}

impl Class {
    pub fn new(id: Identifier, methods: HashMap<String, Function>) -> Self {
        Self { id, methods }
    }

    pub fn get_method(&self, prop: &String) -> Option<&Function> {
        self.methods.get(prop)
    }
}

#[derive(Debug, Clone)]
pub struct Instance {
    pub class: Rc<Class>,
    pub fields: HashMap<String, Value>,
}

impl Instance {
    pub fn new(class: Rc<Class>) -> Self {
        Self {
            class: Rc::clone(&class),
            fields: HashMap::new(),
        }
    }

    pub fn class_name(&self) -> String {
        self.class.id.name.clone()
    }

    pub fn get(&self, prop: &String) -> Option<Value> {
        if let Some(v) = self.fields.get(prop) {
            return Some(v.clone());
        };

        if let Some(method) = self.class.get_method(prop) {
            return Some(Value::Function(method.clone()));
        }

        None
    }
}

impl Callable for Class {
    fn arity(&self) -> usize {
        unimplemented!()
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> EvalResult<Value> {
        let instance = Instance::new(Rc::new(self.clone()));

        Ok(Value::Instance(instance))
    }
}
