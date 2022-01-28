use crate::error::ParserError;

pub mod expr;
pub mod literal;
pub mod parser;
pub mod stmt;

const MAXIMUM_ARGS: usize = 255;

pub type ParseResult<T> = std::result::Result<T, ParserError>;
