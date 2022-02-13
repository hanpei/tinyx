use crate::ast::*;
use crate::error::ParserError;
use crate::position::Pos;
use crate::token::TokenKind;
use crate::{lexer::Lexer, token::Token};

use super::ParseResult;

pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub current_token: Token,
    pub could_be_semi: bool, // auto semi insertion, 可以为";""
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer,
            current_token: Token::new(
                TokenKind::None,
                "init".to_string(),
                Pos::new(1, 1),
                Pos::new(1, 1),
            ),
            could_be_semi: false,
        }
    }

    fn next_token(&mut self) -> ParseResult<Token> {
        self.lexer.next()
    }

    pub(super) fn consume(&mut self) {
        self.current_token = self.next_token().expect("consume error");
        self.asi();
    }

    /**
     * asi: automatic-semicolon-insertion
     * https://262.ecma-international.org/7.0/#sec-automatic-semicolon-insertion
     *  1. lexer:多个eol只生成一个token::eol
     *  2. parser时:
     *      * 遇到 eol 继续consume，标记could_be_semi=true
     *      * 遇到 "}" 或 "eof"，不cosume, 标记could_be_semi=true
     *  3. expect_stmt_terminator 方法，判断当前是token::semi或者could_be_semi=true
     *  4. 单独处理了return stmt特例情况，目前没有 break 和 continue，也没有 --  ++ 操作
     */

    fn asi(&mut self) {
        if self.token_is(TokenKind::Eol) {
            self.consume();
            self.could_be_semi = true;
        } else if self.expect_one_of(&[TokenKind::BraceClose, TokenKind::Eof]) {
            self.could_be_semi = true;
        } else {
            self.could_be_semi = false;
        }
    }

    // pub(super) fn maybe(&mut self, kind: TokenKind) {
    //     match &self.current_token.kind {
    //         maybe if maybe == &kind => self.consume(),
    //         _ => (),
    //     }
    // }

    // 调试用
    pub(super) fn log(&mut self) {
        println!("log token {}", self.current_token);
    }

    // statement结尾的情况';'  '\n'  ';\n'
    // 或者是在block中 最后一个是'}'
    pub(super) fn expect_stmt_terminator(&mut self) -> ParseResult<()> {
        match self.current_token.kind {
            // TokenKind::BraceClose => (),
            // TokenKind::Eof => (),
            TokenKind::Semi => {
                self.consume();
            }
            _ if self.could_be_semi => (),
            _ => {
                return Err(ParserError::missing_semi(
                    self.lexer.filename,
                    self.current_token.loc.start,
                ));
            }
        }

        Ok(())
    }

    pub(super) fn is_stmt_end(&mut self) -> bool {
        if self.expect_one_of(&[
            TokenKind::Semi,
            TokenKind::Eol,
            TokenKind::Eof,
            TokenKind::BraceClose,
        ]) {
            return true;
        }
        false
    }

    pub(super) fn expect(&mut self, kind: TokenKind) -> ParseResult<()> {
        match &self.current_token.kind {
            ty if ty == &kind => Ok(()),
            _ => Err(ParserError::unexpected_token(
                self.lexer.filename,
                &self.current_token,
                &kind,
                self.current_token.loc.start,
            )),
        }
    }

    pub(super) fn eat(&mut self, kind: TokenKind) -> ParseResult<()> {
        match self.expect(kind) {
            Ok(_) => self.consume(),
            Err(e) => return Err(e),
        }
        Ok(())
    }

    pub(super) fn token_is(&mut self, expect_kind: TokenKind) -> bool {
        let kind = &self.current_token.kind;
        kind == &expect_kind
    }

    /**
     * return true if one of tokenkind in the list
     */
    pub(super) fn expect_one_of(&mut self, kinds: &[TokenKind]) -> bool {
        for kind in kinds {
            if kind == &self.current_token.kind {
                return true;
            }
        }
        false
    }

    pub fn parse(&mut self) -> ParseResult<Ast> {
        let node = self.parse_program()?;
        Ok(node)
    }

    /**
     *  Program  
     *      : StatementList EOF
     *      ;
     */
    fn parse_program(&mut self) -> ParseResult<Program> {
        self.consume();
        let node = self.parse_statement_list()?;
        // self.eat(TokenKind::Eof)?;
        let program = Program::new(node, Some(self.lexer.filename.to_string()));
        Ok(program)
    }
}
