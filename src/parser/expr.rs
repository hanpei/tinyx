use crate::{ast::Expression, token::TokenKind, ParseResult};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /**
     *  Expression
     *      : PrimaryExpression
     *      : ... todo
     *      ;
     */
    pub(super) fn parse_expression(&mut self) -> ParseResult<Expression> {
        self.parse_primary_expr()
    }

    /**
     * PrimaryExpression
     *   : Literal
     *   | ParenthesizedExpression
     *   | Identifier
     *   | ThisExpression
     *   | NewExpression
     *   ;
     */
    fn parse_primary_expr(&mut self) -> ParseResult<Expression> {
        match self.lookahead.kind {
            TokenKind::Number | TokenKind::String => self.parse_literal(),
            TokenKind::Identifier => self.parse_identifier(),
            _ => unimplemented!(),
        }
    }

    /**
     * Literal
     *   : NumericLiteral
     *   | StringLiteral
     *   | BooleanLiteral
     *   | NullLiteral
     *   ;
     */
    fn parse_literal(&mut self) -> ParseResult<Expression> {
        match self.lookahead.kind {
            TokenKind::Number => self.parse_number(),
            TokenKind::Identifier => todo!(),
            TokenKind::String => self.parse_string(),
            _ => unreachable!(),
        }
    }

    /**
     * Identifier
     *      : IDENTIFIER
     *      ;
     */
    fn parse_identifier(&mut self) -> ParseResult<Expression> {
        self.expect(TokenKind::Identifier)?;
        let ident = self.lookahead.raw.to_string();
        let expr = Expression::IdentifierExpr(ident);
        self.consume();
        Ok(expr)
    }
}
