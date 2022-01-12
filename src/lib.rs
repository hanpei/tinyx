use error::ParserError;
use std::result;

pub mod ast;
pub mod error;
pub mod evaluator;
pub mod lexer;
pub mod parser;
pub mod position;
pub mod token;
pub mod value;

pub type ParseResult<T> = result::Result<T, ParserError>;
