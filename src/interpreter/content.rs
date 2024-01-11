use crate::tokenizer::token_type::TokenType;

#[derive(Debug, Clone)]
pub enum Content {
    Boolean(bool),
    Integer(i64),
    Floating(f64),
    Character(char),
    String_(String),
    Null,
}

impl Content {
    pub fn from(token_type: TokenType) -> Result<Content, ()> {
        match token_type {
            TokenType::Boolean(b) => Ok(Content::Boolean(b)),
            TokenType::Integer(i) => Ok(Content::Integer(i)),
            TokenType::Floating(f) => Ok(Content::Floating(f)),
            TokenType::Character(c) => Ok(Content::Character(c)),
            TokenType::String_(s) => Ok(Content::String_(s)),
            TokenType::Null => Ok(Content::Null),
            _ => Err(()),
        }
    }
}
