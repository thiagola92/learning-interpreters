use crate::tokenizer::token::Token;

pub enum Expression {
    Binary {
        left: Box<Expression>,
        token: Token,
        right: Box<Expression>,
    },

    Unary {
        token: Token,
        right: Box<Expression>,
    },

    Grouping {
        expr: Box<Expression>,
    },

    Literal {
        token: Token,
    },
}

impl Expression {
    pub fn to_string(&self) -> String {
        match self {
            Expression::Binary { left, token, right } => {
                format!(
                    "({} {} {})",
                    token.lexeme,
                    left.to_string(),
                    right.to_string()
                )
            }
            Expression::Unary { token, right } => {
                format!("({} {})", token.lexeme, right.to_string())
            }
            Expression::Grouping { expr } => format!("(group {})", expr.to_string()),
            Expression::Literal { token } => token.lexeme.clone(),
        }
    }
}
