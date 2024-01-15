use super::expression::Expression;
use crate::tokenizer::token::Token;

pub enum Statement {
    Var { id: Token },

    VarAssign { id: Token, expr: Box<Expression> },

    Print { expr: Box<Expression> },

    Expr { expr: Box<Expression> },
}

impl Statement {
    pub fn to_string(&self) -> String {
        match self {
            Statement::Print { expr } => format!("(print {})", (*expr).to_string()),
            Statement::Expr { expr } => format!("(expr {})", (*expr).to_string()),
            Statement::Var { id } => format!("(var {})", id.lexeme),
            Statement::VarAssign { id, expr } => {
                format!("(var {} {})", id.lexeme, expr.to_string())
            }
        }
    }
}
