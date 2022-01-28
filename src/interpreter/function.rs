use crate::ast::Statement;

use super::env::Env;

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub name: Option<String>,
    pub params: Vec<String>,
    pub body: Vec<Statement>,
    pub closure: Env,
}

impl Function {
    pub fn new(
        name: Option<String>,
        params: Vec<String>,
        body: Vec<Statement>,
        closure: Env,
    ) -> Self {
        Function {
            name,
            params,
            body,
            closure,
        }
    }
}
