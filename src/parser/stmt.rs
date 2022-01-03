use crate::{
    ast::{LetStatement, Statement},
    error::Error,
    token::{Keyword, Operator, TokenKind},
    ParseResult,
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /**
     * StatementList
     *      : Statement
     *      | Statement (seporator) StatementList
     *      ;
     */
    pub(super) fn parse_statement_list(&mut self) -> ParseResult<Vec<Statement>> {
        let mut list = Vec::new();
        let stmt = self.parse_statment()?;
        list.push(stmt);

        while self.current_token.kind != TokenKind::Eof {
            self.expect_stmt_seperator()?;
            match self.current_token.kind {
                TokenKind::Eol => self.consume(),
                TokenKind::Eof => break,
                TokenKind::BraceClose => break,
                _ => {
                    let stmt = self.parse_statment()?;
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
            TokenKind::Keyword(Keyword::Let) => self.parse_let_stmt(),
            _ => {
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
        Ok(Statement::ExpressionStatement(expr))
    }

    /**
     *  BlockStatement
     *      : "{" ("eol") StatementList ("eol") "}"
     *      ;
     */
    fn parse_block_stmt(&mut self) -> ParseResult<Statement> {
        self.eat(TokenKind::BraceOpen)?;
        self.maybe(TokenKind::Eol);
        let stmt = self.parse_statement_list()?;
        self.eat(TokenKind::BraceClose)?;

        Ok(Statement::BlockStatement(stmt))
    }

    /**
     *  EmptyStatement
     *      : ";"
     *      ;
     */

    fn parse_empty_stmt(&mut self) -> ParseResult<Statement> {
        self.eat(TokenKind::Semi)?;
        Ok(Statement::EmptyStatement)
    }

    /**
     *  LetStatement
     *      : "let" Identifier
     *      : "let" Identifier "=" Expression
     *      ;
     */
    fn parse_let_stmt(&mut self) -> ParseResult<Statement> {
        self.eat(TokenKind::Keyword(Keyword::Let))?;
        let ident = self.parse_identifier()?;

        if self.next_token_is(TokenKind::Operator(Operator::Assign)) {
            self.consume();
            let init = self.parse_expression()?;
            Ok(Statement::LetStatement(LetStatement::new(
                ident,
                Some(init),
            )))
        } else {
            let expr = Statement::LetStatement(LetStatement::new(ident, None));
            Ok(expr)
        }
    }
}
