#[derive(Debug)]
pub enum Token {
    Eof,
    Number(f64),
    Identifier(String),
    Eol,
    Operator(String),
}
