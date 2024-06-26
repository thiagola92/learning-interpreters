use crate::tokenizer::token::Token;

pub enum Expression {
    Assignment {
        id: Token,
        op: Token,
        right: Box<Expression>,
    },

    Logical {
        left: Box<Expression>,
        op: Token,
        right: Box<Expression>,
    },

    Binary {
        left: Box<Expression>,
        op: Token,
        right: Box<Expression>,
    },

    Unary {
        op: Token,
        right: Box<Expression>,
    },

    Grouping {
        expr: Box<Expression>,
    },

    Variable {
        id: Token,
    },

    Literal {
        token: Token,
    },
}

impl Expression {
    pub fn to_string(&self) -> String {
        match self {
            Expression::Assignment { id, op, right } => {
                format!("({} {} {})", id.lexeme, op.lexeme, right.to_string())
            }
            Expression::Logical { left, op, right } => {
                format!("({} {} {})", op.lexeme, left.to_string(), right.to_string())
            }
            Expression::Binary { left, op, right } => {
                format!("({} {} {})", op.lexeme, left.to_string(), right.to_string())
            }
            Expression::Unary { op, right } => {
                format!("({} {})", op.lexeme, right.to_string())
            }
            Expression::Grouping { expr } => format!("(group {})", expr.to_string()),
            Expression::Variable { id } => format!("(variable {})", id.lexeme.clone()),
            Expression::Literal { token } => token.lexeme.clone(),
        }
    }
}
