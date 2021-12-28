#[derive(Debug, Clone)]
pub enum Token {
    Eof,
    Eol,
    Number(f64),
    Identifier(String),
    String(String),
    Operator(String),
    Semi,
    None,
    BraceOpen,
    BraceClose,
}

impl Token {
    pub fn kind(&mut self) -> TokenKind {
        match self {
            Token::Eof => TokenKind::Eof,
            Token::Eol => TokenKind::Eol,
            Token::Number(_) => TokenKind::Number,
            Token::Identifier(_) => TokenKind::Identifier,
            Token::String(_) => TokenKind::String,
            Token::Operator(_) => TokenKind::Operator,
            Token::Semi => TokenKind::Semi,
            Token::None => TokenKind::None,
            Token::BraceOpen => TokenKind::BraceOpen,
            Token::BraceClose => TokenKind::BraceClose,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Eof,
    Eol,
    Number,
    Identifier,
    String,
    Operator,
    Semi,
    None,
    BraceOpen,
    BraceClose,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Eof => write!(f, "[Eof]"),
            TokenKind::Eol => write!(f, "[Eol]"),
            TokenKind::Number => write!(f, "[Number]"),
            TokenKind::Identifier => write!(f, "[Identifier]"),
            TokenKind::String => write!(f, "[String]"),
            TokenKind::Operator => write!(f, "[Operator]"),
            TokenKind::Semi => write!(f, "[Semi]"),
            TokenKind::None => write!(f, "[None]"),
            TokenKind::BraceOpen => write!(f, "[BraceOpen]"),
            TokenKind::BraceClose => write!(f, "[BraceClose]"),
        }
    }
}
