use crate::token::Operator;

pub type Ast = Program;

#[derive(Debug, PartialEq)]
pub struct Program {
    body: Vec<Statement>,
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
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    NumericLiteral(f64),
    StringLiteral(String),
    Binary(BinaryExpr),
    Identifier(Identifier),
    Assign(AssignExpr),
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpr {
    left: Box<Expr>,
    op: Operator,
    right: Box<Expr>,
}

impl BinaryExpr {
    pub fn new(left: Expr, op: Operator, right: Expr) -> Self {
        Self {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Identifier {
    name: String,
}

impl Identifier {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug, PartialEq)]
pub struct VariableDeclaration {
    id: Identifier,
    init: Option<Expr>,
}

impl VariableDeclaration {
    pub fn new(id: Identifier, init: Option<Expr>) -> Self {
        Self { id, init }
    }
}

#[derive(Debug, PartialEq)]
pub struct AssignExpr {
    op: Operator,
    left: Identifier,
    right: Box<Expr>,
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
