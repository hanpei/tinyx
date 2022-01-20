use crate::position::{Loc, Pos};

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
        write!(f, "[{}]: {}", self.kind, self.raw)
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Eof,
    Eol,
    Number,
    Identifier,
    String,
    Boolean,
    Operator(Operator),
    Semi,
    Comma,
    None,
    BraceOpen,
    BraceClose,
    ParenOpen,
    ParenClose,
    Keyword(Keyword),
    Null,
}

impl std::fmt::Display for TokenKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TokenKind::Eof => write!(f, "Eof"),
            TokenKind::Eol => write!(f, "Eol"),
            TokenKind::Number => write!(f, "Number"),
            TokenKind::Identifier => write!(f, "Identifier"),
            TokenKind::String => write!(f, "String"),
            TokenKind::Operator(op) => write!(f, "Operator::{}", op),
            TokenKind::Semi => write!(f, "Semi"),
            TokenKind::None => write!(f, "None"),
            TokenKind::BraceOpen => write!(f, "BraceOpen"),
            TokenKind::BraceClose => write!(f, "BraceClose"),
            TokenKind::ParenOpen => write!(f, "ParenOpen"),
            TokenKind::ParenClose => write!(f, "ParenClose"),
            TokenKind::Keyword(key) => write!(f, "Keyword::{}", key),
            TokenKind::Boolean => write!(f, "Boolean"),
            TokenKind::Comma => write!(f, "Comma"),
            TokenKind::Null => write!(f, "null"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Let,
    If,
    Else,
    Fn,
    Return,
    While,
    Print,
}

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::Let => write!(f, "Let"),
            Keyword::If => write!(f, "If"),
            Keyword::Else => write!(f, "Else"),
            Keyword::Fn => write!(f, "Function"),
            Keyword::Return => write!(f, "Return"),
            Keyword::While => write!(f, "While"),
            Keyword::Print => write!(f, "Print"),
        }
    }
}

impl Keyword {
    pub fn from_str(value: &str) -> Option<Keyword> {
        use Keyword::*;
        match value {
            "let" => Some(Let),
            "if" => Some(If),
            "else" => Some(Else),
            "function" => Some(Fn),
            "return" => Some(Return),
            "while" => Some(While),
            "print" => Some(Print),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operator {
    Add,
    Min,
    Mul,
    Div,
    Assign,

    Or,
    And,

    Not,
    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operator::Add => write!(f, "+"),
            Operator::Min => write!(f, "-"),
            Operator::Mul => write!(f, "*"),
            Operator::Div => write!(f, "/"),
            Operator::Assign => write!(f, "="),
            Operator::Not => write!(f, "!"),
            Operator::Equal => write!(f, "=="),
            Operator::NotEqual => write!(f, "!="),
            Operator::LessThan => write!(f, "<"),
            Operator::LessThanEqual => write!(f, "<="),
            Operator::GreaterThan => write!(f, ">"),
            Operator::GreaterThanEqual => write!(f, ">="),
            Operator::Or => write!(f, "or"),
            Operator::And => write!(f, "and"),
        }
    }
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

            "||" => Or,
            "&&" => And,

            "!" => Not,
            "==" => Equal,
            "!=" => NotEqual,
            "<" => LessThan,
            "<=" => LessThanEqual,
            ">" => GreaterThan,
            ">=" => GreaterThanEqual,
            _ => unimplemented!("{}", op),
        }
    }
}
