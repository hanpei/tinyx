use self::stmt::Statement;
pub type Ast = Program;

mod ast_display;
pub mod expr;
pub mod stmt;

#[cfg(test)]
mod tests;

#[derive(Debug, PartialEq)]
pub struct Program {
    pub body: Vec<Statement>,
}

impl Program {
    pub fn new(node: Vec<Statement>) -> Self {
        Program { body: node }
    }
}
