use std::vec;

use crate::error::Error;
use crate::token::TokenKind;
use crate::{ast::*, ParseResult};
use crate::{lexer::Lexer, token::Token};

pub struct Parser<'a> {
    tokenizer: Lexer<'a>,
    ast: Ast,
    lookahead: Token,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, filename: &'a str) -> Self {
        Self {
            tokenizer: Lexer::new(source.as_bytes(), filename),
            ast: Program::new(),
            lookahead: Token::None,
        }
    }

    fn next_token(&mut self) -> ParseResult<Token> {
        self.tokenizer.next()
    }

    fn consume(&mut self) {
        self.lookahead = self.next_token().expect("consume error");
    }

    fn maybe(&mut self, kind: TokenKind) {
        match self.lookahead.kind() {
            maybe if maybe == kind => self.consume(),
            _ => {}
        }
    }

    fn expect(&mut self, kind: TokenKind) -> ParseResult<()> {
        match self.lookahead.kind() {
            TokenKind::Eof => Ok(()),
            ty if ty == kind => Ok(()),
            _ => {
                return Err(Error::unexpected_token(
                    self.tokenizer.filename,
                    self.lookahead.kind(),
                    kind,
                    self.tokenizer.pos(),
                ))
            }
        }
    }

    pub fn parse(&mut self) -> ParseResult<&Ast> {
        self.parse_program()?;
        Ok(&self.ast)
    }

    /**
     *  Program    Vec<Statement>
     *      : StatementList
     *      ;
     */
    fn parse_program(&mut self) -> ParseResult<()> {
        self.lookahead = self.next_token()?;
        let stmt_list = self.parse_statement_list()?;
        self.ast.content(stmt_list);
        Ok(())
    }

    /**
     * StatementList
     *      : Statement
     *      | StatementList Statement -> Statement Statement Statement...
     *      ;
     */
    fn parse_statement_list(&mut self) -> ParseResult<Vec<Statement>> {
        // self.lookahead = self.next_token()?;
        let mut list = Vec::new();
        // let stmt = self.parse_statment()?;
        // list.push(stmt);
        loop {
            match self.lookahead {
                Token::Eof => break,
                Token::Eol => self.consume(),
                Token::BraceClose => self.consume(),
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
     *      ;
     *      ...
     */
    fn parse_statment(&mut self) -> ParseResult<Statement> {
        println!("{:?}", self.lookahead );
        match self.lookahead {
            Token::Number(_) => self.parse_expression_stmt(),
            Token::BraceOpen => self.parse_block_stmt(),
            _ => Err(Error::invalid_token(
                self.tokenizer.filename,
                self.tokenizer.pos(),
            )),
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
        self.maybe(TokenKind::Semi);
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
     *  Expression
     *      : AdditiveExpression
     *      ;
     */
    fn parse_expression(&mut self) -> ParseResult<Expression> {
        self.parse_number()
    }


    /**
     *  NumericLiteral
     */
    fn parse_number(&mut self) -> ParseResult<Expression> {
        self.expect(TokenKind::Number)?;
        Ok(match self.lookahead {
            Token::Number(n) => {
                self.consume();
                Expression::NumericLiteral(n)
            }
            _ => unreachable!(),
        })
    }
}
