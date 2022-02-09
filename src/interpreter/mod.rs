mod callable;
pub mod class;
pub mod env;
pub mod function;
pub mod instance;
mod interpreter;
pub mod visitor;

pub use interpreter::Interpreter;

use crate::error::RuntimeError;
pub type EvalResult<T> = std::result::Result<T, RuntimeError>;

#[cfg(test)]
mod tests;
