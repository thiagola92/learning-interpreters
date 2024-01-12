// Any logic that helps the interpreter.

use super::content::Content;
use crate::tokenizer::token_type::TokenType;

pub fn is_true(content: Content) -> bool {
    match content {
        Content::Boolean(b) => b,
        Content::Integer(i) => i != 0,
        Content::Floating(f) => f != 0.0,
        Content::Character(c) => c != '\0',
        Content::String_(s) => s.len() != 0,
        Content::Null => false,
    }
}

pub fn concat_strings(s1: &String, s2: &String) -> String {
    let mut s3 = s1.clone();
    s3.push_str(s2.as_str());
    s3
}

// Logic is solved if the content can already deduce the logic result. Basically:
//      false and XXX == false
//      true or XXX == true
pub fn is_logic_solved(token_type: &TokenType, content: &Content) -> bool {
    match token_type {
        TokenType::And => match content {
            Content::Boolean(b) => !*b,
            _ => false,
        },
        TokenType::Or => match content {
            Content::Boolean(b) => *b,
            _ => false,
        },
        _ => false,
    }
}
