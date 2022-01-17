use crate::{
    ast::{expr::*, stmt::*},
    value::Value,
    EvalResult,
};

pub type StmtResult = EvalResult<()>;
pub type ExprResult = EvalResult<Value>;

// stmt ===============================
pub trait StmtVisitor {
    fn visit_stmt(&mut self, stmt: &Statement) -> StmtResult {
        match stmt {
            Statement::ExprStmt(expr) => self.visit_expr_stmt(expr),
            Statement::Block(block) => self.visit_block(block),
            Statement::Empty => Ok(()),
            Statement::VariableDeclaration(delc) => self.visit_variable_declare(delc),
            Statement::FunctionDeclaration(delc) => self.visit_function_declare(delc),
            Statement::If(i) => self.visit_if_stmt(i),
            Statement::Return(r) => self.visit_return_stmt(r),
            Statement::PrintStmt(expr) => self.visit_print_stmt(expr),
            Statement::While(w) => self.visit_while_stmt(w),
        }
    }

    fn visit_expr_stmt(&mut self, expr: &Expr) -> StmtResult;
    fn visit_block(&mut self, block: &Vec<Statement>) -> StmtResult;
    fn visit_variable_declare(&mut self, decl: &VariableDeclaration) -> StmtResult;
    fn visit_function_declare(&mut self, decl: &FunctionDeclaration) -> StmtResult;
    fn visit_if_stmt(&mut self, stmt: &IfStatement) -> StmtResult;
    fn visit_return_stmt(&mut self, stmt: &ReturnStatement) -> StmtResult;
    fn visit_print_stmt(&mut self, expr: &Expr) -> StmtResult;
    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> StmtResult;
}

// expr ===============================
pub trait ExprVisitor {
    fn visit_expr(&mut self, expr: &Expr) -> ExprResult {
        match expr {
            Expr::NumericLiteral(n) => self.visit_numeric(n),
            Expr::StringLiteral(s) => self.visit_string(s),
            Expr::BooleanLiteral(b) => self.visit_boolean(*b),
            Expr::Binary(binary) => self.visit_binary(binary),
            Expr::Unary(u) => self.visit_unary(u),
            Expr::Identifier(ident) => self.visit_ident(ident),
            Expr::Assign(a) => self.visit_assign(a),
            Expr::Call(c) => self.visit_call(c),
            Expr::NullLiteral => self.visit_null(),
        }
    }

    fn visit_binary(&mut self, binary: &BinaryExpr) -> ExprResult;
    fn visit_unary(&mut self, unary: &UnaryExpr) -> ExprResult;
    fn visit_assign(&mut self, assign: &AssignExpr) -> ExprResult;
    fn visit_ident(&mut self, ident: &Identifier) -> ExprResult;
    fn visit_call(&mut self, call: &CallExpr) -> ExprResult;

    // literal ===============================
    fn visit_numeric(&mut self, lit: &NumericLiteral) -> ExprResult {
        Ok(Value::Number(lit.value))
    }
    fn visit_string(&mut self, lit: &StringLiteral) -> ExprResult {
        Ok(Value::String(lit.value.to_string()))
    }
    fn visit_boolean(&mut self, lit: bool) -> ExprResult {
        Ok(Value::Boolean(lit))
    }
    fn visit_null(&mut self) -> ExprResult {
        Ok(Value::Null)
    }
}
