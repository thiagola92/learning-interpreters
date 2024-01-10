pub mod debug;
pub mod error;
pub mod expression;

use crate::error::parser_error;
use crate::tokenizer::token::Token;
use crate::tokenizer::token_type::TokenType;
use crate::tokenizer::token_type::TokenType::*;
use error::*;
use expression::Expression;
use expression::Expression::*;
use std::mem::discriminant;

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

    pub fn parse(&mut self) -> Vec<Expression> {
        let mut expressions: Vec<Expression> = Vec::new();

        // Temporary because we don't have scope yet.
        self.advance();

        while !self.is_eof() {
            match self.expression() {
                Ok(e) => expressions.push(e),
                _ => self.synchronize(),
            }
        }

        expressions
    }

    fn expression(&mut self) -> Result<Expression, ()> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expression, ()> {
        let mut expr: Expression = self.comparison()?;
        let token_types: Vec<TokenType> = vec![EqualEqual, NotEqual];

        while self.advance_if_is_any_of(&token_types) {
            let token: Token = self.previous().clone();
            let right: Expression = self.comparison()?;

            expr = Binary {
                left: Box::new(expr),
                token: token,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expression, ()> {
        let mut expr: Expression = self.term()?;
        let token_types: Vec<TokenType> = vec![LessEqual, GreaterEqual, Less, Greater];

        while self.advance_if_is_any_of(&token_types) {
            let token: Token = self.previous().clone();
            let right: Expression = self.term()?;

            expr = Binary {
                left: Box::new(expr),
                token: token,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expression, ()> {
        let mut expr: Expression = self.factorization()?;
        let token_types: Vec<TokenType> = vec![Plus, Minus, Ampersand, Pipe, Caret];

        while self.advance_if_is_any_of(&token_types) {
            let token: Token = self.previous().clone();
            let right: Expression = self.factorization()?;

            expr = Binary {
                left: Box::new(expr),
                token: token,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn factorization(&mut self) -> Result<Expression, ()> {
        let mut expr: Expression = self.unary()?;
        let token_types: Vec<TokenType> = vec![Star, Slash, GreaterGreater, LessLess, StarStar];

        while self.advance_if_is_any_of(&token_types) {
            let token: Token = self.previous().clone();
            let right: Expression = self.unary()?;

            expr = Binary {
                left: Box::new(expr),
                token: token,
                right: Box::new(right),
            };
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expression, ()> {
        let token_types: Vec<TokenType> = vec![Not, Minus, ExclamationMark];

        if self.advance_if_is_any_of(&token_types) {
            let token: Token = self.previous().clone();
            let mut right: Expression = self.unary()?;

            right = Unary {
                token: token,
                right: Box::new(right),
            };

            Ok(right)
        } else {
            Ok(self.primary()?)
        }
    }

    fn primary(&mut self) -> Result<Expression, ()> {
        // Ignore the content, we just need a vector with this types.
        let token_types: Vec<TokenType> = vec![
            Boolean { content: false },
            Integer { content: 0 },
            Floating { content: 0.0 },
            Character { content: '\0' },
            String_ {
                content: String::new(),
            },
            Null,
        ];

        if self.advance_if_is_any_of(&token_types) {
            let expr: Expression = Literal {
                token: self.previous().clone(),
            };

            Ok(expr)
        } else if self.advance_if_is(&ParenthesisOpen) {
            let mut expr: Expression = self.expression()?;

            expr = Grouping {
                expr: Box::new(expr),
            };

            if self.advance_if_is(&ParenthesisClose) {
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
            match self.peek().token_type {
                Var | Const | Enum | Signal | Func | Coro | Struct | Class | Singleton
                | Interface | Constructor | Destructor | Import | When | AtSign | Assert | Test
                | Breakpoint => break,
                _ => self.advance(),
            };
        }
    }

    // Check if current token is the desired token.
    fn is_token(&self, token_type: &TokenType) -> bool {
        !self.is_eof() && discriminant(&self.peek().token_type) == discriminant(token_type)
    }

    // Check if reached/passed EOF.
    fn is_eof(&self) -> bool {
        self.current >= self.tokens.len()
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
    fn advance_if_is_any_of(&mut self, token_types: &Vec<TokenType>) -> bool {
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
