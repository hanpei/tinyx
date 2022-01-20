use crate::{
    ast::{ArgumentList, FunctionDeclaration, Identifier, Statement},
    value::{Function, Value},
    EvalResult,
};

use super::{interpreter, Interpreter};

pub trait Callable {
    fn arity(&self) -> i8;
    fn call(&self, interpreter: &mut Interpreter, arguments: &ArgumentList) -> EvalResult<()>;
}

impl Callable for Function {
    fn arity(&self) -> i8 {
        self.params.len() as i8
    }

    fn call(&self, interpreter: &mut Interpreter, args: &ArgumentList) -> EvalResult<()> {
        let Function {
            name: _,
            params,
            body,
            scope,
        } = self;

        // println!("params {:?}", params);
        // println!("arguments {:?}", arguments);

        for (i, arg) in args.iter().enumerate() {
            let value = interpreter.evaluate(arg)?;
            scope.borrow_mut().define(params[i].clone(), value)
        }

        interpreter.execute(&*body)
    }
}
