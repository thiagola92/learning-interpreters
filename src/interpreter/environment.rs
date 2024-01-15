use std::collections::HashMap;

use super::content::Content;
use super::error::*;
use crate::error::interpreter_error;
use crate::tokenizer::token::Token;

pub struct Environment {
    // I'm thinking in naming vars/consts/etc
    // Making a HashMap for each, this would MAY help me control who can be reassigned.
    values: HashMap<String, Content>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn get(&self, token: &Token) -> Result<Content, ()> {
        match self.values.get(&token.lexeme) {
            Some(v) => Ok(v.clone()),
            None => {
                interpreter_error(token.line, variable_undefined(&token.lexeme.to_string()));
                Err(())
            }
        }
    }

    pub fn define(&mut self, token: &Token, value: Content) {
        self.values.insert(token.lexeme.clone(), value);
    }

    pub fn assign(&mut self, token: &Token, value: Content) -> Result<Content, ()> {
        if self.values.contains_key(&token.lexeme) {
            self.values.insert(token.lexeme.clone(), value);
            Ok(Content::Null)
        } else {
            interpreter_error(token.line, variable_undefined(&token.lexeme.to_string()));
            Err(())
        }
    }
}
