use crate::{lexer::Pos, token::TokenKind};

#[derive(Debug)]
pub enum Error {
    InvalidCharactor(String),
    ParseError(String),
    InvalidToken(String),
    UnexpectedToken(String),
    MissingSemicolon(String),
    LexingError(String),
}

impl Error {
    pub fn invalid_charactor(file: &str, c: char, pos: Pos) -> Error {
        Self::InvalidCharactor(format!(
            "unexpected character {} at {}:{}:{})",
            c, file, pos.ln, pos.col
        ))
    }

    pub fn parse_number_error(file: &str, pos: Pos) -> Error {
        Self::ParseError(format!("invalid number at {}:{}:{}", file, pos.ln, pos.col))
    }
    pub fn invalid_token(file: &str, pos: Pos) -> Error {
        Self::InvalidToken(format!("invalid token at {}:{}:{}", file, pos.ln, pos.col))
    }

    pub fn unexpected_token(file: &str, kind: &TokenKind, expect: &TokenKind, pos: Pos) -> Error {
        Self::UnexpectedToken(format!(
            "unexpected token: {}, expect is {} at {}:{}:{}",
            kind, expect, file, pos.ln, pos.col
        ))
    }
    pub fn missing_semi(file: &str, pos: Pos) -> Error {
        Self::MissingSemicolon(format!(
            "maybe missing semicolon at {}:{}:{}",
            file, pos.ln, pos.col
        ))
    }
}
