pub mod resolver;

pub type ResolveResult<T> = std::result::Result<T, ResolveError>;

pub enum ResolveError {
    Error(String),
}

impl std::fmt::Display for ResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolveError::Error(msg) => write!(f, "ReferenceError: {}", msg),
        }
    }
}
