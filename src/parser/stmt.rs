use crate::{
    ast::{FunctionDeclaration, IfStatement, ReturnStatement, Statement, VariableDeclaration},
    error::Error,
    token::{Keyword, Operator, TokenKind},
    ParseResult,
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /**
     * StatementList
     *      : Statement*
     *      ;
     */
    pub(super) fn parse_statement_list(&mut self) -> ParseResult<Vec<Statement>> {
        let mut list = Vec::new();

        while !self.token_is(TokenKind::Eof) && !self.token_is(TokenKind::BraceClose) {
            match self.current_token.kind {
                TokenKind::Eol => self.consume(),
                _ => list.push(self.parse_statment()?),
            }
        }
        Ok(list)
    }

    /**
     *  Statement
     *      : ExpressionStatement
     *      | BlockStatement
     *      | EmptyStatement
     *      | IfStatement
     *      | ReturnStatement
     *      | VariableDeclarator
     *      | FunctionDeclaration
     *      ;
     *      ...
     */
    fn parse_statment(&mut self) -> ParseResult<Statement> {
        match self.current_token.kind {
            TokenKind::BraceOpen => self.parse_block_stmt(),
            TokenKind::Semi => self.parse_empty_stmt(),
            TokenKind::ParenOpen => self.parse_expression_stmt(),
            TokenKind::Number => self.parse_expression_stmt(),
            TokenKind::String => self.parse_expression_stmt(),
            TokenKind::Operator(_) => self.parse_expression_stmt(),
            TokenKind::Identifier => self.parse_expression_stmt(),
            TokenKind::Keyword(Keyword::Let) => self.parse_variable_stmt(),
            TokenKind::Keyword(Keyword::If) => self.parse_if_stmt(),
            TokenKind::Keyword(Keyword::Fn) => self.parse_fn_declaration_stmt(),
            TokenKind::Keyword(Keyword::Return) => self.parse_return_stmt(),
            _ => {
                println!("parse_statment error token: ");
                self.log();
                return Err(Error::invalid_token(
                    self.tokenizer.filename,
                    self.current_token.loc.start,
                ));
            }
        }
    }

    /**
     *  ExpressionStatement
     *      : Expression STMT_END
     *      ;
     *
     *  STMT_END:
     *      : ";"
     *      | "\n"
     *      | ";\n"
     *      | "}"
     *      ;
     */
    fn parse_expression_stmt(&mut self) -> ParseResult<Statement> {
        let expr = self.parse_expression()?;
        self.expect_stmt_terminator()?;

        Ok(Statement::ExprStmt(expr))
    }

    /**
     *  BlockStatement
     *      : "{" StatementList "}"
     *      ;
     */
    fn parse_block_stmt(&mut self) -> ParseResult<Statement> {
        self.eat(TokenKind::BraceOpen)?;
        self.maybe(TokenKind::Eol);
        let mut list: Vec<Statement> = Vec::new();
        if self.current_token.kind != TokenKind::BraceClose {
            list = self.parse_statement_list()?;
        }
        self.maybe(TokenKind::Eol);
        self.eat(TokenKind::BraceClose)?;

        Ok(Statement::Block(list))
    }

    /**
     *  EmptyStatement
     *      : ";"
     *      | ";\n"
     *      ;
     */

    fn parse_empty_stmt(&mut self) -> ParseResult<Statement> {
        self.eat(TokenKind::Semi)?;
        Ok(Statement::Empty)
    }

    /**
     *  VariableDeclarator
     *      : "let" VariableDeclaration (";")?
     *      ;
     */
    fn parse_variable_stmt(&mut self) -> ParseResult<Statement> {
        self.eat(TokenKind::Keyword(Keyword::Let))?;
        let stmt = self.parse_variable_declaration()?;
        self.expect_stmt_terminator()?;
        Ok(stmt)
    }

    /**
     * VariableDeclaration
     *      : Identifier ( "=" AssignmentExpression)?
     */
    fn parse_variable_declaration(&mut self) -> ParseResult<Statement> {
        let left = self.parse_identifier()?;
        if self.token_is(TokenKind::Operator(Operator::Assign)) {
            self.eat(TokenKind::Operator(Operator::Assign))?;
            let right = self.parse_expression()?;
            Ok(Statement::VariableDeclaration(VariableDeclaration::new(
                left,
                Some(right),
            )))
        } else {
            Ok(Statement::VariableDeclaration(VariableDeclaration::new(
                left, None,
            )))
        }
    }

    /**
     * IfStatement
     *      : "if" "(" Expression ")" Statement ( "else" Statement )?
     *
     */
    fn parse_if_stmt(&mut self) -> ParseResult<Statement> {
        self.eat(TokenKind::Keyword(Keyword::If))?;
        self.eat(TokenKind::ParenOpen)?;
        let test = self.parse_expression()?;
        self.eat(TokenKind::ParenClose)?;

        let consequent = self.parse_statment()?;

        let alternate = if let Ok(()) = self.expect_stmt_terminator() {
            None
        } else {
            self.eat(TokenKind::Keyword(Keyword::Else))?;
            Some(self.parse_statment()?)
        };

        let if_stmt = IfStatement::new(test, consequent, alternate);
        Ok(Statement::If(if_stmt))
    }

    /**
     *  FunctionDeclaration
     *      : "function" Identifier ( "(" ( FormalParameterList )? ")" ) FunctionBody
     *
     *  FormalParameterList
     *      : Identifier ( "," Identifier )*
     *
     *  FunctionBody
     *      : "{" ( Statement )? "}"
     */
    fn parse_fn_declaration_stmt(&mut self) -> ParseResult<Statement> {
        self.eat(TokenKind::Keyword(Keyword::Fn))?;
        let id = self.parse_identifier()?;
        self.eat(TokenKind::ParenOpen)?;

        let mut ids = Vec::new();
        let params = if self.current_token.kind != TokenKind::ParenClose {
            loop {
                let id = self.parse_identifier()?;
                ids.push(id);
                if self.token_is(TokenKind::Comma) {
                    self.consume();
                } else {
                    break;
                }
            }
            Some(ids)
        } else {
            None
        };
        self.eat(TokenKind::ParenClose)?;
        let body = self.parse_block_stmt()?;
        let stmt = FunctionDeclaration::new(id, params, body);
        Ok(Statement::FunctionDeclaration(stmt))
    }

    /**
     *  ReturnStatement
     *      : "return" Expression? STMT_END
     */
    fn parse_return_stmt(&mut self) -> ParseResult<Statement> {
        self.eat(TokenKind::Keyword(Keyword::Return))?;

        let argument = if !self.is_stmt_end() {
            Some(self.parse_expression()?)
        } else {
            None
        };
        let stmt = ReturnStatement::new(argument);
        self.expect_stmt_terminator()?;
        Ok(Statement::Return(stmt))
    }
}
