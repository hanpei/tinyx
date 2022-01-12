use crate::{position::Pos, token::TokenKind};

#[derive(Debug)]
pub enum ParserError {
    InvalidCharactor(String),
    ParseError(String),
    InvalidToken(String),
    UnexpectedToken(String),
    MissingSemicolon(String),
    LexingError(String),
    InvalidAssignment(String),
}

impl ParserError {
    pub fn invalid_charactor(file: &str, c: char, pos: Pos) -> ParserError {
        Self::InvalidCharactor(format!(
            "invalid character {} at {}:{}:{})",
            c, file, pos.ln, pos.col
        ))
    }

    pub fn parse_number_error(file: &str, pos: Pos) -> ParserError {
        Self::ParseError(format!(
            "invalid number at {}:{}:{}",
            file,
            pos.ln,
            pos.col + 1
        ))
    }
    pub fn invalid_token(file: &str, pos: Pos) -> ParserError {
        Self::InvalidToken(format!(
            "invalid token at {}:{}:{}",
            file,
            pos.ln,
            pos.col + 1
        ))
    }

    pub fn unexpected_token(
        file: &str,
        current: &TokenKind,
        expect: &TokenKind,
        pos: Pos,
    ) -> ParserError {
        Self::UnexpectedToken(format!(
            "unexpected token: {}, expected {} at {}:{}:{}",
            current,
            expect,
            file,
            pos.ln,
            pos.col + 1
        ))
    }
    pub fn missing_semi(file: &str, pos: Pos) -> ParserError {
        Self::MissingSemicolon(format!(
            "unexpected token (maybe missing semicolon) at {}:{}:{}",
            file,
            pos.ln,
            pos.col + 1
        ))
    }
    pub fn invalid_assignment(file: &str, pos: Pos) -> ParserError {
        Self::InvalidAssignment(format!(
            "Invalid left-hand side in assignment expression. at {}:{}:{}",
            file,
            pos.ln,
            pos.col + 1
        ))
    }
}

pub enum EvalError {
    SyntaxError(String),
    ReferenceError(String),
}

impl std::fmt::Display for EvalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EvalError::SyntaxError(msg) => write!(f, "Syntax Error: {}", msg),
            EvalError::ReferenceError(variabale) => {
                write!(f, "ReferenceError: {} is not defined", variabale)
            }
        }
    }
}
