use crate::value::Value;

use super::{EvalResult, Interpreter};

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(&self, interpreter: &mut Interpreter, arguments: Vec<Value>) -> EvalResult<Value>;
}
