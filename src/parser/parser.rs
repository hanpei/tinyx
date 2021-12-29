use crate::error::Error;
use crate::lexer::Pos;
use crate::token::TokenKind;
use crate::{ast::*, ParseResult};
use crate::{lexer::Lexer, token::Token};

pub struct Parser<'a> {
    pub tokenizer: Lexer<'a>,
    pub ast: Ast,
    pub lookahead: Token,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str, filename: &'a str) -> Self {
        Self {
            tokenizer: Lexer::new(source.as_bytes(), filename),
            ast: Program::new(),
            lookahead: Token::new(
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
        self.lookahead = self.next_token().expect("consume error");
    }

    pub(super) fn maybe(&mut self, kind: TokenKind) {
        match &self.lookahead.kind {
            maybe if maybe == &kind => self.consume(),
            _ => {}
        }
    }

    pub(super) fn expect_semi_or_eol(&mut self) -> ParseResult<()> {
        match self.lookahead.kind {
            TokenKind::Semi => self.consume(),
            _ => match self.lookahead.kind {
                TokenKind::Eol => self.consume(),
                TokenKind::Eof => self.consume(),
                _ => {
                    return Err(Error::missing_semi(
                        self.tokenizer.filename,
                        self.lookahead.loc.end,
                    ))
                }
            },
        }
        Ok(())
    }

    pub(super) fn expect(&mut self, kind: TokenKind) -> ParseResult<()> {
        match &self.lookahead.kind {
            TokenKind::Eof => Ok(()),
            ty if ty == &kind => Ok(()),
            _ => {
                return Err(Error::unexpected_token(
                    self.tokenizer.filename,
                    &self.lookahead.kind,
                    &kind,
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
        // self.lookahead = self.next_token()?;
        self.consume();
        let stmt_list = self.parse_statement_list()?;
        self.ast.content(stmt_list);
        Ok(())
    }
}
