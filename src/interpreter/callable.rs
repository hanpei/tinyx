use crate::{position::Span, value::Value};

use super::{EvalResult, Interpreter};

pub trait Callable {
    fn arity(&self) -> usize;
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
        span: Span,
    ) -> EvalResult<Value>;
}
