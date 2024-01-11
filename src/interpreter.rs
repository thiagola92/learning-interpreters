mod content;

use crate::parser::expression::Expression;
use crate::parser::expression::Expression::*;
use crate::tokenizer::token_type::TokenType;
use crate::tokenizer::token_type::TokenType::*;
use content::Content;

pub fn evaluate(expr: Expression) {
    // match expr {
    //     Literal { token } => match token.token_type {
    //         Boolean { content }
    //     },
    // }
}
