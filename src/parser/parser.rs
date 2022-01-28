use crate::error::ParserError;
use crate::position::Pos;
use crate::token::TokenKind;
use crate::{ast::*, ParseResult};
use crate::{lexer::Lexer, token::Token};

pub struct Parser<'a> {
    pub lexer: Lexer<'a>,
    pub ast: Ast,
    pub current_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(lexer: Lexer<'a>) -> Self {
        Self {
            lexer,
            // lexer: Lexer::new(source.as_bytes(), filename),
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
        self.lexer.next()
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
    pub(super) fn expect_stmt_terminator(&mut self) -> ParseResult<()> {
        match self.current_token.kind {
            TokenKind::BraceClose => (),
            TokenKind::Eof => self.consume(),
            TokenKind::Eol => self.consume(),
            TokenKind::Semi => {
                self.consume();
                self.maybe(TokenKind::Eol);
            }
            _ => {
                // println!("expect_end_with_semi err");
                self.log();
                return Err(ParserError::missing_semi(
                    self.lexer.filename,
                    self.current_token.loc.start,
                ));
            }
        }

        Ok(())
    }

    pub(super) fn is_stmt_end(&mut self) -> bool {
        if self.expect_one_of(&[TokenKind::Semi, TokenKind::Eol, TokenKind::BraceClose]) {
            return true;
        }
        return false;
    }

    pub(super) fn expect(&mut self, kind: TokenKind) -> ParseResult<()> {
        match &self.current_token.kind {
            ty if ty == &kind => Ok(()),
            _ => {
                return Err(ParserError::unexpected_token(
                    self.lexer.filename,
                    &self.current_token.kind,
                    &kind,
                    self.current_token.loc.start,
                ));
            }
        }
    }

    pub(super) fn eat(&mut self, kind: TokenKind) -> ParseResult<()> {
        match self.expect(kind) {
            Ok(_) => Ok(self.consume()),
            Err(e) => return Err(e),
        }
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
        return false;
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
        self.eat(TokenKind::Eof)?;
        let program = Program::new(node);
        Ok(program)
    }
}
