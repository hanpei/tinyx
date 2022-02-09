use std::collections::HashMap;

use crate::value::Value;

use super::{callable::Callable, function::Function, instance::Instance, EvalResult, Interpreter};

const CONSTRUCTOR_INITIALIZER: &str = "init";

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

impl Callable for Class {
    fn arity(&self) -> usize {
        unimplemented!()
    }

    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> EvalResult<Value> {
        let instance = Instance::new(self);

        if let Some(init) = self.get_method(CONSTRUCTOR_INITIALIZER) {
            init.clone().bind(&instance).call(interpreter, arguments)?;
        }

        Ok(Value::Instance(instance))
    }
}
