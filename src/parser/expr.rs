use std::io::BufRead;

use crate::{
    ast::{BinaryExpr, Expression, Operator},
    token::TokenKind,
    ParseResult, error::Error,
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /**
     *  Expression
     *      : AdditiveExpression
     *      : ... todo
     *      ;
     */
    pub(super) fn parse_expression(&mut self) -> ParseResult<Expression> {
        // self.parse_primary_expr()
        self.parse_additive_expr()
    }

    /**
     *  AdditiveExpression
     *      : Literal
     *      : AdditiveExpression OP Literal -> Literal OP Literal Op Literal ...
     *      ;
     */
    pub(super) fn parse_additive_expr(&mut self) -> ParseResult<Expression> {
        let mut left = self.parse_literal()?;
        while self.lookahead.kind == TokenKind::Operator   {
            let op = Operator::from(&self.lookahead.raw);
            self.consume();
            let right = self.parse_literal()?;
            let binary = BinaryExpr::new(left, op, right);
            left =  Expression::BinaryExpr(binary);
        };
        Ok(left)
    }

    /**
     * PrimaryExpression
     *   : Literal
     *   | Identifier
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
            _ => unreachable!("{:?}", self.lookahead.kind),
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
