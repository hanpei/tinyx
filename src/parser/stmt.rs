use crate::{ast::Statement, token::TokenKind, ParseResult, error::Error};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /**
     * StatementList
     *      : Statement
     *      | StatementList Statement -> Statement Statement Statement...
     *      ;
     */
    pub(super) fn parse_statement_list(&mut self) -> ParseResult<Vec<Statement>> {
        let mut list = Vec::new();
        loop {
            match self.lookahead.kind {
                TokenKind::Eof => break,
                TokenKind::Eol => self.consume(),
                TokenKind::BraceClose => self.consume(),
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
     *      : EmptyStatement
     *      ;
     *      ...
     */
    fn parse_statment(&mut self) -> ParseResult<Statement> {
        // println!("{}", self.lookahead);
        match self.lookahead.kind {
            TokenKind::BraceOpen => self.parse_block_stmt(),
            TokenKind::Semi => self.parse_empty_stmt(),
            TokenKind::Number => self.parse_expression_stmt(),
            TokenKind::String => self.parse_expression_stmt(),
            TokenKind::Identifier => self.parse_expression_stmt(),
            TokenKind::Operator => self.parse_expression_stmt(),
            // _ => Err(Error::invalid_token(
            //     self.tokenizer.filename,
            //     self.tokenizer.loc.start,
            // )),
            _=> unimplemented!()
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
        // self.expect_semi_or_eol()?;
        Ok(Statement::ExpressionStatement(expr))
    }

    /**
     *  BlockStatement
     *      : "{" ExpressionStatement "}"
     *      ;
     */
    fn parse_block_stmt(&mut self) -> ParseResult<Statement> {
        self.expect(TokenKind::BraceOpen)?;
        self.consume();
        let stmt = self.parse_statement_list()?;
        self.expect(TokenKind::BraceClose)?;
        self.consume();

        Ok(Statement::BlockStatement(stmt))
    }

    /**
     *  EmptyStatement
     *      : ";"
     *      ;
     */

    fn parse_empty_stmt(&mut self) -> ParseResult<Statement> {
        self.expect(TokenKind::Semi)?;
        self.consume();
        Ok(Statement::EmptyStatement)
    }
}
