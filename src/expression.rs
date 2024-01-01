use crate::token::{Content, Token};

// https://doc.rust-lang.org/std/boxed/index.html
// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#enum-values
pub enum Expression {
    Literal {
        content: Content,
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

pub fn parentesize_expression(name: String, expressions: Vec<Expression>) -> String {
    let mut text: String = format!("({}", name);

    for e in expressions {
        text.push(' ');

        let txt: String = match e {
            Expression::Literal { content: c } => c.to_string(),
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
