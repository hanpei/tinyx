use std::{cell::RefCell, rc::Rc};

use crate::{ast::*, error::EvalError, token::Operator, value::Value};

use super::{
    env::Environment,
    visitor::{ExprResult, ExprVisitor, StmtResult, StmtVisitor},
};

pub struct Interpreter {
    env: Rc<RefCell<Environment>>,
    result: Option<Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            env: Environment::default(),
            result: None,
        }
    }

    pub fn interpret(&mut self, program: Program) -> StmtResult {
        match self.eval_program(program) {
            Ok(()) => match &self.result {
                Some(v) => println!(" > {}", v),
                None => (),
            },
            Err(e) => println!(" > {}", e),
        }
        Ok(())
    }

    fn eval_program(&mut self, program: Program) -> StmtResult {
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
        self.result = Some(value);
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
        let IfStatement {
            test,
            consequent,
            alternate,
        } = stmt;
        let test = self.evaluate(test)?;
        if test.is_truthy() {
            self.execute(&consequent)?;
        } else {
            match alternate {
                Some(stmt) => self.execute(stmt)?,
                None => (),
            }
        }
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStatement) -> StmtResult {
        todo!()
    }

    fn visit_print_stmt(&mut self, expr: &Expr) -> StmtResult {
        let value = self.evaluate(expr)?;
        println!(" > {}", value);
        self.result = None;
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> StmtResult {
        let WhileStmt { test, body } = stmt;

        while self.evaluate(test)?.is_truthy() {
            self.execute(body)?;
        }
        Ok(())
    }
}

impl ExprVisitor for Interpreter {
    // expr
    fn visit_binary(&mut self, binary: &BinaryExpr) -> ExprResult {
        let BinaryExpr { left, op, right } = binary;
        let left = &self.evaluate(left)?;
        let right = &self.evaluate(right)?;

        let op_err = EvalError::SyntaxError(
            format!("invalid operator at [{} {} {}]", left, op.value, right),
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

                _ => return Err(op_err),
            }),
            _ => return Err(op_err),
        }
    }

    fn visit_unary(&mut self, unary: &UnaryExpr) -> ExprResult {
        let UnaryExpr { op, argument } = unary;
        let value = self.evaluate(argument)?;
        let op_err = EvalError::SyntaxError(
            format!("invalid operator at [{} {}]", op.value, value),
            op.span(),
        );

        match op.value {
            Operator::Min => match value {
                Value::Number(n) => Ok(Value::Number(-n)),
                _ => return Err(op_err),
            },
            Operator::Not => match value {
                Value::Boolean(b) => Ok(Value::Boolean(!b)),
                Value::Null => Ok(Value::Boolean(true)),
                _ => Ok(Value::Boolean(false)),
            },
            _ => Err(op_err),
        }
    }

    fn visit_assign(&mut self, assign: &AssignExpr) -> ExprResult {
        let AssignExpr { op, left, right } = assign;
        match op.value {
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

    fn visit_logical(&mut self, expr: &LogicalExpr) -> ExprResult {
        let LogicalExpr { left, op, right } = expr;
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;
        let op_err = EvalError::SyntaxError(
            format!("invalid operator at [{} {} {}]", left, op.value, right),
            op.span(),
        );

        match op.value {
            Operator::Or => {
                if left.is_truthy() {
                    Ok(left)
                } else {
                    Ok(right)
                }
            }
            Operator::And => {
                if !left.is_truthy() {
                    Ok(left)
                } else {
                    Ok(right)
                }
            }
            _ => Err(op_err),
        }
    }
}
