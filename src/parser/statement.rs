use super::expression::Expression;

pub enum Statement {
    Print { expr: Box<Expression> },
    Expr { expr: Box<Expression> },
}

impl Statement {
    pub fn to_string(&self) -> String {
        match self {
            Statement::Print { expr } => format!("(print {})", (*expr).to_string()),
            Statement::Expr { expr } => format!("(expr {})", (*expr).to_string()),
        }
    }
}
