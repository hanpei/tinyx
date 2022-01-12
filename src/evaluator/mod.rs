use crate::{ast::*, error::EvalError, token::Operator, value::Value};

use self::env::Environment;

mod env;

type EvalResult = std::result::Result<Value, EvalError>;
pub struct Evaluator {
    env: Environment,
}

// pub trait Eval {
//     type Item;
//     fn eval_program(&mut self, program: &Program) -> Self::Item;
//     fn eval_stmt(&mut self, stmt: &Statement) -> Self::Item;
//     fn eval_expr(&mut self, expr: &Expr) -> Self::Item;
//     fn eval_binary_expr(&mut self, binary: &BinaryExpr) -> Self::Item;

//     fn eval_arithmatic(&mut self, op: &Operator, left: f64, right: f64) -> Self::Item;
//     fn eval_string_concat(&mut self, op: &Operator, left: String, right: String) -> Self::Item;
// }

impl Evaluator {
    pub fn new() -> Self {
        Evaluator {
            env: Environment::new(),
        }
    }
    pub fn eval(&mut self, program: Program) {
        match self.eval_program(program) {
            Ok(result) => println!(" > {}", result),
            Err(e) => println!(" > {}", e),
        }
    }
}

impl Evaluator {
    fn eval_program(&mut self, program: Program) -> EvalResult {
        // program.body.iter().map(|stmt| self.eval_stmt(stmt))
        let mut result = Value::Null;
        for stmt in program.body {
            result = self.eval_stmt(stmt)?
        }
        Ok(result)
    }

    fn eval_stmt(&mut self, stmt: Statement) -> EvalResult {
        match stmt {
            Statement::ExprStmt(expr) => self.eval_expr(expr),
            Statement::Block(_) => todo!(),
            Statement::Empty => todo!(),
            Statement::VariableDeclaration(decl) => self.eval_var_decl(decl),
            Statement::FunctionDeclaration(_) => todo!(),
            Statement::If(_) => todo!(),
            Statement::Return(_) => todo!(),
        }
    }

    fn eval_var_decl(&mut self, stmt: VariableDeclaration) -> EvalResult {
        let VariableDeclaration { id, init } = stmt;
        let Identifier { name } = id;
        let value = match init {
            Some(expr) => self.eval_expr(expr)?,
            None => Value::Null,
        };
        self.env.define(name.to_string(), value.clone());
        Ok(value)
    }

    fn eval_expr(&mut self, expr: Expr) -> EvalResult {
        match expr {
            Expr::NumericLiteral(n) => Ok(Value::Float(n)),
            Expr::StringLiteral(s) => Ok(Value::String(s.value.to_string())),
            Expr::BooleanLiteral(_) => todo!(),
            Expr::Binary(binary) => self.eval_binary_expr(binary),
            Expr::Unary(_) => todo!(),
            Expr::Identifier(ident) => self.eval_identifier(ident),
            Expr::Assign(expr) => self.eval_assign(expr),
        }
    }

    fn eval_binary_expr(&mut self, binary: BinaryExpr) -> EvalResult {
        let BinaryExpr { left, op, right } = binary;
        let left = self.eval_expr(*left)?;
        let right = self.eval_expr(*right)?;

        match (left, right) {
            (Value::Float(l), Value::Float(r)) => self.eval_arithmatic(op, l, r),
            (Value::String(l), Value::String(r)) => self.eval_string_concat(op, l, r),
            _ => unimplemented!(),
        }
    }

    fn eval_arithmatic(&mut self, op: Operator, left: f64, right: f64) -> EvalResult {
        let result = match op {
            Operator::Add => left + right,
            Operator::Min => left - right,
            Operator::Mul => left * right,
            Operator::Div => left / right,
            _ => unimplemented!(),
        };
        Ok(Value::Float(result))
    }

    fn eval_string_concat(&mut self, op: Operator, left: String, right: String) -> EvalResult {
        let result = match op {
            Operator::Add => {
                let mut str = String::from(left);
                str.push_str(&right);
                str
            }
            _ => {
                return Err(EvalError::SyntaxError(format!(
                    "invalid operator at \"{}\" {} \"{}\"",
                    left, op, right
                )))
            }
        };
        Ok(Value::String(result))
    }

    fn eval_identifier(&mut self, ident: Identifier) -> EvalResult {
        let Identifier { name } = ident;
        match self.env.lookup(&name) {
            Some(value) => Ok(value.clone()),
            None => return Err(EvalError::ReferenceError(name)),
        }
    }

    fn eval_assign(&mut self, expr: AssignExpr) -> EvalResult {
        let AssignExpr { op, left, right } = expr;
        match op {
            Operator::Assign => match *left {
                Expr::Identifier(ident) => {
                    let Identifier { name } = ident;
                    match self.env.lookup(&name) {
                        Some(_) => {
                            let value = self.eval_expr(*right)?;
                            self.env.define(name, value.clone());
                            Ok(value)
                        }
                        None => return Err(EvalError::ReferenceError(name)),
                    }
                }
                _ => Err(EvalError::SyntaxError(
                    "Invalid left-hand side in assignment".into(),
                )),
            },
            _ => unimplemented!(),
        }
    }
}
