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

#[derive(Clone, PartialEq)]
pub enum FnType {
    None,
    Function,
    Method,
}

#[derive(Clone, PartialEq)]
pub enum ClassType {
    None,
    Class,
    SubClass,
}

pub struct Resolver<'a> {
    interpreter: &'a mut Interpreter,
    scopes: Vec<HashMap<String, IdentState>>,
    fn_type: FnType,
    class_type: ClassType,
}

impl<'a> Resolver<'a> {
    pub fn new(interpreter: &'a mut Interpreter) -> Self {
        Self {
            interpreter,
            scopes: vec![],
            fn_type: FnType::None,
            class_type: ClassType::None,
        }
    }

    pub fn resolve(&mut self, program: &Program) -> ResolveResult<()> {
        self.resolve_block(&program.body)
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn resolve_stmt(&mut self, stmt: &Statement) -> ResolveResult<()> {
        self.walk_stmt(stmt)
    }

    fn resolve_expr(&mut self, expr: &Expr) -> ResolveResult<()> {
        self.walk_expr(expr)
    }

    fn resolve_block(&mut self, block: &[Statement]) -> ResolveResult<()> {
        for stmt in block {
            self.resolve_stmt(stmt)?;
        }
        Ok(())
    }
}

impl<'a> Resolver<'a> {
    fn declare(&mut self, ident: &Identifier) -> ResolveResult<()> {
        if self.scopes.is_empty() {
            return Ok(());
        }

        if self.scopes.last_mut().unwrap().contains_key(&ident.name) {
            return Err(ResolveError::DeclaredError(
                ident.name.to_string(),
                ident.span.clone(),
            ));
        }

        self.scopes
            .last_mut()
            .unwrap()
            .insert(ident.name.clone(), IdentState::Declared);

        Ok(())
    }

    fn define(&mut self, ident: &Identifier) {
        if self.scopes.is_empty() {
            return;
        }
        self.scopes
            .last_mut()
            .unwrap()
            .insert(ident.name.clone(), IdentState::Defined);
    }

    fn resolve_local(&mut self, ident: &Identifier) {
        for (i, scope) in self.scopes.iter().rev().enumerate() {
            if scope.contains_key(&ident.name) {
                self.interpreter.resolve(ident, i);
            }
        }
    }

    fn resolve_function(&mut self, decl: &FunctionDeclaration, ty: FnType) -> ResolveResult<()> {
        let prev = self.fn_type.clone();
        self.fn_type = ty;

        self.begin_scope();
        let FunctionDeclaration {
            id: _,
            params,
            body,
        } = decl;
        for param in params.iter() {
            self.declare(param)?;
            self.define(param)
        }
        self.resolve_block(body)?;
        self.end_scope();

        self.fn_type = prev;
        Ok(())
    }
}

impl<'a> StmtVisitor for Resolver<'a> {
    type Item = ResolveResult<()>;

    fn visit_expr_stmt(&mut self, expr: &Expr) -> Self::Item {
        self.resolve_expr(expr)
    }

    fn visit_block(&mut self, block: &[Statement]) -> Self::Item {
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
        self.declare(id)?;

        if let Some(expr) = init {
            self.resolve_expr(expr)?;
        }
        self.define(id);
        Ok(())
    }

    fn visit_function_declare(&mut self, decl: &FunctionDeclaration) -> Self::Item {
        self.declare(&decl.id)?;
        self.define(&decl.id);
        self.resolve_function(decl, FnType::Function)?;
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
        if self.fn_type == FnType::None {
            return Err(ResolveError::Error("Illegal return statement".to_string()));
        }

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

    fn visit_class_declare(&mut self, class: &ClassDeclaration) -> Self::Item {
        let prev = self.class_type.clone();
        self.class_type = ClassType::Class;

        let ClassDeclaration { id, super_id, body } = class;
        self.declare(id)?;
        self.define(id);

        if let Some(ident) = super_id {
            if ident.name == id.name {
                return Err(ResolveError::SyntaxError(
                    String::from("A class can't inherit from itself."),
                    ident.span.clone(),
                ));
            }
            self.class_type = ClassType::SubClass;

            self.resolve_local(ident);

            // super scope begin
            self.begin_scope();
            self.scopes
                .last_mut()
                .unwrap()
                .insert("super".to_string(), IdentState::Defined);
        }

        self.begin_scope();
        // ???this??????scope???
        self.scopes
            .last_mut()
            .unwrap()
            .insert("this".to_string(), IdentState::Declared);

        for f in body {
            self.resolve_function(f, FnType::Method)?;
        }
        self.end_scope();

        // super scope end
        if let Some(_ident) = super_id {
            self.end_scope();
        }

        self.class_type = prev;
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
            if Some(&IdentState::Declared) == scope.get(&ident.name) {
                return Err(ResolveError::Error(
                    "Can't read local variable in its own initializer.".to_string(),
                ));
            }
        }

        self.resolve_local(ident);
        Ok(())
    }

    fn visit_call(&mut self, call: &CallExpr) -> Self::Item {
        let CallExpr {
            callee,
            arguments,
            span: _,
        } = call;
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

    fn visit_get(&mut self, expr: &GetExpr) -> Self::Item {
        let GetExpr {
            object,
            property: _,
        } = expr;
        self.resolve_expr(object)?;
        Ok(())
    }

    fn visit_set(&mut self, expr: &SetExpr) -> Self::Item {
        let SetExpr {
            object,
            property: _,
            value,
        } = expr;
        self.resolve_expr(object)?;
        self.resolve_expr(value)?;
        Ok(())
    }

    fn visit_this(&mut self, this: &ThisExpr) -> Self::Item {
        if self.class_type == ClassType::None {
            return Err(ResolveError::Error(
                "Can't use 'this' outside of a class.".to_string(),
            ));
        }
        self.resolve_local(&this.into());
        Ok(())
    }

    fn visit_super(&mut self, expr: &SuperExpr) -> Self::Item {
        match self.class_type {
            ClassType::None => {
                return Err(ResolveError::Error(
                    "Can't use 'super' outside of a class.".to_string(),
                ))
            }
            ClassType::Class => {
                return Err(ResolveError::Error(
                    "Can't use 'super' in a class with no superclass.".to_string(),
                ))
            }
            ClassType::SubClass => self.resolve_local(&expr.into()),
        }
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
