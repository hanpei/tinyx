use std::result;
use error::Error;

mod ast;
pub mod lexer;
pub mod error;
pub mod token;
pub mod parser;


pub type ParseResult<T> = result::Result<T, Error>;
