use crate::{
    position::{Pos, Span},
    token::{Token, TokenKind},
    value::Value,
};

#[derive(Debug)]
pub enum ParserError {
    InvalidCharactor(String),
    ParseError(String),
    InvalidToken(String),
    UnexpectedToken(String),
    MissingSemicolon(String),
    LexingError(String),
    InvalidAssignment(String),
    InvalidFunction(String),
}

impl ParserError {
    pub fn invalid_charactor(file: &str, c: char, pos: Pos) -> ParserError {
        Self::InvalidCharactor(format!(
            "invalid character '{}' at {}:{}:{})",
            c, file, pos.ln, pos.col
        ))
    }

    pub fn parse_number_error(file: &str, pos: Pos) -> ParserError {
        Self::ParseError(format!("invalid number at {}:{}:{}", file, pos.ln, pos.col))
    }

    pub fn parse_unicode_error(file: &str, pos: Pos) -> ParserError {
        Self::ParseError(format!(
            "Invalid Unicode escape sequence, at {}:{}:{}",
            file, pos.ln, pos.col
        ))
    }

    pub fn maximum_size_error(file: &str, pos: Pos) -> ParserError {
        Self::ParseError(format!(
            "parse error: elements reach the maximum at {}:{}:{}",
            file, pos.ln, pos.col
        ))
    }
    pub fn invalid_token(file: &str, pos: Pos) -> ParserError {
        Self::InvalidToken(format!("invalid token at {}:{}:{}", file, pos.ln, pos.col))
    }

    pub fn unexpected_token(
        file: &str,
        current: &Token,
        expect: &TokenKind,
        pos: Pos,
    ) -> ParserError {
        Self::UnexpectedToken(format!(
            "unexpected token: [{}], maybe expected [{}] in {}:{}:{}",
            current.raw, expect, file, pos.ln, pos.col
        ))
    }
    pub fn missing_semi(file: &str, pos: Pos) -> ParserError {
        Self::MissingSemicolon(format!(
            "unexpected token (maybe missing semicolon) at {}:{}:{}",
            file, pos.ln, pos.col
        ))
    }
    pub fn invalid_assignment(file: &str, pos: Pos) -> ParserError {
        Self::InvalidAssignment(format!(
            "Invalid left-hand side in assignment expression. at {}:{}:{}",
            file, pos.ln, pos.col
        ))
    }
}

#[derive(Debug)]
pub enum RuntimeError {
    SyntaxError(String, Span),
    ReferenceError(String, Span),
    ReturnedValue(Value), // for return stmt result
    ArgsMismatched(Span),
    Error(String), // TODO: add span
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RuntimeError::SyntaxError(msg, span) => write!(
                f,
                "SyntaxError: {}, at: {}:{}:{}",
                msg, span.filename, span.loc.start.ln, span.loc.start.col
            ),
            RuntimeError::ReferenceError(variabale, span) => write!(
                f,
                "ReferenceError: {} is not defined, at: {}:{}:{}",
                variabale, span.filename, span.loc.start.ln, span.loc.start.col
            ),
            RuntimeError::ArgsMismatched(span) => write!(
                f,
                "SyntaxError: args number mismatched, at: {}:{}:{}",
                span.filename, span.loc.end.ln, span.loc.end.col
            ),
            RuntimeError::Error(msg) => write!(f, "RuntimeError: {}", msg),
            RuntimeError::ReturnedValue(value) => write!(f, "{}", value),
        }
    }
}

pub enum ResolveError {
    Error(String),
    DeclaredError(String, Span),
    SyntaxError(String, Span),
}

impl std::fmt::Display for ResolveError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResolveError::Error(msg) => write!(f, "SyntaxError: {}", msg),
            ResolveError::DeclaredError(name, span) => write!(
                f,
                "SyntaxError:: Identifier '{}' has already been declared, at: {}:{}:{}",
                name, span.filename, span.loc.start.ln, span.loc.start.col
            ),
            ResolveError::SyntaxError(msg, span) => write!(
                f,
                "SyntaxError:: {}, at: {}:{}:{}",
                msg, span.filename, span.loc.start.ln, span.loc.start.col
            ),
        }
    }
}
