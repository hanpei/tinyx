use crate::{
    ast::{BinaryExpr, Expression, Identifier},
    token::{Operator, TokenKind},
    ParseResult,
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
        self.parse_additive_expr()
    }

    /**
     *  AdditiveExpression
     *      : MultiplicativeExpression
     *      : AdditiveExpression OP Literal -> Literal OP Literal Op Literal ...
     *      ;
     */
    pub(super) fn parse_additive_expr(&mut self) -> ParseResult<Expression> {
        let mut left = self.parse_mul_expr()?;
        while self.current_token.kind == TokenKind::Operator(Operator::Add)
            || self.current_token.kind == TokenKind::Operator(Operator::Min)
        {
            let op = Operator::from_str(&self.current_token.raw);
            self.consume();
            let right = self.parse_mul_expr()?;
            left = Expression::BinaryExpr(BinaryExpr::new(left, op, right));
        }
        Ok(left)
    }
    /**
     *  MultiplicativeExpression
     *      : PrimaryExpression
     *      : MultiplicativeExpression OP PrimaryExpression -> PrimaryExpression OP PrimaryExpression Op PrimaryExpression ...
     *      ;
     */
    pub(super) fn parse_mul_expr(&mut self) -> ParseResult<Expression> {
        let mut left = self.parse_primary_expr()?;
        while self.current_token.kind == TokenKind::Operator(Operator::Mul)
            || self.current_token.kind == TokenKind::Operator(Operator::Div)
        {
            let op = Operator::from_str(&self.current_token.raw);
            self.consume();
            let right = self.parse_primary_expr()?;
            left = Expression::BinaryExpr(BinaryExpr::new(left, op, right));
        }
        Ok(left)
    }

    /**
     * PrimaryExpression
     *      : Literal
     *      ：ParenthesizedExpression
     *      | Identifier
     *      ;
     */
    fn parse_primary_expr(&mut self) -> ParseResult<Expression> {
        match self.current_token.kind {
            TokenKind::Number(_) | TokenKind::String => self.parse_literal(),
            TokenKind::Identifier => self.parse_identifier_expr(),
            TokenKind::BracketOpen => self.parse_parenthesized_expr(),
            _ => unimplemented!(),
        }
    }

    /**
     * ParenthesizedExpression
     *   : '(' Expression ')'
     *   ;
     */
    fn parse_parenthesized_expr(&mut self) -> ParseResult<Expression> {
        self.expect(TokenKind::BracketOpen)?;
        self.consume();
        let expr = self.parse_expression()?;
        self.expect(TokenKind::BracketClose)?;
        self.consume();
        Ok(expr)
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
        match self.current_token.kind {
            TokenKind::Number(n) => self.parse_number(n),
            TokenKind::Identifier => todo!(),
            TokenKind::String => self.parse_string(),
            _ => unreachable!("{:?}", self.current_token.kind),
        }
    }

    /**
     * IdentifierExpression
     *      : Identifier
     *      ;
     */
    pub fn parse_identifier_expr(&mut self) -> ParseResult<Expression> {
        let ident = self.parse_identifier()?;
        Ok(Expression::Identifier(ident))
    }

    /**
     *  Identifier
     *      :IDENTIFIER
     */
    pub fn parse_identifier(&mut self) -> ParseResult<Identifier> {
        self.expect(TokenKind::Identifier)?;
        let name = self.current_token.raw.to_string();
        let expr = Identifier::new(name);
        self.consume();
        Ok(expr)
    }
}
