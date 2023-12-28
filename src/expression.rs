use crate::token::Content;
use crate::token::Token;

// https://doc.rust-lang.org/std/boxed/index.html
// https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#enum-values
enum Expression {
    Literal {
        value: Content,
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
