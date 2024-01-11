use crate::tokenizer::token_type::TokenType;

#[derive(Debug, Clone, PartialEq)]
pub enum Content {
    Boolean(bool),
    Integer(i32),
    Floating(f32),
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

    pub fn type_to_string(&self) -> String {
        match self {
            Content::Boolean(_) => "boolean".to_string(),
            Content::Integer(_) => "integer".to_string(),
            Content::Floating(_) => "floating".to_string(),
            Content::Character(_) => "char".to_string(),
            Content::String_(_) => "string".to_string(),
            Content::Null => "null".to_string(),
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            Content::Boolean(b) => format!("{}", b),
            Content::Integer(i) => format!("{}", i),
            Content::Floating(f) => format!("{}", f),
            Content::Character(c) => format!("{}", c),
            Content::String_(s) => format!("{}", *s),
            Content::Null => "null".to_string(),
        }
    }
}
