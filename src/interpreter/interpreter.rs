use std::{borrow::Borrow, cell::RefCell, rc::Rc};

use crate::{
    ast::*,
    error::RuntimeError,
    token::Operator,
    value::{Function, Value},
    EvalResult,
};

use super::{
    callable::Callable,
    env::Environment,
    visitor::{ExprVisitor, StmtVisitor},
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

    pub fn interpret(&mut self, program: Program) -> EvalResult<()> {
        match self.eval_program(program) {
            Ok(()) => match &self.result {
                Some(v) => println!(" > {}", v),
                None => (),
            },
            Err(e) => match e {
                RuntimeError::ReturnedValue(v) => println!(" > {}", v),
                _ => eprintln!(" > {}", e),
            },
        }
        Ok(())
    }

    pub fn set_env(&mut self, env: Rc<RefCell<Environment>>) {
        self.env = env;
    }

    fn eval_program(&mut self, program: Program) -> EvalResult<()> {
        for stmt in &program.body {
            self.execute(stmt)?;
        }
        Ok(())
    }

    pub fn evaluate(&mut self, expr: &Expr) -> EvalResult<Value> {
        self.visit_expr(expr)
    }

    pub fn execute(&mut self, stmt: &Statement) -> EvalResult<()> {
        self.visit_stmt(stmt)
    }
    pub fn execute_block(
        &mut self,
        block: &Vec<Statement>,
        env: Rc<RefCell<Environment>>,
    ) -> EvalResult<()> {
        let prev_env = Rc::clone(&env);
        self.env = Environment::extends(&env);

        for stmt in block {
            self.execute(stmt)?;
        }
        self.env = prev_env;
        Ok(())
    }
}

impl StmtVisitor for Interpreter {
    type Item = EvalResult<()>;

    // stmt
    fn visit_expr_stmt(&mut self, expr: &Expr) -> Self::Item {
        let value = self.evaluate(expr)?;
        // 把ExpressionStatement最后一个expression的结果显示出来
        self.result = Some(value);
        Ok(())
    }

    fn visit_block(&mut self, block: &Vec<Statement>) -> Self::Item {
        let prev_env = Rc::clone(&self.env);
        self.env = Environment::extends(&self.env);
        for stmt in block {
            self.execute(stmt)?;
        }
        self.env = prev_env;
        Ok(())
    }

    fn visit_variable_declare(&mut self, decl: &VariableDeclaration) -> Self::Item {
        let VariableDeclaration { id, init } = decl;
        let Identifier { name, .. } = id;
        let value = match init {
            Some(expr) => self.evaluate(expr)?,
            None => Value::Null,
        };
        self.env
            .borrow_mut()
            .define(name.to_string(), value.clone());

        Ok(())
    }

    fn visit_function_declare(&mut self, decl: &FunctionDeclaration) -> Self::Item {
        let FunctionDeclaration { id, params, body } = decl;

        let closure = Rc::clone(&self.env);

        let func = Function::new(
            Some(id.name.to_string()),
            params.iter().map(|i| i.name.to_string()).collect(),
            *body.clone(),
            closure,
        );

        self.env
            .borrow_mut()
            .define(id.name.to_string(), Value::Function(func));

        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &IfStatement) -> Self::Item {
        let IfStatement {
            test,
            consequent,
            alternate,
        } = stmt;
        let test = self.evaluate(test)?;
        if test.is_truthy() {
            let prev_env = Rc::clone(&self.env);
            self.env = Environment::extends(&self.env);
            self.execute(&consequent)?;
            self.env = prev_env;
        } else {
            match alternate {
                Some(stmt) => {
                    let prev_env = Rc::clone(&self.env);
                    self.env = Environment::extends(&self.env);
                    self.execute(stmt)?;
                    self.env = prev_env;
                }
                None => (),
            }
        }
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStatement) -> Self::Item {
        let ReturnStatement { argument } = stmt;

        if let Some(expr) = argument {
            let value = self.evaluate(expr)?;
            self.result = Some(value);
        }
        match &self.result {
            Some(v) => Err(RuntimeError::ReturnedValue(v.clone())),
            None => Err(RuntimeError::ReturnedValue(Value::Null)),
        }
    }

    fn visit_print_stmt(&mut self, expr: &Expr) -> Self::Item {
        let value = self.evaluate(expr)?;
        println!(" > {}", value);
        self.result = None;
        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Self::Item {
        let WhileStmt { test, body } = stmt;

        while self.evaluate(test)?.is_truthy() {
            self.execute(body)?;
        }
        Ok(())
    }

    fn visit_empty(&mut self) -> Self::Item {
        Ok(())
    }
}

impl ExprVisitor for Interpreter {
    type Item = EvalResult<Value>;

    // expr
    fn visit_binary(&mut self, binary: &BinaryExpr) -> Self::Item {
        let BinaryExpr { left, op, right } = binary;
        let left = &self.evaluate(left)?;
        let right = &self.evaluate(right)?;

        let op_err = RuntimeError::SyntaxError(
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

    fn visit_unary(&mut self, unary: &UnaryExpr) -> Self::Item {
        let UnaryExpr { op, argument } = unary;
        let value = self.evaluate(argument)?;
        let op_err = RuntimeError::SyntaxError(
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

    fn visit_assign(&mut self, assign: &AssignExpr) -> Self::Item {
        let AssignExpr { op, left, right } = assign;
        match op.value {
            Operator::Assign => {
                let Identifier { name, span } = left;
                let value = self.evaluate(&*right)?;
                match self.env.borrow_mut().assign(&name, value.clone()) {
                    Some(v) => Ok(v),
                    None => return Err(RuntimeError::ReferenceError(name.into(), span.clone())),
                }
            }
            _ => unimplemented!(),
        }
    }

    fn visit_ident(&mut self, ident: &Identifier) -> Self::Item {
        let Identifier { name, span } = ident;
        match self.env.as_ref().borrow().lookup(&name) {
            Some(value) => Ok(value),
            None => return Err(RuntimeError::ReferenceError(name.to_string(), span.clone())),
        }
    }

    fn visit_call(&mut self, call: &CallExpr) -> Self::Item {
        let CallExpr { callee, arguments } = call;
        let value = self.evaluate(callee)?;

        let mut list = Vec::new();
        for arg in arguments.iter() {
            list.push(self.evaluate(arg)?);
        }

        match value {
            Value::Function(function) => function.call(self, list)?,
            _ => return Err(RuntimeError::Error("invalid callee".to_string())),
        }

        match &self.result {
            Some(r) => Ok(r.clone()),
            _ => Ok(Value::Null),
        }
    }

    fn visit_logical(&mut self, expr: &LogicalExpr) -> Self::Item {
        let LogicalExpr { left, op, right } = expr;
        let left = self.evaluate(left)?;
        let right = self.evaluate(right)?;
        let op_err = RuntimeError::SyntaxError(
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

    fn visit_numeric(&mut self, lit: &NumericLiteral) -> Self::Item {
        Ok(Value::Number(lit.value))
    }
    fn visit_string(&mut self, lit: &StringLiteral) -> Self::Item {
        Ok(Value::String(lit.value.to_string()))
    }
    fn visit_boolean(&mut self, lit: bool) -> Self::Item {
        Ok(Value::Boolean(lit))
    }
    fn visit_null(&mut self) -> Self::Item {
        Ok(Value::Null)
    }
}
