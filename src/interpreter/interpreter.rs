use std::{collections::HashMap, rc::Rc};

use crate::{ast::*, error::RuntimeError, token::Operator, value::Value};

use super::{
    callable::Callable,
    env::{Env, EnvMethod},
    function::Function,
    visitor::{ExprVisitor, StmtVisitor},
    EvalResult,
};

pub struct Interpreter {
    global: Env,
    env: Env,
    locals: HashMap<String, usize>,
    result: Option<Value>,
}

impl Interpreter {
    pub fn new() -> Self {
        let globals = Env::create();
        Interpreter {
            env: Rc::clone(&globals),
            global: Rc::clone(&globals),
            result: None,
            locals: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, program: Program) -> EvalResult<()> {
        match self.eval_program(program) {
            Ok(()) => match &self.result {
                Some(v) => println!(" > {}", v),
                None => (),
            },
            // Err(e) => eprintln!(" > ERROR: {}", e)
            Err(e) => match e {
                RuntimeError::ReturnedValue(v) => println!(" > {}", v),
                _ => eprintln!(" > ERROR: {}", e),
            },
        }
        Ok(())
    }

    fn eval_program(&mut self, program: Program) -> EvalResult<()> {
        for stmt in &program.body {
            self.execute(stmt)?;
        }
        Ok(())
    }

    fn evaluate(&mut self, expr: &Expr) -> EvalResult<Value> {
        self.walk_expr(expr)
    }

    fn execute(&mut self, stmt: &Statement) -> EvalResult<()> {
        self.walk_stmt(stmt)
    }

    pub(super) fn execute_block(&mut self, block: &Vec<Statement>, env: Env) -> EvalResult<()> {
        let prev_env = Rc::clone(&self.env);
        self.env = env;
        for stmt in block {
            match self.execute(stmt) {
                Ok(_) => (),
                Err(e) => {
                    self.env = prev_env;
                    return Err(e);
                }
            }
        }
        self.env = prev_env;
        Ok(())
    }

    pub fn resolve(&mut self, ident: &Identifier, depth: usize) {
        // 这里key一定要有唯一性, 不能直接用String,否则会被覆盖,
        // 重写了Display trait, 用identifier.to_string()当做key: "name@ln:col"
        self.locals.insert(ident.to_string(), depth);
        println!("var: {:?}", self.locals);
    }

    fn look_up_variable(&self, ident: &Identifier) -> EvalResult<Value> {
        let value = match self.locals.get(&ident.to_string()) {
            // lookup in locals
            Some(distance) => self.env.get_at(*distance, &ident.name),
            // lookup in global
            None => self.global.borrow_mut().get(&ident.name),
        };
        match value {
            Some(v) => Ok(v),
            None => Err(RuntimeError::ReferenceError(
                ident.name.clone(),
                ident.span.clone(),
            )),
        }
    }
}

impl StmtVisitor for Interpreter {
    type Item = EvalResult<()>;

    // stmt
    fn visit_expr_stmt(&mut self, expr: &Expr) -> Self::Item {
        let value = self.evaluate(expr)?;
        self.result = Some(value); // 把ExpressionStatement最后一个expression的结果显示出来
        Ok(())
    }

    fn visit_block(&mut self, block: &Vec<Statement>) -> Self::Item {
        self.execute_block(block, Env::extends(&self.env))
    }

    fn visit_variable_declare(&mut self, decl: &VariableDeclaration) -> Self::Item {
        let VariableDeclaration { id, init } = decl;
        let Identifier { name, .. } = id;
        let value = match init {
            Some(expr) => self.evaluate(expr)?,
            None => Value::Null,
        };
        self.env.define(name.clone(), value);
        Ok(())
    }

    fn visit_function_declare(&mut self, decl: &FunctionDeclaration) -> Self::Item {
        let FunctionDeclaration { id, params, body } = decl;

        let closure = Rc::clone(&self.env);

        let func = Function::new(
            Some(id.name.clone()),
            params.iter().map(|i| i.name.to_string()).collect(),
            body.clone(),
            closure,
        );

        self.env.define(id.name.clone(), Value::Function(func));

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
            self.execute(&consequent)?;
        } else {
            if let Some(stmt) = alternate {
                self.execute(stmt)?
            }
        }
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStatement) -> Self::Item {
        let ReturnStatement { argument } = stmt;

        if let Some(expr) = argument {
            self.result = Some(self.evaluate(expr)?);
        }
        match &self.result {
            Some(v) => Err(RuntimeError::ReturnedValue(v.clone())),
            None => Err(RuntimeError::ReturnedValue(Value::Null)),
        }
    }

    fn visit_print_stmt(&mut self, expr: &Expr) -> Self::Item {
        let value = self.evaluate(expr)?;
        println!(" > print: {}", value);
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
            (Value::String(l), Value::String(r)) => Ok(match op.value {
                Operator::Add => Value::String(format!("{}{}", l, r)),
                _ => unimplemented!(),
            }),

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
                if let Some(distance) = self.locals.get(&left.to_string()) {
                    match self.env.assign_at(*distance, name, value.clone()) {
                        true => Ok(value),
                        false => Err(RuntimeError::ReferenceError(name.clone(), span.clone())),
                    }
                } else {
                    match self.env.borrow_mut().assign(name, value.clone()) {
                        true => Ok(value),
                        false => Err(RuntimeError::ReferenceError(name.clone(), span.clone())),
                    }
                }
            }
            _ => unreachable!(),
        }
    }

    fn visit_ident(&mut self, ident: &Identifier) -> Self::Item {
        self.look_up_variable(ident)
    }

    fn visit_call(&mut self, call: &CallExpr) -> Self::Item {
        let CallExpr { callee, arguments } = call;
        let value = self.evaluate(callee)?;

        let mut list = Vec::new();
        for arg in arguments.iter() {
            list.push(self.evaluate(arg)?);
        }

        match value {
            Value::Function(function) => function.call(self, list),
            _ => return Err(RuntimeError::Error("invalid callee".to_string())),
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
        Ok(Value::String(lit.value.clone()))
    }
    fn visit_boolean(&mut self, lit: bool) -> Self::Item {
        Ok(Value::Boolean(lit))
    }
    fn visit_null(&mut self) -> Self::Item {
        Ok(Value::Null)
    }
}
