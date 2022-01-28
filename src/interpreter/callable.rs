use crate::{error::RuntimeError, value::Value};

use super::{
    env::{Env, EnvMethod},
    function::Function,
    EvalResult, Interpreter,
};

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> EvalResult<Value>;
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
