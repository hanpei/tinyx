use crate::{ast::Expression, error::Error, token::TokenKind, ParseResult};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /**
     *  NumericLiteral
     */
    pub(super) fn parse_number(&mut self, n: f64) -> ParseResult<Expression> {
        let expr = Expression::NumericLiteral(n);
        self.consume();
        Ok(expr)
    }

    pub(super) fn parse_string(&mut self) -> ParseResult<Expression> {
        self.expect(TokenKind::String)?;
        let expr = Expression::StringLiteral(self.current_token.raw.to_string());
        self.consume();
        Ok(expr)
    }
}
