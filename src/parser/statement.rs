use super::expression::Expression;
use crate::tokenizer::token::Token;

pub enum Statement {
    Var {
        identifier: Token,
    },

    VarAssign {
        identifier: Token,
        expr: Box<Expression>,
    },

    Print {
        expr: Box<Expression>,
    },

    Block {
        stmts: Vec<Statement>,
        level: u8,
    },

    Expr {
        expr: Box<Expression>,
    },
}

impl Statement {
    pub fn to_string(&self) -> String {
        match self {
            Statement::Var { identifier } => format!("(var {})", identifier.lexeme),
            Statement::VarAssign { identifier, expr } => {
                format!("(var {} {})", identifier.lexeme, expr.to_string())
            }
            Statement::Print { expr } => format!("(print {})", (*expr).to_string()),
            Statement::Block { stmts, level } => {
                let tabs: String = "\t".repeat((*level) as usize);
                let mut block: String = format!("(block");

                for stmt in stmts {
                    block.push_str(format!("\n{}{}", tabs, stmt.to_string()).as_str());
                }

                block.push_str(format!(")").as_str());
                block
            }
            Statement::Expr { expr } => format!("(expr {})", (*expr).to_string()),
        }
    }
}
