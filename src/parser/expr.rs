use crate::{
    ast::*,
    error::ParserError,
    position::{Span, WithSpan},
    token::{Operator, TokenKind},
    ParseResult,
};

use super::{parser::Parser, MAXIMUM_ARGS};

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
     * AssignmentExpression:
     *      : IDENTIFIER "=" AssignmentExpression
     *      | LogicORExpression
     *      ;
     */
    pub(super) fn parse_assign_expr(&mut self) -> ParseResult<Expr> {
        let left = self.parse_logic_or_expr()?;

        if !self.token_is(TokenKind::Operator(Operator::Assign)) {
            return Ok(left);
        }

        let span_op = self.parse_op();

        match left {
            Expr::Identifier(ident) => Ok(Expr::Assign(AssignExpr::new(
                span_op,
                ident,
                self.parse_assign_expr()?,
            ))),
            _ => {
                return Err(ParserError::invalid_assignment(
                    self.lexer.filename,
                    span_op.loc.start,
                ));
            }
        }
    }

    /**
     * LogicORExpression:
     *      : LogicANDExpress ( "or" LogicANDExpress )*
     *      ;
     */
    pub(super) fn parse_logic_or_expr(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_logic_and_expr()?;
        while self.token_is(TokenKind::Operator(Operator::Or)) {
            let span_op = self.parse_op();
            let right = self.parse_logic_and_expr()?;

            left = Expr::Logical(LogicalExpr::new(left, span_op, right))
        }
        Ok(left)
    }

    /**
     * LogicANDExpress:
     *      : EqualityExpression ( "and" EqualityExpression )*
     *      ;
     */
    pub(super) fn parse_logic_and_expr(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_equality_expr()?;
        while self.token_is(TokenKind::Operator(Operator::And)) {
            let span_op = self.parse_op();
            let right = self.parse_equality_expr()?;

            left = Expr::Logical(LogicalExpr::new(left, span_op, right))
        }
        Ok(left)
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
            let span_op = self.parse_op();
            let right = self.parse_relational_expr()?;

            left = Expr::Binary(BinaryExpr::new(left, span_op, right))
        }
        Ok(left)
    }

    /**
     * LeftHandSideExpression
     *      | todo..
     */
    pub(super) fn _parse_left_hand_side_expr(&mut self) -> ParseResult<Expr> {
        todo!()
    }

    /**
     * RelationalExpression
     *      : AdditiveExpression ( RELATIONAL_OPERATOR AdditiveExpression )*?
     *      ;
     *
     * RELATIONAL_OPERATOR
     *      : ( "<" | "<=" | ">" | ">=" | "==" | "!=" )
     *      ;
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
            let span_op = self.parse_op();
            let right = self.parse_additive_expr()?;

            left = Expr::Binary(BinaryExpr::new(left, span_op, right));
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
            let span_op = self.parse_op();
            let right = self.parse_mul_expr()?;

            left = Expr::Binary(BinaryExpr::new(left, span_op, right));
        }
        Ok(left)
    }

    /**
     *  MultiplicativeExpression
     *      : UnaryExpression ((MUL|DIV) UnaryExpression)*
     *      ;
     */
    pub(super) fn parse_mul_expr(&mut self) -> ParseResult<Expr> {
        let mut left = self.parse_unary_expr()?;
        while self.expect_one_of(&[
            TokenKind::Operator(Operator::Mul),
            TokenKind::Operator(Operator::Div),
        ]) {
            let span_op = self.parse_op();
            let right = self.parse_unary_expr()?;

            left = Expr::Binary(BinaryExpr::new(left, span_op, right));
        }
        Ok(left)
    }

    /**
     *  UnaryExpression
     *      : (- | !) UnaryExpression
     *      | CallExpression
     *      ;
     */
    pub(super) fn parse_unary_expr(&mut self) -> ParseResult<Expr> {
        if self.expect_one_of(&[
            TokenKind::Operator(Operator::Min),
            TokenKind::Operator(Operator::Not),
        ]) {
            let span_op = self.parse_op();
            let argument = self.parse_unary_expr()?;
            return Ok(Expr::Unary(UnaryExpr::new(span_op, argument)));
        }

        self.parse_call_expr()
    }

    /**
     * CallExpression
     *      : PrimaryExpression ( "(" Arguments? ")" )*
     *      ;
     */
    fn parse_call_expr(&mut self) -> ParseResult<Expr> {
        let mut expr = self.parse_primary_expr()?;

        while self.token_is(TokenKind::ParenOpen) {
            self.eat(TokenKind::ParenOpen)?;
            let arguments = self.parse_arguments()?;
            self.eat(TokenKind::ParenClose)?;
            expr = Expr::Call(CallExpr::new(expr, arguments));
        }

        Ok(expr)
    }

    /**
     * Arguments
     *      : Expression ("," Expression)*
     *      ;
     */
    fn parse_arguments(&mut self) -> ParseResult<ArgumentList> {
        if !self.token_is(TokenKind::ParenClose) {
            let mut list = Vec::new();
            let expr = self.parse_expression()?;
            list.push(Box::new(expr));
            while self.token_is(TokenKind::Comma) {
                self.consume();
                // arguments limit
                if list.len() >= MAXIMUM_ARGS {
                    return Err(ParserError::maximum_size_error(
                        self.lexer.filename,
                        self.current_token.loc.start,
                    ));
                }
                let expr = self.parse_expression()?;
                list.push(Box::new(expr));
            }
            Ok(Some(list))
        } else {
            Ok(None)
        }
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
            TokenKind::Number | TokenKind::String | TokenKind::Boolean | TokenKind::Null => {
                self.parse_literal()
            }
            TokenKind::Identifier => self.parse_identifier_expr(),
            TokenKind::ParenOpen => self.parse_parenthesized_expr(),
            _ => {
                println!("parse_primary_expr error");
                self.log();
                return Err(ParserError::invalid_token(
                    self.lexer.filename,
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
            TokenKind::Null => self.parse_null(),
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
     *      ;
     */
    pub fn parse_identifier(&mut self) -> ParseResult<Identifier> {
        self.expect(TokenKind::Identifier)?;
        let name = self.current_token.raw.to_string();
        let expr = Identifier::new(
            name,
            Span::new(self.lexer.filename.into(), self.current_token.loc),
        );
        self.consume();
        Ok(expr)
    }

    /**
     * helper
     */
    fn parse_op(&mut self) -> WithSpan<Operator> {
        let op = Operator::from_str(&self.current_token.raw);
        let op_span = WithSpan::new(op, self.lexer.filename.to_string(), self.current_token.loc);
        self.consume();
        op_span
    }
}
