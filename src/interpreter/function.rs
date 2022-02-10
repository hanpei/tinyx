use crate::{ast::Statement, error::RuntimeError, position::Span, value::Value};

use super::{
    callable::Callable,
    env::{Env, EnvMethod},
    instance::Instance,
    EvalResult, Interpreter,
};

#[derive(Debug, Clone)]
pub struct Function {
    pub name: Option<String>,
    pub params: Vec<String>,
    pub body: Vec<Statement>,
    pub closure: Env,
}

impl Function {
    pub fn new(
        name: Option<String>,
        params: Vec<String>,
        body: Vec<Statement>,
        closure: Env,
    ) -> Self {
        Function {
            name,
            params,
            body,
            closure,
        }
    }

    pub fn bind(self, instance: &Instance) -> Self {
        let mut this_env = Env::extends(&self.closure);
        this_env.define("this".to_string(), Value::Instance(instance.clone()));
        Function {
            closure: this_env,
            ..self
        }
    }
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        args: Vec<Value>,
        span: Span,
    ) -> EvalResult<Value> {
        let Function {
            name: _,
            params,
            body,
            closure,
        } = self;

        if self.arity() != args.len() {
            return Err(RuntimeError::ArgsMismatched(span));
        }

        let mut env = Env::extends(closure);
        for (i, arg) in args.into_iter().enumerate() {
            env.define(params[i].clone(), arg)
        }

        match interpreter.execute_block(body, env) {
            Ok(_) => Ok(Value::Null),
            Err(e) => match e {
                RuntimeError::ReturnedValue(v) => Ok(v),
                _ => Err(e),
            },
        }
    }
}
