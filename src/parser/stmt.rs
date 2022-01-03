use crate::{
    ast::{Statement, VariableDeclaration},
    error::Error,
    token::{Keyword, Operator, TokenKind},
    ParseResult,
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /**
     * StatementList
     *      | (Statement Seperator)*
     *      ;
     */
    pub(super) fn parse_statement_list(&mut self) -> ParseResult<Vec<Statement>> {
        let mut list = Vec::new();

        while self.current_token.kind != TokenKind::Eof {
            match self.current_token.kind {
                TokenKind::Eol => self.consume(),
                // TokenKind::Eof => break,
                TokenKind::BraceClose => break,
                _ => {
                    let stmt = self.parse_statment()?;
                    self.expect_stmt_seperator()?;

                    list.push(stmt);
                }
            }
        }

        Ok(list)
    }

    /**
     *  Statement
     *      : ExpressionStatement
     *      : BlockStatement
     *      : LetStatement
     *      : EmptyStatement
     *      ;
     *      ...
     */
    fn parse_statment(&mut self) -> ParseResult<Statement> {
        match self.current_token.kind {
            TokenKind::BraceOpen => self.parse_block_stmt(),
            TokenKind::Semi => self.parse_empty_stmt(),
            TokenKind::BracketOpen => self.parse_expression_stmt(),
            TokenKind::Number(_) => self.parse_expression_stmt(),
            TokenKind::String => self.parse_expression_stmt(),
            TokenKind::Operator(_) => self.parse_expression_stmt(),
            TokenKind::Identifier => self.parse_expression_stmt(),
            TokenKind::Keyword(Keyword::Let) => self.parse_variable_stmt(),
            _ => {
                println!("error token: ");
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
     *      : Expresssion
     *      : Expression ";"
     *      ;
     */
    fn parse_expression_stmt(&mut self) -> ParseResult<Statement> {
        let expr = self.parse_expression()?;
        Ok(Statement::ExprStmt(expr))
    }

    /**
     *  BlockStatement
     *      : '{' ('eol') StatementList ('eol') '}'
     *      ;
     */
    fn parse_block_stmt(&mut self) -> ParseResult<Statement> {
        self.eat(TokenKind::BraceOpen)?;
        self.maybe(TokenKind::Eol);
        let stmt = self.parse_statement_list()?;
        self.maybe(TokenKind::Eol);
        self.eat(TokenKind::BraceClose)?;
        Ok(Statement::Block(stmt))
    }

    /**
     *  EmptyStatement
     *      : ";"
     *      ;
     */

    fn parse_empty_stmt(&mut self) -> ParseResult<Statement> {
        self.eat(TokenKind::Semi)?;
        Ok(Statement::Empty)
    }

    /**
     *  VariableStatement
     *      : "let" VariableDeclaration
     *      ;
     */
    fn parse_variable_stmt(&mut self) -> ParseResult<Statement> {
        self.eat(TokenKind::Keyword(Keyword::Let))?;
        self.parse_variable_declaration()
    }

    /**
     * VariableDeclaration
     *      : Identifier (AssignmentExpression)?
     */
    fn parse_variable_declaration(&mut self) -> ParseResult<Statement> {
        let left = self.parse_identifier()?;
        if self.next_token_is(TokenKind::Operator(Operator::Assign)) {
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
}
