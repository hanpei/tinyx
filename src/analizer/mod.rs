use crate::error::ResolveError;

pub mod resolver;

pub type ResolveResult<T> = std::result::Result<T, ResolveError>;
