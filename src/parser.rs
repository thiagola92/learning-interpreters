use crate::error::{error, EXPECT_CLOSE_PARENTHESIS};
use crate::expression::Expression;
use crate::token::{Token, TokenType, TokenType::*};

struct Parser {
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

    fn expression(&mut self) -> Expression {
        self.equality()
    }

    fn equality(&mut self) -> Expression {
        let mut exp: Expression = self.comparison();

        while self.any(vec![Equal, NotEqual]) {
            exp = Expression::Binary {
                left: Box::new(exp),
                token: self.previous().clone(),
                right: Box::new(self.comparison()),
            }
        }

        exp
    }

    fn comparison(&mut self) -> Expression {
        let mut exp: Expression = self.term();

        while self.any(vec![LessEqual, GreaterEqual, Less, Greater]) {
            exp = Expression::Binary {
                left: Box::new(exp),
                token: self.previous().clone(),
                right: Box::new(self.term()),
            }
        }

        exp
    }

    fn term(&mut self) -> Expression {
        let mut exp: Expression = self.factorization();

        while self.any(vec![Plus, Minus]) {
            exp = Expression::Binary {
                left: Box::new(exp),
                token: self.previous().clone(),
                right: Box::new(self.factorization()),
            }
        }

        exp
    }

    fn factorization(&mut self) -> Expression {
        let mut exp: Expression = self.unary();

        while self.any(vec![Star, Slash]) {
            exp = Expression::Binary {
                left: Box::new(exp),
                token: self.previous().clone(),
                right: Box::new(self.unary()),
            }
        }

        exp
    }

    fn unary(&mut self) -> Expression {
        if self.any(vec![Not, Minus]) {
            Expression::Unary {
                token: self.previous().clone(),
                exp: Box::new(self.unary()),
            }
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Expression {
        if self.any(vec![ParenthesisOpen]) {
            let exp: Expression = self.expression();
            self.consume(ParenthesisClose, EXPECT_CLOSE_PARENTHESIS);
            Expression::Grouping { exp: Box::new(exp) }
        } else {
            Expression::Literal {
                token: self.next().clone(),
            }
        }
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

    fn check(&self, token_type: &TokenType) -> bool {
        !self.is_eof() && self.peek().token_type == *token_type
    }

    fn next(&mut self) -> &Token {
        if !self.is_eof() {
            self.current += 1
        };
        self.previous()
    }

    fn consume(&mut self, token_type: TokenType, msg: &str) {
        if self.check(&token_type) {
            self.next();
        }

        error(self.peek().line, msg)
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
