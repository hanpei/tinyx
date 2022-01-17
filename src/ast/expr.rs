use crate::{
    position::{Span, WithSpan},
    token::Operator,
};

#[derive(Debug, PartialEq)]
pub enum Expr {
    NumericLiteral(NumericLiteral),
    StringLiteral(StringLiteral),
    BooleanLiteral(bool),
    NullLiteral,
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Identifier(Identifier),
    Assign(AssignExpr),
    Call(CallExpr),
}

#[derive(Debug, PartialEq)]
pub struct StringLiteral {
    pub value: String,
    pub span: Span,
}

impl StringLiteral {
    pub fn new(value: String, span: Span) -> Self {
        Self { value, span }
    }
}

#[derive(Debug, PartialEq)]
pub struct NumericLiteral {
    pub value: f64,
    pub span: Span,
}

impl NumericLiteral {
    pub fn new(value: f64, span: Span) -> Self {
        Self { value, span }
    }
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub op: WithSpan<Operator>,
    pub right: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(left: Expr, op: WithSpan<Operator>, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct UnaryExpr {
    pub op: WithSpan<Operator>,
    pub argument: Box<Expr>,
}

impl UnaryExpr {
    pub fn new(op: WithSpan<Operator>, argument: Expr) -> Self {
        Self {
            op,
            argument: Box::new(argument),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    pub name: String,
    pub span: Span,
}

impl Identifier {
    pub fn new(name: String, span: Span) -> Self {
        Self { name, span }
    }
}

#[derive(Debug, PartialEq)]
pub struct AssignExpr {
    pub op: Operator,
    pub left: Identifier,
    pub right: Box<Expr>,
}

impl AssignExpr {
    pub fn new(op: Operator, left: Identifier, right: Expr) -> Self {
        Self {
            op,
            left,
            right: Box::new(right),
        }
    }
}

pub type ArgumentList = Option<Vec<Box<Expr>>>;

#[derive(Debug, PartialEq)]
pub struct CallExpr {
    pub callee: Identifier,
    pub arguments: ArgumentList,
}

impl CallExpr {
    pub fn new(callee: Identifier, arguments: ArgumentList) -> Self {
        Self { callee, arguments }
    }
}
