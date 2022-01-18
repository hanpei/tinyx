use std::fmt::Display;

use super::{expr::*, stmt::*, Program};

impl Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stmt in &self.body {
            // writeln!(f, "PROGRAM")?;
            writeln!(f, "{},", stmt)?;
        }
        Ok(())
    }
}

impl Display for Statement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Statement::ExprStmt(e) => write!(f, "{}", e),
            Statement::Block(b) => {
                write!(f, "Block: ")?;
                write!(f, "[ ")?;
                for (i, stmt) in b.iter().enumerate() {
                    if i == b.len() - 1 {
                        write!(f, "{}", stmt)?;
                    } else {
                        write!(f, "{}, ", stmt)?;
                    }
                }
                write!(f, " ]")?;
                Ok(())
            }
            Statement::Empty => write!(f, ""),
            Statement::VariableDeclaration(v) => write!(f, "{}", v),
            Statement::FunctionDeclaration(func) => write!(f, "{}", func),
            Statement::If(i) => write!(f, "{}", i),
            Statement::Return(r) => write!(f, "{}", r),
            Statement::PrintStmt(e) => {
                write!(f, "Print: ")?;
                write!(f, "{{ ")?;
                write!(f, "{}", e)?;
                write!(f, " }}")
            }
            Statement::While(s) => write!(f, "{}", s),
        }
    }
}

impl Display for IfStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "If: ")?;
        write!(f, "{{ ")?;
        write!(f, "test: {}, ", self.test)?;
        write!(f, "consequent: {}", self.consequent)?;
        if let Some(expr) = &self.alternate {
            write!(f, ", alternate: {}", expr)?;
        }
        write!(f, " }}")
    }
}

impl Display for VariableDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "VarDecl: ")?;
        write!(f, "{{ ")?;
        write!(f, "ident: {}, ", self.id)?;
        if let Some(e) = &self.init {
            write!(f, "init: {}", e)?;
        }
        write!(f, " }}")
    }
}

impl Display for FunctionDeclaration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Function: ")?;
        write!(f, "{{ ")?;
        write!(f, "ident: {}, ", self.id)?;
        if let Some(idents) = &self.params {
            write!(f, "params: [ ")?;
            for (i, ident) in idents.iter().enumerate() {
                if i == idents.len() - 1 {
                    write!(f, "{}", ident)?;
                } else {
                    write!(f, "{}, ", ident)?;
                }
            }
            write!(f, " ], ")?;
        }
        write!(f, "body: {}", self.body)?;
        write!(f, " }}")
    }
}

impl Display for ReturnStatement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(expr) = &self.argument {
            write!(f, "Return: {}", expr)
        } else {
            Ok(())
        }
    }
}
impl Display for WhileStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "While: ")?;
        write!(f, "{{ ")?;
        write!(f, "test: {}, ", self.test)?;
        write!(f, "body: {}", self.body)?;
        write!(f, " }}")
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::NumericLiteral(n) => write!(f, "{}", n),
            Expr::StringLiteral(s) => write!(f, "{}", s),
            Expr::BooleanLiteral(b) => write!(f, "{}", b),
            Expr::Binary(b) => write!(f, "{}", b),
            Expr::Unary(u) => write!(f, "{}", u),
            Expr::Identifier(i) => write!(f, "{}", i),
            Expr::Assign(a) => write!(f, "{}", a),
            Expr::Call(c) => write!(f, "{}", c),
            Expr::NullLiteral => write!(f, "null"),
            Expr::Logical(l) => write!(f, "{}", l),
        }
    }
}

impl Display for BinaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Binary: ")?;
        write!(f, "{{ ")?;
        write!(f, "{} {} {}", self.left, self.op.value, self.right)?;
        write!(f, " }}")
    }
}

impl Display for UnaryExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unary: ")?;
        write!(f, "{{ ")?;
        write!(f, "{}{}", self.op.value, self.argument)?;
        write!(f, " }}")
    }
}

impl Display for AssignExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Assign: ")?;
        write!(f, "{{ ")?;
        write!(
            f,
            "left: {}, op: {}, right: {}",
            self.left, self.op.value, self.right
        )?;
        write!(f, " }}")
    }
}

impl Display for LogicalExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Logical: ")?;
        write!(f, "{{ ")?;
        write!(f, "{} {} {}", self.left, self.op.value, self.right)?;
        write!(f, " }}")
    }
}

impl Display for CallExpr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Call: ")?;
        write!(f, "{{ ")?;
        write!(f, "callee: {}, ", self.callee)?;

        if let Some(args) = &self.arguments {
            write!(f, "args: [ ")?;
            for (i, arg) in args.iter().enumerate() {
                if i == args.len() - 1 {
                    write!(f, "{}", arg)?;
                } else {
                    write!(f, "{}, ", arg)?;
                }
            }
            write!(f, " ]")?;
        } else {
            write!(f, "[]")?;
        }
        write!(f, " }}")
    }
}

impl Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{{ ")?;
        write!(f, "{}", self.name)
        // write!(f, " }}")
    }
}

impl Display for StringLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{{ ")?;
        write!(f, "\"{}\"", self.value)
        // write!(f, " }}")
    }
}

impl Display for NumericLiteral {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
