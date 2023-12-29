use std::ptr::null;

use crate::token::{Content, Token, TokenType};

// https://doc.rust-lang.org/std/boxed/index.html
// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#enum-values
pub enum Expression {
    Literal {
        token: Token,
    },

    Grouping {
        exp: Box<Expression>,
    },

    Unary {
        token: Token,
        exp: Box<Expression>,
    },

    Binary {
        left: Box<Expression>,
        token: Token,
        right: Box<Expression>,
    },
}

fn parentesize_expression(name: String, expressions: Vec<Expression>) -> String {
    let mut text: String = format!("({}", name);

    for e in expressions {
        text.push(' ');

        let txt: String = match e {
            Expression::Literal { token: t } => t.content_to_string(),
            Expression::Grouping { exp } => parentesize_expression("group".to_string(), vec![*exp]),
            Expression::Unary { token, exp } => parentesize_expression(token.lexeme, vec![*exp]),
            Expression::Binary { left, token, right } => {
                parentesize_expression(token.lexeme, vec![*left, *right])
            }
        };

        text.push_str(txt.as_str());
    }

    text.push(')');

    return text;
}

pub fn fast_test() {
    let exp: Expression = Expression::Binary {
        left: Box::new(Expression::Unary {
            token: Token {
                token_type: TokenType::Minus,
                lexeme: "-".to_string(),
                content: Content { null: null() },
                line: 1,
            },
            exp: Box::new(Expression::Literal {
                token: Token {
                    token_type: TokenType::Integer,
                    lexeme: "123".to_string(),
                    content: Content { integer: 123 },
                    line: 1,
                },
            }),
        }),
        token: Token {
            token_type: TokenType::Star,
            lexeme: "*".to_string(),
            content: Content { null: null() },
            line: 1,
        },
        right: Box::new(Expression::Grouping {
            exp: Box::new(Expression::Literal {
                token: Token {
                    token_type: TokenType::Floating,
                    lexeme: "45.67".to_string(),
                    content: Content { floating: 45.67 },
                    line: 1,
                },
            }),
        }),
    };

    println!("{}", parentesize_expression("".to_string(), vec![exp]));
}
