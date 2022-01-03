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
            ast: Program::new(Vec::new()),
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
            _ => (),
        }
    }

    // 调试用
    pub(super) fn log(&mut self) {
        println!("token {}", self.current_token);
    }

    // statement结尾的情况';'  '\n'  ';\n'
    // 或者是在block中 最后一个是'}'
    pub(super) fn expect_stmt_seperator(&mut self) -> ParseResult<()> {
        match self.current_token.kind {
            TokenKind::BraceClose => (),
            TokenKind::Eof => (),
            TokenKind::Semi => {
                self.consume();
                self.maybe(TokenKind::Eol);
            }
            _ => match self.current_token.kind {
                TokenKind::Eol => self.consume(),
                _ => {
                    return Err(Error::missing_semi(
                        self.tokenizer.filename,
                        self.current_token.loc.start,
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

    pub(super) fn eat(&mut self, kind: TokenKind) -> ParseResult<()> {
        match self.expect(kind) {
            Ok(_) => Ok(self.consume()),
            Err(e) => return Err(e),
        }
    }

    pub(super) fn next_token_is(&mut self, expect_kind: TokenKind) -> bool {
        let kind = &self.current_token.kind;
        kind == &expect_kind
    }

    pub fn parse(&mut self) -> ParseResult<Ast> {
        let node = self.parse_program()?;
        Ok(node)
    }

    /**
     *  Program  
     *      : StatementList
     *      ;
     */
    fn parse_program(&mut self) -> ParseResult<Program> {
        self.consume();
        let node = self.parse_statement_list()?;
        self.eat(TokenKind::Eof)?;
        let program = Program::new(node);
        Ok(program)
    }
}
