use crate::{ast::*, error::EvalError, token::Operator, value::Value};

mod env;

type EvalResult = Result<Value, EvalError>;
pub struct Evaluator {}

pub trait Eval {
    type Item;
    fn eval_program(&mut self, program: &Program) -> Self::Item;
    fn eval_stmt(&mut self, stmt: &Statement) -> Self::Item;
    fn eval_expr(&mut self, expr: &Expr) -> Self::Item;
    fn eval_binary_expr(&mut self, binary: &BinaryExpr) -> Self::Item;

    fn eval_arithmatic(&mut self, op: &Operator, left: f64, right: f64) -> Self::Item;
    fn eval_string_concat(&mut self, op: &Operator, left: String, right: String) -> Self::Item;
}

// TODO: eval
impl Evaluator {
    pub fn new() -> Self {
        Evaluator {}
    }
    pub fn eval(&mut self, program: &Program) {
        self.eval_program(program);
    }
}

impl Eval for Evaluator {
    type Item = EvalResult;

    fn eval_program(&mut self, program: &Program) -> Self::Item {
        // program.body.iter().map(|stmt| self.eval_stmt(stmt))
        let mut result = Value::Null;
        for stmt in &program.body {
            result = self.eval_stmt(stmt)?
        }
        println!(" > {}", result);
        Ok(result)
    }

    fn eval_stmt(&mut self, stmt: &Statement) -> Self::Item {
        match stmt {
            Statement::ExprStmt(expr) => self.eval_expr(expr),
            Statement::Block(_) => todo!(),
            Statement::Empty => todo!(),
            Statement::VariableDeclaration(_) => todo!(),
            Statement::FunctionDeclaration(_) => todo!(),
            Statement::If(_) => todo!(),
            Statement::Return(_) => todo!(),
        }
    }

    fn eval_expr(&mut self, expr: &Expr) -> Self::Item {
        match expr {
            Expr::NumericLiteral(n) => Ok(Value::Float(*n)),
            Expr::StringLiteral(s) => Ok(Value::String(s.to_string())),
            Expr::BooleanLiteral(_) => todo!(),
            Expr::Binary(binary) => self.eval_binary_expr(binary),
            Expr::Unary(_) => todo!(),
            Expr::Identifier(_) => todo!(),
            Expr::Assign(_) => todo!(),
        }
    }

    fn eval_binary_expr(&mut self, binary: &BinaryExpr) -> Self::Item {
        let BinaryExpr { left, op, right } = binary;
        let left = self.eval_expr(left)?;
        let right = self.eval_expr(right)?;

        match (left, right) {
            (Value::Float(l), Value::Float(r)) => self.eval_arithmatic(op, l, r),
            (Value::String(l), Value::String(r)) => self.eval_string_concat(op, l, r),
            _ => unimplemented!(),
        }
    }

    fn eval_arithmatic(&mut self, op: &Operator, left: f64, right: f64) -> Self::Item {
        let result = match op {
            Operator::Add => left + right,
            Operator::Min => left - right,
            Operator::Mul => left * right,
            Operator::Div => left / right,
            _ => unimplemented!(),
        };
        Ok(Value::Float(result))
    }

    fn eval_string_concat(&mut self, op: &Operator, left: String, right: String) -> Self::Item {
        let result = match op {
            Operator::Add => {
                let mut str = String::from(left);
                str.push_str(&right);
                str
            }
            _ => unimplemented!(),
        };
        Ok(Value::String(result))
    }
}
