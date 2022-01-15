use crate::{
    position::{Span, WithSpan},
    token::Operator,
};
pub type Ast = Program;

mod ast_display;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub body: Vec<Statement>,
}

impl Program {
    pub fn new(node: Vec<Statement>) -> Self {
        Program { body: node }
    }
}

#[derive(Debug, PartialEq)]
pub enum Statement {
    ExprStmt(Expr),
    Block(Vec<Statement>),
    Empty,
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    If(IfStatement),
    Return(ReturnStatement),
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    NumericLiteral(NumericLiteral),
    StringLiteral(StringLiteral),
    BooleanLiteral(bool),
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Identifier(Identifier),
    Assign(AssignExpr),
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
pub struct VariableDeclaration {
    pub id: Identifier,
    pub init: Option<Expr>,
}

impl VariableDeclaration {
    pub fn new(id: Identifier, init: Option<Expr>) -> Self {
        Self { id, init }
    }
}

#[derive(Debug, PartialEq)]
pub struct AssignExpr {
    pub op: Operator,
    pub left: Box<Identifier>,
    pub right: Box<Expr>,
}

impl AssignExpr {
    pub fn new(op: Operator, left: Identifier, right: Expr) -> Self {
        Self {
            op,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct IfStatement {
    test: Box<Expr>,
    consequent: Box<Statement>,
    alternate: Option<Box<Statement>>,
}

impl IfStatement {
    pub fn new(test: Expr, consequent: Statement, alternate: Option<Statement>) -> Self {
        Self {
            test: Box::new(test),
            consequent: Box::new(consequent),
            alternate: match alternate {
                Some(stmt) => Some(Box::new(stmt)),
                None => None,
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct FunctionDeclaration {
    id: Identifier,
    params: Option<Vec<Identifier>>,
    body: Box<Statement>,
}

impl FunctionDeclaration {
    pub fn new(id: Identifier, params: Option<Vec<Identifier>>, body: Statement) -> Self {
        Self {
            id,
            params,
            body: Box::new(body),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ReturnStatement {
    argument: Option<Box<Expr>>,
}

impl ReturnStatement {
    pub fn new(argument: Option<Expr>) -> Self {
        Self {
            argument: match argument {
                Some(expr) => Some(Box::new(expr)),
                None => None,
            },
        }
    }
}
