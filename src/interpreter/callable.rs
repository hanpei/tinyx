use crate::{
    ast::{ArgumentList, FunctionDeclaration, Identifier, Statement},
    value::{Function, Value},
    EvalResult,
};

use super::{interpreter, Interpreter};

pub trait Callable {
    fn arity(&self) -> i8;
    fn call(interpreter: &Interpreter, arguments: ArgumentList) -> EvalResult<Value>;
}

impl Callable for Function {
    fn arity(&self) -> i8 {
        todo!()
    }

    fn call(interpreter: &Interpreter, args: ArgumentList) -> EvalResult<Value> {
        todo!()
    }
}
