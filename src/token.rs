use crate::lexer::{Loc, Pos};

#[derive(Debug)]
pub struct Token {
    pub kind: TokenKind,
    pub raw: String,
    pub loc: Loc,
}

impl Token {
    pub fn new(kind: TokenKind, raw: String, start: Pos, end: Pos) -> Self {
        Self {
            kind,
            raw,
            loc: Loc::new(start, end),
        }
    }
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{{ ")?;
        write!(f, "kind: {}, ", self.kind)?;
        write!(f, "raw: \"{}\", ", self.raw)?;
        write!(f, "start:({}, {}), ", self.loc.start.ln, self.loc.start.col)?;
        write!(f, "end:({}, {})", self.loc.end.ln, self.loc.end.col)?;
        write!(f, " }}")?;
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Eof,
    Eol,
    Number(f64),
    Identifier,
    String,
    Operator(Operator),
    Semi,
    None,
    BraceOpen,
    BraceClose,
    BracketOpen,
    BracketClose,
    Keyword(Keyword),
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Eof => write!(f, "[ Eof ]"),
            TokenKind::Eol => write!(f, "[ Eol ]"),
            TokenKind::Number(_) => write!(f, "[ Number ]"),
            TokenKind::Identifier => write!(f, "[ Identifier ]"),
            TokenKind::String => write!(f, "[ String ]"),
            TokenKind::Operator(_) => write!(f, "[ Operator ]"),
            TokenKind::Semi => write!(f, "[ Semi ]"),
            TokenKind::None => write!(f, "[ None ]"),
            TokenKind::BraceOpen => write!(f, "[ BraceOpen ]"),
            TokenKind::BraceClose => write!(f, "[ BraceClose ]"),
            TokenKind::BracketOpen => write!(f, "[ BracketOpen ]"),
            TokenKind::BracketClose => write!(f, "[ BracketClose ]"),
            TokenKind::Keyword(_) => write!(f, "[ BracketClose ]"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Let,
}

impl Keyword {
    pub fn from_str(value: &str) -> Option<Keyword> {
        use Keyword::*;
        match value {
            "let" => Some(Let),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Add,
    Min,
    Mul,
    Div,
    Assign
}

impl Operator {
    pub fn from_str(op: &str) -> Self {
        use Operator::*;
        match op {
            "+" => Add,
            "-" => Min,
            "*" => Mul,
            "/" => Div,
            "=" => Assign,
            _ => unimplemented!("{}", op),
        }
    }
}
