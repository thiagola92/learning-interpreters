use super::expression::Expression;

pub enum Statement {
    Expr { expr: Box<Expression> },
    Print { expr: Box<Expression> },
}
