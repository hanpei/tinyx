use crate::ast::*;

// stmt ===============================
pub trait StmtVisitor {
    type Item;
    fn walk_stmt(&mut self, stmt: &Statement) -> Self::Item {
        match stmt {
            Statement::ExprStmt(expr) => self.visit_expr_stmt(expr),
            Statement::Block(block) => self.visit_block(block),
            Statement::Empty => self.visit_empty(),
            Statement::VariableDeclaration(delc) => self.visit_variable_declare(delc),
            Statement::FunctionDeclaration(delc) => self.visit_function_declare(delc),
            Statement::If(i) => self.visit_if_stmt(i),
            Statement::Return(r) => self.visit_return_stmt(r),
            Statement::PrintStmt(expr) => self.visit_print_stmt(expr),
            Statement::While(w) => self.visit_while_stmt(w),
            Statement::ClassDeclaration(class) => self.visit_class_declare(class),
        }
    }

    fn visit_expr_stmt(&mut self, expr: &Expr) -> Self::Item;
    fn visit_block(&mut self, block: &[Statement]) -> Self::Item;
    fn visit_empty(&mut self) -> Self::Item;
    fn visit_variable_declare(&mut self, decl: &VariableDeclaration) -> Self::Item;
    fn visit_function_declare(&mut self, decl: &FunctionDeclaration) -> Self::Item;
    fn visit_class_declare(&mut self, class: &ClassDeclaration) -> Self::Item;
    fn visit_if_stmt(&mut self, stmt: &IfStatement) -> Self::Item;
    fn visit_return_stmt(&mut self, stmt: &ReturnStatement) -> Self::Item;
    fn visit_print_stmt(&mut self, expr: &Expr) -> Self::Item;
    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Self::Item;
}

// expr ===============================
pub trait ExprVisitor {
    type Item;

    fn walk_expr(&mut self, expr: &Expr) -> Self::Item {
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
            Expr::Logical(l) => self.visit_logical(l),
            Expr::Get(m) => self.visit_get(m),
            Expr::Set(s) => self.visit_set(s),
            Expr::This(t) => self.visit_this(t),
            Expr::Super(s) => self.visit_super(s),
        }
    }

    fn visit_binary(&mut self, binary: &BinaryExpr) -> Self::Item;
    fn visit_unary(&mut self, unary: &UnaryExpr) -> Self::Item;
    fn visit_assign(&mut self, assign: &AssignExpr) -> Self::Item;
    fn visit_ident(&mut self, ident: &Identifier) -> Self::Item;
    fn visit_call(&mut self, call: &CallExpr) -> Self::Item;
    fn visit_logical(&mut self, expr: &LogicalExpr) -> Self::Item;
    fn visit_get(&mut self, expr: &GetExpr) -> Self::Item;
    fn visit_set(&mut self, expr: &SetExpr) -> Self::Item;
    fn visit_this(&mut self, this: &ThisExpr) -> Self::Item;
    fn visit_super(&mut self, expr: &SuperExpr) -> Self::Item;

    // literal ===============================
    fn visit_numeric(&mut self, lit: &NumericLiteral) -> Self::Item;
    fn visit_string(&mut self, lit: &StringLiteral) -> Self::Item;
    fn visit_boolean(&mut self, lit: bool) -> Self::Item;
    fn visit_null(&mut self) -> Self::Item;
}
