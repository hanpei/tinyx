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
    Dot,
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
            TokenKind::Dot => write!(f, "Dot"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Keyword {
    Nil,
    Let,
    If,
    Else,
    Fn,
    Return,
    While,
    Print,
    Class,
    Extends,
    This,
}

impl std::fmt::Display for Keyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Keyword::*;
        match self {
            Let => write!(f, "Let"),
            If => write!(f, "If"),
            Else => write!(f, "Else"),
            Fn => write!(f, "Function"),
            Return => write!(f, "Return"),
            While => write!(f, "While"),
            Print => write!(f, "Print"),
            Class => write!(f, "Class"),
            Extends => write!(f, "Extends"),
            This => write!(f, "This"),
            _ => write!(f, ""),
        }
    }
}

impl From<&String> for Keyword {
    fn from(s: &String) -> Self {
        use Keyword::*;
        match s.as_str() {
            "let" => Let,
            "if" => If,
            "else" => Else,
            "fn" => Fn,
            "return" => Return,
            "while" => While,
            "print" => Print,
            "class" => Class,
            "extends" => Extends,
            "this" => This,
            _ => Nil,
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

impl From<&String> for Operator {
    fn from(str: &String) -> Self {
        use Operator::*;
        match str.as_str() {
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
            _ => unimplemented!("{}", str),
        }
    }
}
