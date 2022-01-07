#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub ln: usize,
    pub col: usize,
}

impl Pos {
    pub fn new(ln: usize, col: usize) -> Self {
        Self { ln, col }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Loc {
    pub start: Pos,
    pub end: Pos,
}

impl Loc {
    pub fn new(start: Pos, end: Pos) -> Self {
        Self { start, end }
    }
}

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
        write!(f, "[{}] {}", self.kind, self.raw)
    }
}

#[derive(Debug, PartialEq)]
pub enum TokenKind {
    Eof,
    Eol,
    Number,
    Identifier,
    String,
    Operator(Operator),
    Semi,
    None,
    BraceOpen,
    BraceClose,
    ParenOpen,
    ParenClose,
    Keyword(Keyword),
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
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Let,
    If,
    Else,
}

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Keyword::Let => write!(f, "Let"),
            Keyword::If => write!(f, "If"),
            Keyword::Else => write!(f, "Else"),
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
    Assign,

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
            Operator::Add => write!(f, "Add"),
            Operator::Min => write!(f, "Min"),
            Operator::Mul => write!(f, "Mul"),
            Operator::Div => write!(f, "Div"),
            Operator::Assign => write!(f, "Assign"),
            Operator::Not => write!(f, "Not"),
            Operator::Equal => write!(f, "Equal"),
            Operator::NotEqual => write!(f, "NotEqual"),
            Operator::LessThan => write!(f, "LessThan"),
            Operator::LessThanEqual => write!(f, "LessThanEqual"),
            Operator::GreaterThan => write!(f, "GreaterThan"),
            Operator::GreaterThanEqual => write!(f, "GreaterThanEqual"),
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
