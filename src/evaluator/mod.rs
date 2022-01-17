use std::{
    cell::RefCell,
    fmt::{Binary, Result},
    rc::Rc,
    result,
};

use crate::{
    ast::*,
    error::EvalError,
    position::{Span, WithSpan},
    token::Operator,
    value::Value,
    EvalResult,
};

use self::{
    env::Environment,
    visitor::{ExprResult, ExprVisitor, StmtResult, StmtVisitor},
};

mod env;
mod visitor;

pub struct Interpreter {
    env: Rc<RefCell<Environment>>,
    result: Value,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: Environment::default(),
            result: Value::Null,
        }
    }

    pub fn interpret(&mut self, program: Program) -> StmtResult {
        match self.eval_program(program) {
            Ok(()) => println!(" > {}", self.result),
            Err(e) => println!(" > {}", e),
        }
        Ok(())
    }

    fn eval_program(&mut self, program: Program) -> result::Result<(), EvalError> {
        for stmt in &program.body {
            self.execute(stmt)?;
        }
        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> ExprResult {
        self.visit_expr(expr)
    }

    fn execute(&mut self, stmt: &Statement) -> StmtResult {
        self.visit_stmt(stmt)
    }
}

impl StmtVisitor for Interpreter {
    // stmt
    fn visit_expr_stmt(&mut self, expr: &Expr) -> StmtResult {
        let value = self.evaluate(expr)?;
        // 把ExpressionStatement最后一个expression的结果显示出来
        self.result = value;
        Ok(())
    }

    fn visit_block(&mut self, block: &Vec<Statement>) -> StmtResult {
        let prev_env = Rc::clone(&self.env);
        self.env = Environment::extends(&self.env);
        for stmt in block {
            self.execute(stmt)?;
        }
        self.env = prev_env;
        Ok(())
    }

    fn visit_variable_declare(&mut self, decl: &VariableDeclaration) -> StmtResult {
        let VariableDeclaration { id, init } = decl;
        let Identifier { name, .. } = id;
        let value = match init {
            Some(expr) => self.evaluate(expr)?,
            None => Value::Null,
        };
        self.env.borrow_mut().define(&name, value.clone());
        Ok(())
    }

    fn visit_function_declare(&mut self, decl: &FunctionDeclaration) -> StmtResult {
        todo!()
    }

    fn visit_if_stmt(&mut self, stmt: &IfStatement) -> StmtResult {
        todo!()
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStatement) -> StmtResult {
        todo!()
    }
}

impl ExprVisitor for Interpreter {
    // expr
    fn visit_binary(&mut self, binary: &BinaryExpr) -> ExprResult {
        let BinaryExpr { left, op, right } = binary;
        let left = &self.evaluate(left)?;
        let right = &self.evaluate(right)?;

        let op_err = EvalError::SyntaxError(
            format!(
                "invalid operator at \"{}\" {} \"{}\"",
                left, op.value, right
            ),
            op.span(),
        );

        match (left, right) {
            (Value::String(_), Value::String(_)) => todo!(),

            (Value::Number(l), Value::Number(r)) => Ok(match op.value {
                Operator::Add => Value::Number(l + r),
                Operator::Min => Value::Number(l - r),
                Operator::Mul => Value::Number(l * r),
                Operator::Div => Value::Number(l / r),

                Operator::Equal => Value::Boolean(l == r),
                Operator::NotEqual => Value::Boolean(l != r),
                Operator::LessThan => Value::Boolean(l < r),
                Operator::LessThanEqual => Value::Boolean(l <= r),
                Operator::GreaterThan => Value::Boolean(l > r),
                Operator::GreaterThanEqual => Value::Boolean(l >= r),

                Operator::Assign | Operator::Not => return Err(op_err),
            }),
            _ => return Err(op_err),
        }
    }

    fn visit_unary(&mut self, unary: &UnaryExpr) -> ExprResult {
        todo!()
    }

    fn visit_assign(&mut self, assign: &AssignExpr) -> ExprResult {
        let AssignExpr { op, left, right } = assign;
        match op {
            Operator::Assign => {
                let Identifier { name, span } = left;
                let value = self.evaluate(&*right)?;
                match self.env.borrow_mut().assign(&name, value.clone()) {
                    Some(v) => Ok(v),
                    None => return Err(EvalError::ReferenceError(name.into(), span.clone())),
                }
            }
            _ => unimplemented!(),
        }
    }

    fn visit_ident(&mut self, ident: &Identifier) -> ExprResult {
        let Identifier { name, span } = ident;
        match self.env.borrow().lookup(&name) {
            Some(value) => Ok(value),
            None => return Err(EvalError::ReferenceError(name.to_string(), span.clone())),
        }
    }

    fn visit_call(&mut self, call: &CallExpr) -> ExprResult {
        todo!()
    }
}
