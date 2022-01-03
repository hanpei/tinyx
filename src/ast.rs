use std::fmt::Debug;

use crate::token::{Keyword, Operator};

pub type Ast = Program;

#[derive(Debug)]
pub struct Program {
    body: Vec<Statement>,
}

impl Program {
    pub fn new(node: Vec<Statement>) -> Self {
        Program { body: node }
    }
}

#[derive(Debug)]
pub enum Statement {
    ExpressionStatement(Expression),
    BlockStatement(Vec<Statement>),
    EmptyStatement,
    LetStatement(LetStatement),
}

#[derive(Debug)]
pub enum Expression {
    NumericLiteral(f64),
    StringLiteral(String),
    BinaryExpr(BinaryExpr),
    Identifier(Identifier),
}

#[derive(Debug)]
pub struct BinaryExpr {
    left: Box<Expression>,
    op: Operator,
    right: Box<Expression>,
}

impl BinaryExpr {
    pub fn new(left: Expression, op: Operator, right: Expression) -> Self {
        Self {
            left: Box::new(left),
            op,
            right: Box::new(right),
        }
    }
}

#[derive(Debug)]
pub struct Identifier {
    name: String,
}

impl Identifier {
    pub fn new(name: String) -> Self {
        Self { name }
    }
}

#[derive(Debug)]
pub struct LetStatement {
    id: Identifier,
    init: Option<Expression>,
}

impl LetStatement {
    pub fn new(id: Identifier, init: Option<Expression>) -> Self {
        Self { id, init }
    }
}
