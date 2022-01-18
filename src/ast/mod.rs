mod ast_display;
mod expr;
mod stmt;

pub use expr::*;
pub use stmt::*;

#[cfg(test)]
mod tests;

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
