use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{position::Span, value::Value};

use super::{callable::Callable, function::Function, instance::Instance, EvalResult, Interpreter};

const CONSTRUCTOR_INITIALIZER: &str = "init";

#[derive(Debug, Clone)]
pub struct Class {
    pub id: String,
    pub super_class: Option<Rc<RefCell<Class>>>,
    pub methods: HashMap<String, Function>,
}

impl Class {
    pub fn new(
        id: String,
        super_class: Option<Rc<RefCell<Class>>>,
        methods: HashMap<String, Function>,
    ) -> Self {
        Self {
            id,
            super_class,
            methods,
        }
    }

    pub fn get_method(&self, prop: &str) -> Option<Function> {
        match self.methods.get(prop) {
            Some(m) => Some(m.clone()),
            // find super class
            None => match &self.super_class {
                Some(class) => class.borrow().get_method(prop),
                _ => None,
            },
        }
    }
}

impl Callable for Class {
    fn arity(&self) -> usize {
        unimplemented!()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
        span: Span,
    ) -> EvalResult<Value> {
        let instance = Instance::new(self);

        if let Some(init) = self.get_method(CONSTRUCTOR_INITIALIZER) {
            init.bind(&instance).call(interpreter, arguments, span)?;
        }

        Ok(Value::Instance(instance))
    }
}
