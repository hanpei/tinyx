use crate::{
    ast::{ArgumentList, FunctionDeclaration, Identifier, Statement},
    error::RuntimeError,
    value::{Function, Value},
    EvalResult,
};

use super::{interpreter, Interpreter};

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: &ArgumentList) -> EvalResult<()>;
}

impl Callable for Function {
    fn arity(&self) -> usize {
        self.params.len()
    }

    fn call(&self, interpreter: &mut Interpreter, args: &ArgumentList) -> EvalResult<()> {
        let Function {
            name,
            params,
            body,
            scope,
        } = self;

        if self.arity() != args.len() {
            return Err(RuntimeError::Error("args number mismatched".into()));
        }

        // println!("{:?}", scope);

        // println!("params {:?}", params);
        // println!("arguments {:?}", arguments);

        for (i, arg) in args.iter().enumerate() {
            let value = interpreter.evaluate(arg)?;
            scope.borrow_mut().define(params[i].clone(), value)
        }

        interpreter.execute(&*body)
    }
}
