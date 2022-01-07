use crate::{
    ast::{AssignExpr, BinaryExpr, Expr, Identifier},
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
     *  AssignmentExpression
     *      : AdditiveExpression
     *      | LeftHandSideExpression ASSIGN AssignmentExpression
     */
    pub(super) fn parse_assign_expr(&mut self) -> ParseResult<Expr> {
        let left = self.parse_relational_expr()?;

        if TokenKind::Operator(Operator::Assign) != self.current_token.kind {
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
     * RelationalExpression
     *      : AdditiveExpression ( RelationalOperator AdditiveExpression )*?
     *
     * RelationalOperator
     *      : ( "<" | "<=" | ">" | ">=" | "==" | "!=" )
     */

    pub(super) fn parse_relational_expr(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_additive_expr()?;
        while self.token_is(TokenKind::Operator(Operator::LessThan))
            || self.token_is(TokenKind::Operator(Operator::LessThanEqual))
            || self.token_is(TokenKind::Operator(Operator::GreaterThan))
            || self.token_is(TokenKind::Operator(Operator::GreaterThanEqual))
            || self.token_is(TokenKind::Operator(Operator::Equal))
            || self.token_is(TokenKind::Operator(Operator::NotEqual))
        {
            let op = Operator::from_str(&self.current_token.raw);
            self.consume();
            let right = self.parse_additive_expr()?;
            left = Expr::Binary(BinaryExpr::new(left, op, right));
        }
        Ok(left)
    }

    /**
    *  AdditiveExpression
           :MultiplicativeExpression ((ADD|MIN) MultiplicativeExpression)*
    */
    pub(super) fn parse_additive_expr(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_mul_expr()?;
        while self.current_token.kind == TokenKind::Operator(Operator::Add)
            || self.current_token.kind == TokenKind::Operator(Operator::Min)
        {
            let op = Operator::from_str(&self.current_token.raw);
            self.consume();
            let right = self.parse_mul_expr()?;
            left = Expr::Binary(BinaryExpr::new(left, op, right));
        }
        Ok(left)
    }

    /**
     *  MultiplicativeExpression
     *      : PrimaryExpression ((MUL|DIV) PrimaryExpression)*
     */
    pub(super) fn parse_mul_expr(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_primary_expr()?;
        while self.current_token.kind == TokenKind::Operator(Operator::Mul)
            || self.current_token.kind == TokenKind::Operator(Operator::Div)
        {
            let op = Operator::from_str(&self.current_token.raw);
            self.consume();
            let right = self.parse_primary_expr()?;
            left = Expr::Binary(BinaryExpr::new(left, op, right));
        }
        Ok(left)
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
