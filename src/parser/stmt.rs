use crate::{
    ast::*,
    error::ParserError,
    token::{Keyword, Operator, TokenKind},
};

use super::{parser::Parser, ParseResult, MAXIMUM_ARGS};

impl<'a> Parser<'a> {
    /**
     * StatementList
     *      : Statement*
     *      ;
     */
    pub(super) fn parse_statement_list(&mut self) -> ParseResult<Vec<Statement>> {
        let mut list = Vec::new();

        while !self.token_is(TokenKind::Eof) && !self.token_is(TokenKind::BraceClose) {
            match self.current_token.kind {
                TokenKind::Eol => self.consume(),
                _ => list.push(self.parse_statment()?),
            }
        }
        Ok(list)
    }

    /**
     *  Statement
     *      : ExpressionStatement
     *      | BlockStatement
     *      | EmptyStatement
     *      | IfStatement
     *      | ReturnStatement
     *      | VariableDeclarator
     *      | FunctionDeclaration
     *      | PrintStatement
     *      | WhileStatement
     *      | ClassDeclaration
     *      ;
     *      ...
     */
    fn parse_statment(&mut self) -> ParseResult<Statement> {
        match self.current_token.kind {
            TokenKind::BraceOpen => self.parse_block_stmt(),
            TokenKind::Semi => self.parse_empty_stmt(),
            TokenKind::ParenOpen => self.parse_expression_stmt(),
            TokenKind::Number => self.parse_expression_stmt(),
            TokenKind::String => self.parse_expression_stmt(),
            TokenKind::Null => self.parse_expression_stmt(),
            TokenKind::Operator(_) => self.parse_expression_stmt(),
            TokenKind::Identifier => self.parse_expression_stmt(),
            TokenKind::Keyword(Keyword::Let) => self.parse_variable_stmt(),
            TokenKind::Keyword(Keyword::If) => self.parse_if_stmt(),
            TokenKind::Keyword(Keyword::Fn) => self.parse_fn_declaration(),
            TokenKind::Keyword(Keyword::Return) => self.parse_return_stmt(),
            TokenKind::Keyword(Keyword::Print) => self.parse_print_stmt(),
            TokenKind::Keyword(Keyword::While) => self.parse_while_stmt(),
            TokenKind::Keyword(Keyword::Class) => self.parse_class_declaration(),
            _ => {
                println!("parse_statment error token: ");
                self.log();
                return Err(ParserError::invalid_token(
                    self.lexer.filename,
                    self.current_token.loc.start,
                ));
            }
        }
    }

    /**
     *  ExpressionStatement
     *      : Expression STMT_END
     *      ;
     *
     *  STMT_END:
     *      : ";"
     *      | "\n"
     *      | ";\n"
     *      | "}"
     *      ;
     */
    fn parse_expression_stmt(&mut self) -> ParseResult<Statement> {
        let expr = self.parse_expression()?;
        self.expect_stmt_terminator()?;

        Ok(Statement::ExprStmt(expr))
    }

    /**
     *  BlockStatement
     *      : "{" StatementList "}"
     *      ;
     */
    fn parse_block_stmt(&mut self) -> ParseResult<Statement> {
        self.eat(TokenKind::BraceOpen)?;
        self.maybe(TokenKind::Eol);
        let mut list: Vec<Statement> = Vec::new();
        if self.current_token.kind != TokenKind::BraceClose {
            list = self.parse_statement_list()?;
        }
        self.maybe(TokenKind::Eol);
        self.eat(TokenKind::BraceClose)?;
        self.maybe(TokenKind::Eol);

        Ok(Statement::Block(list))
    }

    /**
     *  EmptyStatement
     *      : ";"
     *      | ";\n"
     *      ;
     */

    fn parse_empty_stmt(&mut self) -> ParseResult<Statement> {
        self.eat(TokenKind::Semi)?;
        Ok(Statement::Empty)
    }

    /**
     *  VariableDeclarator
     *      : "let" VariableDeclaration (";")?
     *      ;
     */
    fn parse_variable_stmt(&mut self) -> ParseResult<Statement> {
        self.eat(TokenKind::Keyword(Keyword::Let))?;
        let stmt = self.parse_variable_declaration()?;
        self.expect_stmt_terminator()?;
        Ok(stmt)
    }

    /**
     * VariableDeclaration
     *      : Identifier ( "=" AssignmentExpression)?
     */
    fn parse_variable_declaration(&mut self) -> ParseResult<Statement> {
        let left = self.parse_identifier()?;
        if self.token_is(TokenKind::Operator(Operator::Assign)) {
            self.eat(TokenKind::Operator(Operator::Assign))?;
            let right = self.parse_expression()?;
            Ok(Statement::VariableDeclaration(VariableDeclaration::new(
                left,
                Some(right),
            )))
        } else {
            Ok(Statement::VariableDeclaration(VariableDeclaration::new(
                left, None,
            )))
        }
    }

