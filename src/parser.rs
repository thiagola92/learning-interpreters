pub mod debug;
pub mod error;
pub mod expression;
pub mod statement;
pub mod utility;

use crate::error::parser_error;
use crate::tokenizer::token::Token;
use crate::tokenizer::token_type::TokenType;
use error::*;
use expression::Expression;
use statement::Statement;
use std::mem::discriminant;
use utility::*;

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Parser {
        Parser {
            tokens: tokens,
            current: 0,
        }
    }

    pub fn parse(&mut self) -> Vec<Statement> {
        let mut statements: Vec<Statement> = Vec::new();

        while !self.is_eof() {
            match self.declaration() {
                Ok(s) => statements.push(s),
                _ => self.synchronize(),
            }
        }

        statements
    }

    fn declaration(&mut self) -> Result<Statement, ()> {
        match self.peek().token_type {
            TokenType::Var => self.var(),
            _ => self.statement(),
        }
    }

    fn var(&mut self) -> Result<Statement, ()> {
        self.advance(); // Consume "var" token.

        if !self.is_token(&IDENTIFIER) {
            parser_error(self.peek().line, EXPECT_VAR_IDENTIFIER.to_string());
            return Err(());
        }

        let var: Statement;
        let id: Token = self.advance().clone();

        if self.advance_if_is(&TokenType::Equal) {
            var = Statement::VarAssign {
                id: id,
                expr: Box::new(self.expression()?),
            }
        } else {
            var = Statement::Var { id: id }
        }

        if self.advance_if_is(&TokenType::Newline) {
            Ok(var)
        } else {
            parser_error(self.peek().line, EXPECT_NEWLINE.to_string());
            Err(())
        }
    }

    fn statement(&mut self) -> Result<Statement, ()> {
        match self.peek().token_type {
            TokenType::Print => self.print(),
            TokenType::Indent(lvl) => self.block(lvl),
            TokenType::Newline => self.empty_line(),
            _ => self.expr(),
        }
    }

    fn print(&mut self) -> Result<Statement, ()> {
        self.advance(); // Consume "print" token.

        let expr: Expression = self.expression()?;

        if self.advance_if_is(&TokenType::Newline) {
            Ok(Statement::Print {
                expr: Box::new(expr),
            })
        } else {
            parser_error(self.peek().line, EXPECT_NEWLINE.to_string());
            Err(())
        }
    }

    fn block(&mut self, lvl: u8) -> Result<Statement, ()> {
        self.advance(); // Consume "tab" token.

        let mut statements: Vec<Statement> = Vec::new();

        while !self.is_eof() {
            match self.peek().token_type {
                TokenType::Indent(i) => {
                    if i == lvl {
                        self.advance();
                        continue;
                    } else if i < lvl {
                        break;
                    }
                }
                _ => (),
            }

            match self.declaration() {
                Ok(s) => statements.push(s),
                _ => self.synchronize(),
            }
        }

        Ok(Statement::Block {
            stmts: statements,
            lvl: lvl,
        })
    }

    fn empty_line(&mut self) -> Result<Statement, ()> {
        Err(()) // Force synchronization to an useful line.
    }

    fn expr(&mut self) -> Result<Statement, ()> {
        let expr: Expression = self.expression()?;

        if self.advance_if_is(&TokenType::Newline) {
            Ok(Statement::Expr {
                expr: Box::new(expr),
            })
        } else {
            parser_error(self.peek().line, EXPECT_NEWLINE.to_string());
            Err(())
        }
    }

    fn expression(&mut self) -> Result<Expression, ()> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expression, ()> {
        let mut expr: Expression = self.equality()?;

        if self.advance_if_is_any_of(&ASSIGNMENTS) {
            let op: Token = self.previous().clone();
            let right: Expression = self.assignment()?;

            expr = match expr {
                Expression::Variable { id } => Expression::Assignment {
                    id: id,
                    op: op,
                    right: Box::new(right),
                },
                _ => {
                    parser_error(op.line, invalid_var_on_assignment(&op.lexeme));
                    return Err(());
                }
            }
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expression, ()> {
        let mut expr: Expression = self.comparison()?;

        while self.advance_if_is_any_of(&EQUALITIES) {
            let op: Token = self.previous().clone();
            let right: Expression = self.comparison()?;

            expr = Expression::Binary {
                left: Box::new(expr),
                op: op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression, ()> {
        let mut expr: Expression = self.term()?;

        while self.advance_if_is_any_of(&COMPARASIONS) {
            let op: Token = self.previous().clone();
            let right: Expression = self.term()?;

            expr = Expression::Binary {
                left: Box::new(expr),
                op: op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression, ()> {
        let mut expr: Expression = self.factorization()?;

        while self.advance_if_is_any_of(&TERMS) {
            let op: Token = self.previous().clone();
            let right: Expression = self.factorization()?;

            expr = Expression::Binary {
                left: Box::new(expr),
                op: op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factorization(&mut self) -> Result<Expression, ()> {
        let mut expr: Expression = self.unary()?;

        while self.advance_if_is_any_of(&FACTORIZATIONS) {
            let op: Token = self.previous().clone();
            let right: Expression = self.unary()?;

            expr = Expression::Binary {
                left: Box::new(expr),
                op: op,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression, ()> {
        if self.advance_if_is_any_of(&UNARIES) {
            let op: Token = self.previous().clone();
            let mut right: Expression = self.unary()?;

            right = Expression::Unary {
                op: op,
                right: Box::new(right),
            };

            Ok(right)
        } else {
            Ok(self.primary()?)
        }
    }

    fn primary(&mut self) -> Result<Expression, ()> {
        if self.advance_if_is_any_of(&LITERALS) {
            let expr: Expression = Expression::Literal {
                token: self.previous().clone(),
            };

            Ok(expr)
        } else if self.advance_if_is(&IDENTIFIER) {
            let expr: Expression = Expression::Variable {
                id: self.previous().clone(),
            };

            Ok(expr)
        } else if self.advance_if_is(&TokenType::ParenthesisOpen) {
            let mut expr: Expression = self.expression()?;

            expr = Expression::Grouping {
                expr: Box::new(expr),
            };

            if self.advance_if_is(&TokenType::ParenthesisClose) {
                Ok(expr)
            } else {
                parser_error(self.peek().line, EXPECT_CLOSE_PARENTHESIS.to_string());
                Err(())
            }
        } else {
            parser_error(self.peek().line, EXPECT_EXPRESSION.to_string());
            Err(())
        }
    }

    // Synchronize to a state that we expect everything to be okay.
    // This is used after some syntax error in code.
    fn synchronize(&mut self) {
        self.advance(); // Skip token that raised error.

        while !self.is_eof() {
            if is_statement(&self.peek().token_type) {
                break;
            } else {
                self.advance();
            }
        }
    }

    // Check if current token is the desired token.
    fn is_token(&self, token_type: &TokenType) -> bool {
        !self.is_eof() && discriminant(&self.peek().token_type) == discriminant(token_type)
    }

    // Check if reached EOF.
    fn is_eof(&self) -> bool {
        match self.peek().token_type {
            TokenType::Eof => true,
            _ => false,
        }
    }

    // Return the current token and advance to next token.
    fn advance(&mut self) -> &Token {
        if !self.is_eof() {
            self.current += 1
        };
        self.previous()
    }

    // Advance if current token is the desired token, returns if it was.
    fn advance_if_is(&mut self, token_type: &TokenType) -> bool {
        if self.is_token(&token_type) {
            self.advance();
            true
        } else {
            false
        }
    }

    // Advance if current token is any of the tokens, returns if it was.
    fn advance_if_is_any_of(&mut self, token_types: &[TokenType]) -> bool {
        for token_type in token_types {
            if self.advance_if_is(&token_type) {
                return true;
            }
        }
        false
    }

    // Get current token.
    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    // Get previous token.
    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }
}
