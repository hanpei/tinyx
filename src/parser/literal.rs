use crate::{ast::Expr, token::TokenKind, ParseResult};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /**
     *  NumericLiteral
     */
    pub(super) fn parse_number(&mut self, n: f64) -> ParseResult<Expr> {
        let expr = Expr::NumericLiteral(n);
        self.consume();
        Ok(expr)
    }

    pub(super) fn parse_string(&mut self) -> ParseResult<Expr> {
        self.expect(TokenKind::String)?;
        let expr = Expr::StringLiteral(self.current_token.raw.to_string());
        self.consume();
        Ok(expr)
    }
}
