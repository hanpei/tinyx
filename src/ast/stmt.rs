use super::expr::{Expr, Identifier};

#[derive(Debug, PartialEq)]
pub enum Statement {
    ExprStmt(Expr),
    Block(Vec<Statement>),
    Empty,
    VariableDeclaration(VariableDeclaration),
    FunctionDeclaration(FunctionDeclaration),
    If(IfStatement),
    Return(ReturnStatement),
    PrintStmt(Expr),
    While(WhileStmt),
}

#[derive(Debug, PartialEq)]
pub struct WhileStmt {
    pub test: Box<Expr>,
    pub body: Box<Statement>,
}

impl WhileStmt {
    pub fn new(test: Expr, body: Statement) -> Self {
        Self {
            test: Box::new(test),
            body: Box::new(body),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct IfStatement {
    pub test: Box<Expr>,
    pub consequent: Box<Statement>,
    pub alternate: Option<Box<Statement>>,
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
    pub id: Identifier,
    pub params: Option<Vec<Identifier>>,
    pub body: Box<Statement>,
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
    pub argument: Option<Box<Expr>>,
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
