use crate::{
    ast::Expression,
    error::Error,
    token::{Token, TokenKind},
    ParseResult,
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /**
     *  NumericLiteral
     */
    pub(super) fn parse_number(&mut self) -> ParseResult<Expression> {
        self.expect(TokenKind::Number)?;
        match self.lookahead.raw.parse::<f64>() {
            Ok(n) => {
                let expr = Expression::NumericLiteral(n);
                self.consume();

                Ok(expr)
            }
            Err(_) => Err(Error::parse_number_error(
                self.tokenizer.filename,
                self.tokenizer.pos(),
            )),
        }
    }

    pub(super) fn parse_string(&mut self) -> ParseResult<Expression> {
        self.expect(TokenKind::String)?;
        let expr = Expression::StringLiteral(self.lookahead.raw.to_string());
        self.consume();
        Ok(expr)
    }
}
