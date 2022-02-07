use crate::{
    ast::{FunctionDeclaration, Statement},
    error::RuntimeError,
    value::Value,
};

use super::{
    callable::Callable,
    env::{Env, EnvMethod},
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
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> EvalResult<Value> {
        let Function {
            name: _,
            params,
            body,
            closure,
        } = self;

        if self.arity() != args.len() {
            return Err(RuntimeError::Error("args number mismatched".into()));
        }

        let mut env = Env::extends(closure);
        for (i, arg) in args.into_iter().enumerate() {
            env.define(params[i].clone(), arg)
        }

        match interpreter.execute_block(body, env) {
            Ok(_) => Ok(Value::Null),
            Err(e) => match e {
                RuntimeError::ReturnedValue(v) => Ok(v),
                _ => return Err(e),
            },
        }
    }
}
