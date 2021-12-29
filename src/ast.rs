use std::fmt::Debug;

pub type Ast = Program;

#[derive(Debug)]
pub struct Program {
    body: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Program { body: Vec::new() }
    }
    pub fn content(&mut self, node: Vec<Statement>) {
        self.body = node;
    }
}

#[derive(Debug)]
pub enum Statement {
    ExpressionStatement(Expression),
    BlockStatement(Vec<Statement>),
    EmptyStatement,
}

#[derive(Debug)]
pub enum Expression {
    Expr,
    NumericLiteral(f64),
    StringLiteral(String),
    BinaryExpr(BinaryExpr),
    IdentifierExpr(String),
}

// #[derive(Debug)]
// pub struct StringLiteral {
//     value: String,
// }

// impl StringLiteral {
//     pub fn new(value: String) -> Self {
//         Self { value }
//     }
// }

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
pub enum Operator {
    Add,
    Min,
    Mul,
    Div,
}

impl Operator {
    pub fn from(op: &str) -> Self {
        use Operator::*;
        match op {
            "+" => Add,
            "-" => Min,
            "*" => Mul,
            "/" => Div,
            _ => unimplemented!("{}", op),
        }
    }
}

// #[derive(Debug)]
// pub struct Identifier {
//     name: String,
// }

// impl Identifier {
//     pub fn new(name: String) -> Self {
//         Self { name }
//     }
// }
