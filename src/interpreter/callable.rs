use crate::{
    ast::Statement,
    error::RuntimeError,
    value::{Function, Value},
    EvalResult,
};

use super::{Environment, Interpreter};

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> EvalResult<()>;
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(&self, interpreter: &mut Interpreter, args: Vec<Value>) -> EvalResult<()> {
        let Function {
            name: _,
            params,
            body,
            closure,
        } = self;

        if self.arity() != args.len() {
            return Err(RuntimeError::Error("args number mismatched".into()));
        }

        let env = Environment::extends(&interpreter.env);
        // println!("callable env: {:?}", env);
        for (i, arg) in args.into_iter().enumerate() {
            env.borrow_mut().define(params[i].clone(), arg)
        }

        interpreter.execute_block(body, env)
    }
}
