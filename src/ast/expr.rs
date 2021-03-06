use crate::{
    position::{Span, WithSpan},
    token::Operator,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    NumericLiteral(NumericLiteral),
    StringLiteral(StringLiteral),
    BooleanLiteral(bool),
    NullLiteral,
    Binary(BinaryExpr),
    Logical(LogicalExpr),
    Unary(UnaryExpr),
    Identifier(Identifier),
    Assign(AssignExpr),
    Call(CallExpr),
    Get(GetExpr),
    Set(SetExpr),
    This(ThisExpr),
    Super(SuperExpr),
}

#[derive(Debug, PartialEq, Clone)]
pub struct StringLiteral {
    pub value: String,
    pub span: Span,
}

impl StringLiteral {
    pub fn new(value: String, span: Span) -> Self {
        Self { value, span }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct NumericLiteral {
    pub value: f64,
    pub span: Span,
}

impl NumericLiteral {
    pub fn new(value: f64, span: Span) -> Self {
        Self { value, span }
    }
}

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub struct Identifier {
    pub name: String,
    pub span: Span,
}

impl Identifier {
    pub fn new(name: String, span: Span) -> Self {
        Self { name, span }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct AssignExpr {
    pub op: WithSpan<Operator>,
    pub left: Identifier,
    pub right: Box<Expr>,
}

impl AssignExpr {
    pub fn new(op: WithSpan<Operator>, left: Identifier, right: Expr) -> Self {
        Self {
            op,
            left,
            right: Box::new(right),
        }
    }
}

pub type ArgumentList = Vec<Expr>;
#[derive(Debug, PartialEq, Clone)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub arguments: ArgumentList,
    pub span: Span,
}

impl CallExpr {
    pub fn new(callee: Expr, arguments: ArgumentList, span: Span) -> Self {
        Self {
            callee: Box::new(callee),
            arguments,
            span,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct LogicalExpr {
    pub left: Box<Expr>,
    pub op: WithSpan<Operator>,
    pub right: Box<Expr>,
}

impl LogicalExpr {
    pub fn new(left: Expr, op: WithSpan<Operator>, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct GetExpr {
    pub object: Box<Expr>,
    pub property: Identifier,
}

impl GetExpr {
    pub fn new(object: Expr, property: Identifier) -> Self {
        Self {
            object: Box::new(object),
            property,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SetExpr {
    pub object: Box<Expr>,
    pub property: Identifier,
    pub value: Box<Expr>,
}

impl SetExpr {
    pub fn new(object: Expr, property: Identifier, value: Expr) -> Self {
        Self {
            object: Box::new(object),
            property,
            value: Box::new(value),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct ThisExpr {
    pub span: Span,
}

impl ThisExpr {
    pub fn new(span: Span) -> Self {
        Self { span }
    }
}

impl From<&ThisExpr> for Identifier {
    fn from(this: &ThisExpr) -> Self {
        Identifier {
            name: "this".to_string(),
            span: this.span.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct SuperExpr {
    pub method: Identifier,
    pub span: Span,
}

impl SuperExpr {
    pub fn new(method: Identifier, span: Span) -> Self {
        Self { method, span }
    }
}

impl From<&SuperExpr> for Identifier {
    fn from(this: &SuperExpr) -> Self {
        Identifier {
            name: "super".to_string(),
            span: this.span.clone(),
        }
    }
}
