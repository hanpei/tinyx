use crate::token::Operator;

pub type Ast = Program;

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
    IfStatement(IfStatement),
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
    pub left: Box<Expr>,
    pub op: Operator,
    pub right: Box<Expr>,
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
    left: Box<Expr>,
    right: Box<Expr>,
}

impl AssignExpr {
    pub fn new(op: Operator, left: Expr, right: Expr) -> Self {
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
