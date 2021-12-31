use crate::error::Error;
use crate::lexer::Pos;
use crate::token::TokenKind;
use crate::{ast::*, ParseResult};
use crate::{lexer::Lexer, token::Token};

pub struct Parser<'a> {
    pub tokenizer: Lexer<'a>,
    pub ast: Ast,
    pub current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, filename: &'a str) -> Self {
        Self {
            tokenizer: Lexer::new(source.as_bytes(), filename),
            ast: Program::new(),
            current_token: Token::new(
                TokenKind::None,
                "init".to_string(),
                Pos::new(1, 1),
                Pos::new(1, 1),
            ),
        }
    }

    fn next_token(&mut self) -> ParseResult<Token> {
        self.tokenizer.next()
    }

    pub(super) fn consume(&mut self) {
        self.current_token = self.next_token().expect("consume error");
    }

    pub(super) fn maybe(&mut self, kind: TokenKind) {
        match &self.current_token.kind {
            maybe if maybe == &kind => self.consume(),
            _ => {}
        }
    }

    // 调试用
    pub(super) fn log(&mut self) {
        println!("token {}", self.current_token);
    }

    pub(super) fn expect_semi_or_eol(&mut self) -> ParseResult<()> {
        match self.current_token.kind {
            TokenKind::BraceClose => self.consume(),
            TokenKind::Semi => self.consume(),
            _ => match self.current_token.kind {
                TokenKind::Eol => self.consume(),
                TokenKind::Eof => self.consume(),
                _ => {
                    return Err(Error::missing_semi(
                        self.tokenizer.filename,
                        self.current_token.loc.end,
                    ))
                }
            },
        }
        Ok(())
    }

    pub(super) fn expect(&mut self, kind: TokenKind) -> ParseResult<()> {
        match &self.current_token.kind {
            ty if ty == &kind => Ok(()),
            _ => {
                return Err(Error::unexpected_token(
                    self.tokenizer.filename,
                    &self.current_token.kind,
                    &kind,
                    self.current_token.loc.start,
                ))
            }
        }
    }
    pub(super) fn next_token_is(&mut self, expect_kind: TokenKind) -> bool {
        let kind = &self.current_token.kind;
        kind == &expect_kind
    }

    pub fn parse(&mut self) -> ParseResult<&Ast> {
        self.parse_program()?;
        Ok(&self.ast)
    }

    /**
     *  Program  
     *      : StatementList -> = Vec<Statement>
     *      ;
     */
    fn parse_program(&mut self) -> ParseResult<()> {
        // self.lookahead = self.next_token()?;
        self.consume();
        let stmt_list = self.parse_statement_list()?;
        self.ast.content(stmt_list);
        Ok(())
    }
}