    /**
     * IfStatement
     *      : "if" "(" Expression ")" Statement ( "else" Statement )?
     *
     */
    fn parse_if_stmt(&mut self) -> ParseResult<Statement> {
        self.eat(TokenKind::Keyword(Keyword::If))?;
        self.eat(TokenKind::ParenOpen)?;
        let test = self.parse_expression()?;
        self.eat(TokenKind::ParenClose)?;

        let consequent = self.parse_statment()?;

        let alternate = if self.token_is(TokenKind::Keyword(Keyword::Else)) {
            self.eat(TokenKind::Keyword(Keyword::Else))?;
            Some(self.parse_statment()?)
        } else {
            None
        };

        let if_stmt = IfStatement::new(test, consequent, alternate);
        Ok(Statement::If(if_stmt))
    }

    /**
     *  FunctionDeclaration
     *      : "function" Identifier "(" ( ParameterList )? ")" BlockStatement
     *      ;
     *
     *  ParameterList
     *      : Identifier ( "," Identifier )*
     *      ;
     */
    fn parse_fn_declaration(&mut self) -> ParseResult<Statement> {
        self.eat(TokenKind::Keyword(Keyword::Fn))?;
        self.parse_fn_body()
    }

    /**
     *  FunctionBody
     *      : Identifier "(" ( ParameterList )? ")" BlockStatement
     *      ;
     */
    fn parse_fn_body(&mut self) -> ParseResult<Statement> {
        let id = self.parse_identifier()?;

        self.eat(TokenKind::ParenOpen)?;
        let mut params: ParamList = Vec::new();
        if !self.token_is(TokenKind::ParenClose) {
            params = self.parse_params()?;
        }
        self.eat(TokenKind::ParenClose)?;

        let body = self.parse_block_stmt()?;

        if let Statement::Block(block) = body {
            let stmt = FunctionDeclaration::new(id, params, block);
            Ok(Statement::FunctionDeclaration(stmt))
        } else {
            unreachable!()
        }
    }

    /**
     * ClassDeclaration
     *      : "class" IDENTIFIER ( "extends" IDENTIFIER )? "{" FunctionBody* "}"
     *      ;
     */
    fn parse_class_declaration(&mut self) -> ParseResult<Statement> {
        self.eat(TokenKind::Keyword(Keyword::Class))?;
        let id = self.parse_identifier()?;

        if self.token_is(TokenKind::Keyword(Keyword::Extends)) {
            self.eat(TokenKind::Keyword(Keyword::Extends))?;
            self.parse_identifier()?;
        }

        self.eat(TokenKind::BraceOpen)?;
        self.maybe(TokenKind::Eol);

        let mut list = Vec::new();
        while !self.token_is(TokenKind::BraceClose) && !self.token_is(TokenKind::Eof) {
            let method = self.parse_fn_body()?;
            if let Statement::FunctionDeclaration(decl) = method {
                list.push(decl);
            } else {
                unreachable!("invalid function declarationQ");
            }
        }

        self.maybe(TokenKind::Eol);
        self.eat(TokenKind::BraceClose)?;

        let class = ClassDeclaration::new(id, list);
        Ok(Statement::ClassDeclaration(class))
    }

    /**
     * ParameterList
     *      : Identifier ( "," Identifier )*
     *      ;
     */
    fn parse_params(&mut self) -> ParseResult<ParamList> {
        let mut list = Vec::new();
        let ident = self.parse_identifier()?;
        list.push(ident);
        while self.token_is(TokenKind::Comma) {
            self.consume();
            // paramlist limit
            if list.len() >= MAXIMUM_ARGS {
                return Err(ParserError::maximum_size_error(
                    self.lexer.filename,
                    self.current_token.loc.start,
                ));
            }
            let ident = self.parse_identifier()?;
            list.push(ident);
        }
        Ok(list)
    }

    /**
     *  ReturnStatement
     *      : "return" Expression? STMT_END
     */
    fn parse_return_stmt(&mut self) -> ParseResult<Statement> {
        self.eat(TokenKind::Keyword(Keyword::Return))?;

        let argument = if !self.is_stmt_end() {
            Some(self.parse_expression()?)
        } else {
            None
        };
        let stmt = ReturnStatement::new(argument);
        self.expect_stmt_terminator()?;
        Ok(Statement::Return(stmt))
    }

    /**
     *  WhileStatement
     *      : "while" "(" expression ")" statement
     *      ;
     */
    fn parse_while_stmt(&mut self) -> ParseResult<Statement> {
        self.eat(TokenKind::Keyword(Keyword::While))?;

        self.eat(TokenKind::ParenOpen)?;
        let test = self.parse_expression()?;
        self.eat(TokenKind::ParenClose)?;

        let body = self.parse_statment()?;
        let stmt = WhileStmt::new(test, body);

        // self.expect_stmt_terminator()?;
        Ok(Statement::While(stmt))
    }

    /**
     * 后面应该挪到builtin中
     * PrintStatement
     *      : "print" Expression STMT_END
     *      ;
     */
    fn parse_print_stmt(&mut self) -> ParseResult<Statement> {
        self.eat(TokenKind::Keyword(Keyword::Print))?;
        let expr = self.parse_expression()?;
        self.expect_stmt_terminator()?;
        Ok(Statement::PrintStmt(expr))
    }
}
