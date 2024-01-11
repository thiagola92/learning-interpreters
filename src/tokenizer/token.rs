use super::token_type::TokenType;
use super::token_type::TokenType::*;

#[derive(Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

impl Token {
    pub fn to_string(&self) -> String {
        match &self.token_type {
            Boolean(content) => format!("Boolean({})", content),
            Integer(content) => format!("Integer({})", content),
            Floating(content) => format!("Floating({})", content),
            Character(content) => format!("Character({})", content),
            String_(content) => format!("String_({})", content),
            Indent(level) => format!("Indent({})", level),
            Identifier(name) => format!("Identifier({})", name),
            Comment(content) => format!("Comment({})", content),
            _ => format!("{:?}", &self.token_type),
        }
    }
}
