use crate::token::{Pos, TokenKind};

#[derive(Debug)]
pub enum Error {
    InvalidCharactor(String),
    ParseError(String),
    InvalidToken(String),
    UnexpectedToken(String),
    MissingSemicolon(String),
    LexingError(String),
    InvalidAssignment(String),
}

impl Error {
    pub fn invalid_charactor(file: &str, c: char, pos: Pos) -> Error {
        Self::InvalidCharactor(format!(
            "invalid character {} at {}:{}:{})",
            c, file, pos.ln, pos.col
        ))
    }

    pub fn parse_number_error(file: &str, pos: Pos) -> Error {
        Self::ParseError(format!(
            "invalid number at {}:{}:{}",
            file,
            pos.ln,
            pos.col + 1
        ))
    }
    pub fn invalid_token(file: &str, pos: Pos) -> Error {
        Self::InvalidToken(format!(
            "invalid token at {}:{}:{}",
            file,
            pos.ln,
            pos.col + 1
        ))
    }

    pub fn unexpected_token(file: &str, kind: &TokenKind, expect: &TokenKind, pos: Pos) -> Error {
        Self::UnexpectedToken(format!(
            "unexpected token: {}, expect is {} at {}:{}:{}",
            kind,
            expect,
            file,
            pos.ln,
            pos.col + 1
        ))
    }
    pub fn missing_semi(file: &str, pos: Pos) -> Error {
        Self::MissingSemicolon(format!(
            "unexpected token (maybe missing semicolon) at {}:{}:{}",
            file,
            pos.ln,
            pos.col + 1
        ))
    }
    pub fn invalid_assignment(file: &str, pos: Pos) -> Error {
        Self::InvalidAssignment(format!(
            "Invalid left-hand side in assignment expression. at {}:{}:{}",
            file,
            pos.ln,
            pos.col + 1
        ))
    }
}
