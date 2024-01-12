use super::error::*;
use super::expression::Expression;
use crate::error::parser_error;
use crate::tokenizer::token::Token;

pub enum Statement {
    Var {
        id: Token,
        op: Option<Token>,
        expr: Option<Box<Expression>>,
    },

    Print {
        expr: Box<Expression>,
    },

    Expr {
        expr: Box<Expression>,
    },
}

impl Statement {
    pub fn to_string(&self) -> String {
        match self {
            Statement::Print { expr } => format!("(print {})", (*expr).to_string()),
            Statement::Expr { expr } => format!("(expr {})", (*expr).to_string()),
            Statement::Var { id, op, expr } => match op {
                None => format!("(var {})", id.lexeme),
                Some(t) => format!(
                    "(var {} {} {})",
                    id.lexeme,
                    t.lexeme,
                    match expr {
                        Some(e) => e.to_string(),
                        None => {
                            parser_error(t.line, EXPECT_VAR_EXPRESSION.to_string());
                            "null".to_string()
                        }
                    }
                ),
            },
        }
    }
}
