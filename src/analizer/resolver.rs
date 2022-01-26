use std::collections::HashMap;

use crate::{
    ast::*,
    interpreter::{
        visitor::{ExprVisitor, StmtVisitor},
        Interpreter,
    },
};

use super::{ResolveError, ResolveResult};

#[derive(PartialEq)]
pub enum IdentState {
    Declared,
    Defined,
}

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, IdentState>>,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: vec![],
        }
    }

    pub fn resolve(&mut self, program: &Program) -> ResolveResult<()> {
        self.resolve_block(&program.body)
    }

    fn begin_scope(&mut self) {
        let map = HashMap::new();
        self.scopes.push(map);
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn resolve_stmt(&mut self, stmt: &Statement) -> ResolveResult<()> {
        self.visit_stmt(stmt)
    }

    fn resolve_expr(&mut self, expr: &Expr) -> ResolveResult<()> {
        self.visit_expr(expr)
    }

    fn resolve_block(&mut self, block: &Vec<Statement>) -> ResolveResult<()> {
        for stmt in block {
            self.resolve_stmt(stmt)?;
        }
        Ok(())
    }
}

impl<'a> Resolver<'a> {
    fn declare(&mut self, name: String) {
        if self.scopes.is_empty() {
            return;
        }
        self.scopes
            .last_mut()
            .unwrap()
            .insert(name, IdentState::Declared);
    }

    fn define(&mut self, name: String) {
        if self.scopes.is_empty() {
            return;
        }
        self.scopes
            .last_mut()
            .unwrap()
            .insert(name, IdentState::Defined);
    }

    fn resolve_local(&mut self, ident: &Identifier) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&ident.name) {
                self.interpreter.resolve(ident, i);
                return;
            }
        }
    }

    fn resolve_function(&mut self, decl: &FunctionDeclaration) -> ResolveResult<()> {
        self.begin_scope();
        let FunctionDeclaration {
            id: _,
            params,
            body,
        } = decl;
        for param in params.iter() {
            self.declare(param.name.clone());
            self.define(param.name.clone())
        }
        self.resolve_block(body)?;
        self.end_scope();
        Ok(())
    }
}

impl<'a> StmtVisitor for Resolver<'a> {
    type Item = ResolveResult<()>;

    fn visit_expr_stmt(&mut self, expr: &Expr) -> Self::Item {
        self.resolve_expr(expr)
    }

    fn visit_block(&mut self, block: &Vec<Statement>) -> Self::Item {
        self.begin_scope();
        self.resolve_block(block)?;
        self.end_scope();
        Ok(())
    }

    fn visit_empty(&mut self) -> Self::Item {
        Ok(())
    }

    fn visit_variable_declare(&mut self, decl: &VariableDeclaration) -> Self::Item {
        let VariableDeclaration { id, init } = decl;
        self.declare(id.name.clone());

        if let Some(expr) = init {
            self.resolve_expr(expr)?;
        }
        self.define(id.name.clone());
        Ok(())
    }

    fn visit_function_declare(&mut self, decl: &FunctionDeclaration) -> Self::Item {
        let FunctionDeclaration { id, .. } = decl;
        self.declare(id.name.to_string());
        self.define(id.name.to_string());
        self.resolve_function(decl)?;
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &IfStatement) -> Self::Item {
        let IfStatement {
            test,
            consequent,
            alternate,
        } = stmt;
        self.resolve_expr(test)?;
        self.resolve_stmt(consequent)?;
        if let Some(stmt) = alternate {
            self.resolve_stmt(stmt)?;
        }
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStatement) -> Self::Item {
        let ReturnStatement { argument } = stmt;
        if let Some(expr) = argument {
            self.resolve_expr(expr)?;
        }
        Ok(())
    }

    fn visit_print_stmt(&mut self, expr: &Expr) -> Self::Item {
        self.resolve_expr(expr)
    }

    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Self::Item {
        let WhileStmt { test, body } = stmt;
        self.resolve_expr(test)?;
        self.resolve_stmt(body)?;
        Ok(())
    }
}

impl<'a> ExprVisitor for Resolver<'a> {
    type Item = ResolveResult<()>;

    fn visit_binary(&mut self, binary: &BinaryExpr) -> Self::Item {
        let BinaryExpr { left, op: _, right } = binary;
        self.resolve_expr(left)?;
        self.resolve_expr(right)?;
        Ok(())
    }

    fn visit_unary(&mut self, unary: &UnaryExpr) -> Self::Item {
        let UnaryExpr { op: _, argument } = unary;
        self.resolve_expr(argument)?;
        Ok(())
    }

    fn visit_assign(&mut self, assign: &AssignExpr) -> Self::Item {
        let AssignExpr { op: _, left, right } = assign;
        self.resolve_expr(right)?;
        self.resolve_local(left);
        Ok(())
    }

    fn visit_ident(&mut self, ident: &Identifier) -> Self::Item {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.get(&ident.name) == Some(&IdentState::Declared) {
                return Err(ResolveError::Error(
                    "Can't read local variable in its own initializer.".to_string(),
                ));
            }
        }

        self.resolve_local(ident);
        Ok(())
    }

    fn visit_call(&mut self, call: &CallExpr) -> Self::Item {
        let CallExpr { callee, arguments } = call;
        self.resolve_expr(callee)?;
        for arg in arguments.iter() {
            self.resolve_expr(arg)?;
        }
        Ok(())
    }

    fn visit_logical(&mut self, expr: &LogicalExpr) -> Self::Item {
        let LogicalExpr { left, op: _, right } = expr;
        self.resolve_expr(left)?;
        self.resolve_expr(right)?;
        Ok(())
    }

    fn visit_numeric(&mut self, _lit: &NumericLiteral) -> Self::Item {
        Ok(())
    }

    fn visit_string(&mut self, _lit: &StringLiteral) -> Self::Item {
        Ok(())
    }

    fn visit_boolean(&mut self, _lit: bool) -> Self::Item {
        Ok(())
    }

    fn visit_null(&mut self) -> Self::Item {
        Ok(())
    }
}
