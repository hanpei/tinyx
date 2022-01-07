use crate::{ast::*, value::Value};

mod env;

pub struct Evaluator {}

pub trait Eval {
    type Item;
    fn eval_program(&mut self, program: &Program) -> Self::Item;
    fn eval_stmt(&mut self, stmt: &Statement) -> Self::Item;
    fn eval_expr(&mut self, expr: &Expr) -> Self::Item;
    fn eval_binary_expr(&mut self, expr: &BinaryExpr) -> Self::Item;
}

// TODO: eval
impl Evaluator {
    pub fn eval(&mut self) {
        unimplemented!()
    }
}

impl Eval for Evaluator {
    type Item = Value;

    fn eval_program(&mut self, program: &Program) -> Self::Item {
        // program.body.iter().map(|stmt| self.eval_stmt(stmt))
        todo!()
    }

    fn eval_stmt(&mut self, stmt: &Statement) -> Self::Item {
        todo!()
    }

    fn eval_expr(&mut self, expr: &Expr) -> Self::Item {
        todo!()
    }

    fn eval_binary_expr(&mut self, expr: &BinaryExpr) -> Self::Item {
        todo!()
    }
}
