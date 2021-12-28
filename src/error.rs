use crate::token::{Token, TokenKind};

#[derive(Debug)]
pub enum Error {
    InvalidCharactor(String),
    ParseError(String),
    InvalidToken(String),
    UnexpectedToken(String),
}

impl Error {
    pub fn invalid_charactor(file: &str, c: char, pos: (usize, usize)) -> Error {
        Self::InvalidCharactor(format!(
            "unexpected character {} at {}:{}:{})",
            c, file, pos.0, pos.1
        ))
    }

    pub fn parse_number_error(file: &str, s: &str, pos: (usize, usize)) -> Error {
        Self::ParseError(format!(
            "invalid number {} at {}:{}:{}",
            s, file, pos.0, pos.1
        ))
    }
    pub fn invalid_token(file: &str, pos: (usize, usize)) -> Error {
        Self::InvalidToken(format!("invalid token at {}:{}:{}", file, pos.0, pos.1))
    }

    pub fn unexpected_token(
        file: &str,
        kind: TokenKind,
        expect: TokenKind,
        pos: (usize, usize),
    ) -> Error {
        Self::UnexpectedToken(format!(
            "unexpected token: {}, expect is {} at {}:{}:{}",
            kind, expect, file, pos.0, pos.1
        ))
    }
}
