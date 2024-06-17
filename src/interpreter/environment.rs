use std::collections::HashMap;

use super::content::Content;
use super::error::*;
use crate::error::interpreter_error;
use crate::tokenizer::token::Token;

// I'm thinking in transforming in Enum, so I wouldn't need Option<>.
#[derive(Clone)]
pub struct Environment {
    // I'm thinking in naming vars/consts/etc
    // Making a HashMap for each, this would MAY help me control who can be reassigned.
    values: HashMap<String, Content>,
    pub enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn from(environment: Environment) -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: Some(Box::new(environment)),
        }
    }

    pub fn get(&self, token: &Token) -> Result<Content, ()> {
        match self.values.get(&token.lexeme) {
            Some(v) => return Ok(v.clone()),
            None => (),
        };

        match &self.enclosing {
            Some(e) => (*e).get(token),
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
            return Ok(Content::Null);
        }

        match &mut self.enclosing {
            Some(e) => (*e).assign(token, value),
            None => {
                interpreter_error(token.line, variable_undefined(&token.lexeme.to_string()));
                Err(())
            }
        }
    }
}
