use error::{ParserError, RuntimeError};
use value::Value;

pub mod ast;
pub mod error;
pub mod interpreter;
pub mod lexer;
pub mod parser;
pub mod position;
pub mod token;
pub mod value;

pub type ParseResult<T> = std::result::Result<T, ParserError>;
pub type EvalResult<T> = std::result::Result<T, RuntimeError>;
