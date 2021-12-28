use std::result;
use error::Error;

mod ast;
pub mod lexer;
pub mod parser;
pub mod error;
pub mod token;



pub type ParseResult<T> = result::Result<T, Error>;
