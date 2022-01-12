use crate::{
    ast::{Expr, StringLiteral},
    error::ParserError,
    token::TokenKind,
    ParseResult,
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /**
     *  Literal
     *      : NUMBER
     *      | STRING
     *      | BOOLEAN
     */
    pub(super) fn parse_number(&mut self) -> ParseResult<Expr> {
        self.expect(TokenKind::Number)?;

        match self.current_token.raw.parse::<f64>() {
            Ok(n) => {
                self.consume();
                Ok(Expr::NumericLiteral(n))
            }
            Err(_e) => Err(ParserError::parse_number_error(
                self.lexer.filename,
                self.current_token.loc.start,
            )),
        }
    }

    pub(super) fn parse_string(&mut self) -> ParseResult<Expr> {
        self.expect(TokenKind::String)?;
        let expr = Expr::StringLiteral(StringLiteral::new(
            self.current_token.raw.to_string(),
            self.current_token.loc,
        ));
        self.consume();
        Ok(expr)
    }

    pub(super) fn parse_boolean(&mut self) -> ParseResult<Expr> {
        self.expect(TokenKind::Boolean)?;
        let b = if self.current_token.raw == "true" {
            true
        } else if self.current_token.raw == "false" {
            false
        } else {
            println!("parse_boolean error");
            self.log();
            return Err(ParserError::invalid_token(
                self.lexer.filename,
                self.current_token.loc.start,
            ));
        };
        let expr = Expr::BooleanLiteral(b);
        self.consume();
        Ok(expr)
    }
}
