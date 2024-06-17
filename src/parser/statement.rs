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

    If {
        condition: Box<Expression>,
        statement: Box<Statement>,
    },

    IfElse {
        condition: Box<Expression>,
        if_statement: Box<Statement>,
        else_statement: Box<Statement>,
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
                let mut block: String = format!("(block-{}", *level);

                for stmt in stmts {
                    block.push_str(format!("\n{}{}", tabs, stmt.to_string()).as_str());
                }

                block.push_str(format!(")").as_str());
                block
            }
            Statement::If {
                condition,
                statement,
            } => format!("(if {} {})", condition.to_string(), statement.to_string()),
            Self::IfElse {
                condition,
                if_statement,
                else_statement,
            } => format!(
                "(if {} {} else {})",
                condition.to_string(),
                if_statement.to_string(),
                else_statement.to_string()
            ),
            Statement::Expr { expr } => format!("(expr {})", (*expr).to_string()),
        }
    }
}
