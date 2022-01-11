use std::fmt::Binary;

use crate::{
    ast::{AssignExpr, BinaryExpr, Expr, Identifier, UnaryExpr},
    error::Error,
    token::{Operator, TokenKind},
    ParseResult,
};

use super::parser::Parser;

impl<'a> Parser<'a> {
    /**
     *  Expression
     *      : AssignmentExpression
     */
    pub(super) fn parse_expression(&mut self) -> ParseResult<Expr> {
        self.parse_assign_expr()
        // self.parse_additive_expr()
    }

    /**
     *  EqualityExpression
     *      : RelationalExpression ( ( "!=" | "==" ) RelationalExpression )*
     *      ;
     */
    pub(super) fn parse_equality_expr(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_relational_expr()?;
        while self.expect_one_of(&[
            TokenKind::Operator(Operator::Equal),
            TokenKind::Operator(Operator::Equal),
        ]) {
            let op = Operator::from_str(&self.current_token.raw);
            self.consume();
            let right = self.parse_relational_expr()?;
            left = Expr::Binary(BinaryExpr::new(left, op, right))
        }
        Ok(left)
    }

    /**
     * AssignmentExpression:
     *      : EqualityExpression
     *      | IDENTIFIER "=" AssignmentExpression
     *      ;
     */
    pub(super) fn parse_assign_expr(&mut self) -> ParseResult<Expr> {
        let left = self.parse_equality_expr()?;

        if !self.token_is(TokenKind::Operator(Operator::Assign)) {
            return Ok(left);
        }

        let op = Operator::from_str(&self.current_token.raw);
        let op_loc = self.current_token.loc;
        self.consume();

        match left {
            Expr::Identifier(ident) => Ok(Expr::Assign(AssignExpr::new(
                op,
                Expr::Identifier(ident),
                self.parse_assign_expr()?,
            ))),
            _ => {
                return Err(Error::invalid_assignment(
                    self.tokenizer.filename,
                    op_loc.start,
                ));
            }
        }
    }

    /**
     * LeftHandSideExpression
     *      | todo..
     */
    pub(super) fn parse_left_hand_side_expr(&mut self) -> ParseResult<Expr> {
        todo!()
    }

    /**
     * RelationalExpression
     *      : AdditiveExpression ( RELATIONAL_OPERATOR AdditiveExpression )*?
     *
     * RELATIONAL_OPERATOR
     *      : ( "<" | "<=" | ">" | ">=" | "==" | "!=" )
     */

    pub(super) fn parse_relational_expr(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_additive_expr()?;
        while self.expect_one_of(&[
            TokenKind::Operator(Operator::LessThan),
            TokenKind::Operator(Operator::LessThanEqual),
            TokenKind::Operator(Operator::GreaterThan),
            TokenKind::Operator(Operator::GreaterThanEqual),
            TokenKind::Operator(Operator::Equal),
            TokenKind::Operator(Operator::NotEqual),
        ]) {
            let op = Operator::from_str(&self.current_token.raw);
            self.consume();
            let right = self.parse_additive_expr()?;
            left = Expr::Binary(BinaryExpr::new(left, op, right));
        }
        Ok(left)
    }

    /**
     *  AdditiveExpression
     *      :MultiplicativeExpression ((ADD|MIN) MultiplicativeExpression)*
     *      ;
     */
    pub(super) fn parse_additive_expr(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_mul_expr()?;
        while self.expect_one_of(&[
            TokenKind::Operator(Operator::Add),
            TokenKind::Operator(Operator::Min),
        ]) {
            let op = Operator::from_str(&self.current_token.raw);
            self.consume();
            let right = self.parse_mul_expr()?;
            left = Expr::Binary(BinaryExpr::new(left, op, right));
        }
        Ok(left)
    }

    /**
     *  MultiplicativeExpression
     *      : UnaryExpression ((MUL|DIV) UnaryExpression)*
     */
    pub(super) fn parse_mul_expr(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_unary_expr()?;
        while self.expect_one_of(&[
            TokenKind::Operator(Operator::Mul),
            TokenKind::Operator(Operator::Div),
        ]) {
            let op = Operator::from_str(&self.current_token.raw);
            self.consume();
            let right = self.parse_unary_expr()?;
            left = Expr::Binary(BinaryExpr::new(left, op, right));
        }
        Ok(left)
    }

    /**
     *  UnaryExpression
     *      : (+ | - | !) UnaryExpression
     *      | PrimaryExpression
     *      ;
     */
    pub(super) fn parse_unary_expr(&mut self) -> ParseResult<Expr> {
        if self.expect_one_of(&[
            TokenKind::Operator(Operator::Min),
            TokenKind::Operator(Operator::Add),
            TokenKind::Operator(Operator::Not),
        ]) {
            // self.parse_unary_expr()?;
            let op = Operator::from_str(&self.current_token.raw);
            self.consume();
            let argument = self.parse_unary_expr()?;
            return Ok(Expr::Unary(UnaryExpr::new(op, argument)));
        }

        self.parse_primary_expr()
    }

    /**
     * PrimaryExpression
     *      : Literal
     *      | Identifier
     *      | ParenthesizedExpression
     *      ;
     */
    fn parse_primary_expr(&mut self) -> ParseResult<Expr> {
        match self.current_token.kind {
            TokenKind::Number | TokenKind::String | TokenKind::Boolean => self.parse_literal(),
            TokenKind::Identifier => self.parse_identifier_expr(),
            TokenKind::ParenOpen => self.parse_parenthesized_expr(),
            _ => {
                println!("parse_primary_expr error");
                self.log();
                return Err(Error::invalid_token(
                    self.tokenizer.filename,
                    self.current_token.loc.start,
                ));
            } // _ => unimplemented!(),
        }
    }

    /**
     * ParenthesizedExpression
     *   : '(' Expression ')'
     *   ;
     */
    fn parse_parenthesized_expr(&mut self) -> ParseResult<Expr> {
        self.eat(TokenKind::ParenOpen)?;
        let expr = self.parse_expression()?;
        self.eat(TokenKind::ParenClose)?;
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
    fn parse_literal(&mut self) -> ParseResult<Expr> {
        match self.current_token.kind {
            TokenKind::Number => self.parse_number(),
            TokenKind::String => self.parse_string(),
            TokenKind::Boolean => self.parse_boolean(),
            _ => unreachable!("{:?}", self.current_token.kind),
        }
    }

    /**
     * IdentifierExpression
     *      : Identifier
     *      ;
     */
    pub fn parse_identifier_expr(&mut self) -> ParseResult<Expr> {
        let ident = self.parse_identifier()?;
        Ok(Expr::Identifier(ident))
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
