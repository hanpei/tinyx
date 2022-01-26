use std::{cell::RefCell, rc::Rc};

use crate::ast::Statement;

use super::Environment;

#[derive(Debug, PartialEq, Clone)]
pub struct Function {
    pub name: Option<String>,
    pub params: Vec<String>,
    pub body: Vec<Statement>,
    pub closure: Rc<RefCell<Environment>>,
}

impl Function {
    pub fn new(
        name: Option<String>,
        params: Vec<String>,
        body: Vec<Statement>,
        closure: Rc<RefCell<Environment>>,
    ) -> Self {
        Function {
            name,
            params,
            body,
            closure,
        }
    }
}
