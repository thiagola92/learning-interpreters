use crate::error::{parser_error, EXPECT_CLOSE_PARENTHESIS, EXPECT_EXPRESSION};
use crate::expression::Expression;
use crate::token::{Token, TokenType, TokenType::*};

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

    pub fn parse(&mut self) -> Option<Expression> {
        match self.expression() {
            Ok(e) => Some(e),
            _ => None,
        }
    }

    fn expression(&mut self) -> Result<Expression, ()> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expression, ()> {
        let mut exp: Expression = match self.comparison() {
            Ok(e) => e,
            _ => return Err(()),
        };

        while self.any(vec![Equal, NotEqual]) {
            exp = match self.comparison() {
                Ok(e) => Expression::Binary {
                    left: Box::new(exp),
                    token: self.previous().clone(),
                    right: Box::new(e),
                },
                _ => return Err(()),
            }
        }

        Ok(exp)
    }

    fn comparison(&mut self) -> Result<Expression, ()> {
        let mut exp: Expression = match self.term() {
            Ok(e) => e,
            _ => return Err(()),
        };

        while self.any(vec![LessEqual, GreaterEqual, Less, Greater]) {
            exp = match self.term() {
                Ok(e) => Expression::Binary {
                    left: Box::new(exp),
                    token: self.previous().clone(),
                    right: Box::new(e),
                },
                _ => return Err(()),
            }
        }

        Ok(exp)
    }

    fn term(&mut self) -> Result<Expression, ()> {
        let mut exp: Expression = match self.factorization() {
            Ok(e) => e,
            _ => return Err(()),
        };

        while self.any(vec![Plus, Minus]) {
            exp = match self.factorization() {
                Ok(e) => Expression::Binary {
                    left: Box::new(exp),
                    token: self.previous().clone(),
                    right: Box::new(e),
                },
                _ => return Err(()),
            }
        }

        Ok(exp)
    }

    fn factorization(&mut self) -> Result<Expression, ()> {
        let mut exp: Expression = match self.unary() {
            Ok(e) => e,
            _ => return Err(()),
        };

        while self.any(vec![Star, Slash]) {
            exp = match self.unary() {
                Ok(e) => Expression::Binary {
                    left: Box::new(exp),
                    token: self.previous().clone(),
                    right: Box::new(e),
                },
                _ => return Err(()),
            }
        }

        Ok(exp)
    }

    fn unary(&mut self) -> Result<Expression, ()> {
        if self.any(vec![Not, Minus]) {
            match self.unary() {
                Ok(e) => Ok(Expression::Unary {
                    token: self.previous().clone(),
                    exp: Box::new(e),
                }),
                _ => Err(()),
            }
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expression, ()> {
        if self.any(vec![
            True, False, Integer, Floating, Character, String, Null,
        ]) {
            return Ok(Expression::Literal {
                token: self.previous().clone(),
            });
        }

        if self.any(vec![ParenthesisOpen]) {
            let exp: Expression = match self.expression() {
                Ok(e) => e,
                _ => return Err(()),
            };

            match self.consume(ParenthesisClose, EXPECT_CLOSE_PARENTHESIS) {
                Ok(..) => return Ok(Expression::Grouping { exp: Box::new(exp) }),
                _ => (),
            };
        }

        parser_error(self.peek(), EXPECT_EXPRESSION);

        Err(())
    }

    fn any(&mut self, types: Vec<TokenType>) -> bool {
        for token_type in types.iter() {
            if self.check(token_type) {
                self.next();
                return true;
            }
        }

        false
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<(), ()> {
        if self.check(&token_type) {
            self.next();
            Ok(())
        } else {
            parser_error(self.peek(), msg);
            Err(())
        }
    }

    fn synchronize(&mut self) {
        // Skip token that raised error.
        self.next();

        while !self.is_eof() {
            match self.peek().token_type {
                Var | Const | Enum | Signal | Func | Proc | Struct | Class | Singleton
                | Interface | Constructor | Destructor | Import | When | AtSign | Assert | Test
                | Breakpoint => return,
                _ => self.next(),
            };
        }
    }

    fn check(&self, token_type: &TokenType) -> bool {
        !self.is_eof() && self.peek().token_type == *token_type
    }

    fn next(&mut self) -> &Token {
        if !self.is_eof() {
            self.current += 1
        };
        self.previous()
    }

    fn is_eof(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.current).unwrap()
    }

    fn previous(&self) -> &Token {
        self.tokens.get(self.current - 1).unwrap()
    }
}
