mod content;
mod error;
mod utility;

use crate::error::interpreter_error;
use crate::parser::expression::Expression;
use crate::parser::statement::Statement;
use crate::tokenizer::token::Token;
use crate::tokenizer::token_type::TokenType;
use content::Content;
use error::*;
use utility::*;

pub static INTERPRETER: Interpreter = Interpreter {};

pub struct Interpreter {
    //
}

impl Interpreter {
    pub fn interpret(&self, statements: Vec<Statement>) {
        for s in statements {
            self.execute(s);
        }
    }

    // Analogue to evaluate() but for statements.
    fn execute(&self, stmt: Statement) {
        let _ = match stmt {
            Statement::Print { expr } => self.print(*expr),
            Statement::Expr { expr } => self.expression(*expr),
            Statement::Var {
                id: _,
                op: _,
                expr: _,
            } => (), // TODO
        };
    }

    fn print(&self, expr: Expression) {
        match self.evaluate(expr) {
            Ok(c) => println!("{}", c.to_string()),
            _ => (),
        }
    }

    fn expression(&self, expr: Expression) {
        match self.evaluate(expr) {
            _ => (),
        }
    }

    // Analogue to execute() but for expressions.
    fn evaluate(&self, expr: Expression) -> Result<Content, ()> {
        let c: Content = match expr {
            Expression::Literal { token } => Content::from(token.token_type)?,
            Expression::Grouping { expr } => self.evaluate(*expr)?,
            Expression::Unary { op, right } => self.unary(op, *right)?,
            Expression::Binary { left, op, right } => self.binary(*left, op, *right)?,
            Expression::Variable { id: _ } => Content::Null, // TODO
        };

        Ok(c)
    }

    fn unary(&self, op: Token, right: Expression) -> Result<Content, ()> {
        let content: Content = self.evaluate(right)?;

        let c: Content = match op.token_type {
            // Bitwise
            TokenType::ExclamationMark => self.unary_bit_not(content, op)?,
            // Logical
            TokenType::Not => self.unary_logic_not(content, op)?,
            // Math
            TokenType::Minus => self.unary_minus(content, op)?,
            _ => return Err(()),
        };

        Ok(c)
    }

    fn unary_minus(&self, content: Content, op: Token) -> Result<Content, ()> {
        let c: Content = match content {
            Content::Integer(i) => Content::Integer(-i),
            Content::Floating(f) => Content::Floating(-f),
            _ => {
                interpreter_error(op.line, unary_unsupported(&"-", &content));
                return Err(());
            }
        };

        Ok(c)
    }

    fn unary_logic_not(&self, content: Content, _op: Token) -> Result<Content, ()> {
        Ok(Content::Boolean(!is_true(content)))
    }

    fn unary_bit_not(&self, content: Content, op: Token) -> Result<Content, ()> {
        let c: Content = match content {
            Content::Integer(i) => Content::Integer(!i),
            _ => {
                interpreter_error(op.line, unary_unsupported(&"!", &content));
                return Err(());
            }
        };

        Ok(c)
    }

    fn binary(&self, left: Expression, op: Token, right: Expression) -> Result<Content, ()> {
        let l_content: Content = self.evaluate(left)?;

        if is_logic_solved(&op.token_type, &l_content) {
            return Ok(l_content);
        }

        let r_content: Content = self.evaluate(right)?;

        let c2: Content = match op.token_type {
            // Bitwise
            TokenType::Ampersand => self.binary_bit_and(l_content, r_content, op)?,
            TokenType::Pipe => self.binary_bit_or(l_content, r_content, op)?,
            TokenType::Caret => self.binary_bit_xor(l_content, r_content, op)?,
            // Comparassion
            TokenType::Greater => self.binary_greater(l_content, r_content, op)?,
            TokenType::Less => self.binary_less(l_content, r_content, op)?,
            TokenType::EqualEqual => self.binary_equal(l_content, r_content, op)?,
            TokenType::NotEqual => self.binary_not_equal(l_content, r_content, op)?,
            TokenType::GreaterEqual => self.binary_greater_equal(l_content, r_content, op)?,
            TokenType::LessEqual => self.binary_less_equal(l_content, r_content, op)?,
            // Logical
            TokenType::And => r_content,
            TokenType::Or => r_content,
            // Math
            TokenType::Plus => self.binary_plus(l_content, r_content, op)?,
            TokenType::Minus => self.binary_minus(l_content, r_content, op)?,
            TokenType::Star => self.binary_star(l_content, r_content, op)?,
            TokenType::Slash => self.binary_slash(l_content, r_content, op)?,
            TokenType::Percentage => self.binary_percentage(l_content, r_content, op)?,
            TokenType::StarStar => self.binary_starstar(l_content, r_content, op)?,
            _ => return Err(()),
        };

        Ok(c2)
    }

    fn binary_bit_and(&self, left: Content, right: Content, op: Token) -> Result<Content, ()> {
        let c: Content = match (&left, &right) {
            (Content::Integer(i1), Content::Integer(i2)) => Content::Integer(*i1 & *i2),
            _ => {
                interpreter_error(op.line, binary_unsupported(&"&", &left, &right));
                return Err(());
            }
        };

        Ok(c)
    }

    fn binary_bit_or(&self, left: Content, right: Content, op: Token) -> Result<Content, ()> {
        let c: Content = match (&left, &right) {
            (Content::Integer(i1), Content::Integer(i2)) => Content::Integer(*i1 | *i2),
            _ => {
                interpreter_error(op.line, binary_unsupported(&"|", &left, &right));
                return Err(());
            }
        };

        Ok(c)
    }

    fn binary_bit_xor(&self, left: Content, right: Content, op: Token) -> Result<Content, ()> {
        let c: Content = match (&left, &right) {
            (Content::Integer(i1), Content::Integer(i2)) => Content::Integer(*i1 ^ *i2),
            _ => {
                interpreter_error(op.line, binary_unsupported(&"^", &left, &right));
                return Err(());
            }
        };

        Ok(c)
    }

    fn binary_greater(&self, left: Content, right: Content, op: Token) -> Result<Content, ()> {
        let c: Content = match (&left, &right) {
            (Content::Integer(i1), Content::Integer(i2)) => Content::Boolean(*i1 > *i2),
            (Content::Integer(i1), Content::Floating(f2)) => Content::Boolean((*i1 as f32) > *f2),
            (Content::Floating(f1), Content::Integer(i2)) => Content::Boolean(*f1 > (*i2 as f32)),
            (Content::Floating(f1), Content::Floating(f2)) => Content::Boolean(*f1 > *f2),
            _ => {
                interpreter_error(op.line, binary_unsupported(&">", &left, &right));
                return Err(());
            }
        };

        Ok(c)
    }

    fn binary_less(&self, left: Content, right: Content, op: Token) -> Result<Content, ()> {
        let c: Content = match (&left, &right) {
            (Content::Integer(i1), Content::Integer(i2)) => Content::Boolean(*i1 < *i2),
            (Content::Integer(i1), Content::Floating(f2)) => Content::Boolean((*i1 as f32) < *f2),
            (Content::Floating(f1), Content::Integer(i2)) => Content::Boolean(*f1 < (*i2 as f32)),
            (Content::Floating(f1), Content::Floating(f2)) => Content::Boolean(*f1 < *f2),
            _ => {
                interpreter_error(op.line, binary_unsupported(&"<", &left, &right));
                return Err(());
            }
        };

        Ok(c)
    }

    fn binary_equal(&self, left: Content, right: Content, _token: Token) -> Result<Content, ()> {
        Ok(Content::Boolean(left == right))
    }

    fn binary_not_equal(
        &self,
        left: Content,
        right: Content,
        _token: Token,
    ) -> Result<Content, ()> {
        Ok(Content::Boolean(left != right))
    }

    fn binary_greater_equal(
        &self,
        left: Content,
        right: Content,
        op: Token,
    ) -> Result<Content, ()> {
        let c: Content = match (&left, &right) {
            (Content::Integer(i1), Content::Integer(i2)) => Content::Boolean(*i1 >= *i2),
            (Content::Integer(i1), Content::Floating(f2)) => Content::Boolean((*i1 as f32) >= *f2),
            (Content::Floating(f1), Content::Integer(i2)) => Content::Boolean(*f1 >= (*i2 as f32)),
            (Content::Floating(f1), Content::Floating(f2)) => Content::Boolean(*f1 >= *f2),
            _ => {
                interpreter_error(op.line, binary_unsupported(&">=", &left, &right));
                return Err(());
            }
        };

        Ok(c)
    }

    fn binary_less_equal(&self, left: Content, right: Content, op: Token) -> Result<Content, ()> {
        let c: Content = match (&left, &right) {
            (Content::Integer(i1), Content::Integer(i2)) => Content::Boolean(*i1 <= *i2),
            (Content::Integer(i1), Content::Floating(f2)) => Content::Boolean((*i1 as f32) <= *f2),
            (Content::Floating(f1), Content::Integer(i2)) => Content::Boolean(*f1 <= (*i2 as f32)),
            (Content::Floating(f1), Content::Floating(f2)) => Content::Boolean(*f1 <= *f2),
            _ => {
                interpreter_error(op.line, binary_unsupported(&"<=", &left, &right));
                return Err(());
            }
        };

        Ok(c)
    }

    fn binary_plus(&self, left: Content, right: Content, op: Token) -> Result<Content, ()> {
        let c: Content = match (&left, &right) {
            (Content::Integer(i1), Content::Integer(i2)) => Content::Integer(*i1 + *i2),
            (Content::Integer(i1), Content::Floating(f2)) => Content::Floating((*i1 as f32) + *f2),
            (Content::Floating(f1), Content::Integer(i2)) => Content::Floating(*f1 + (*i2 as f32)),
            (Content::Floating(f1), Content::Floating(f2)) => Content::Floating(*f1 + *f2),
            (Content::String_(s1), Content::String_(s2)) => {
                Content::String_(concat_strings(&s1, &s2))
            }
            _ => {
                interpreter_error(op.line, binary_unsupported(&"+", &left, &right));
                return Err(());
            }
        };

        Ok(c)
    }

    fn binary_minus(&self, left: Content, right: Content, op: Token) -> Result<Content, ()> {
        let c: Content = match (&left, &right) {
            (Content::Integer(i1), Content::Integer(i2)) => Content::Integer(*i1 - *i2),
            (Content::Integer(i1), Content::Floating(f2)) => Content::Floating((*i1 as f32) - *f2),
            (Content::Floating(f1), Content::Integer(i2)) => Content::Floating(*f1 - (*i2 as f32)),
            (Content::Floating(f1), Content::Floating(f2)) => Content::Floating(*f1 - *f2),
            _ => {
                interpreter_error(op.line, binary_unsupported(&"-", &left, &right));
                return Err(());
            }
        };

        Ok(c)
    }

    fn binary_star(&self, left: Content, right: Content, op: Token) -> Result<Content, ()> {
        let c: Content = match (&left, &right) {
            (Content::Integer(i1), Content::Integer(i2)) => Content::Integer(*i1 * *i2),
            (Content::Integer(i1), Content::Floating(f2)) => Content::Floating((*i1 as f32) * *f2),
            (Content::Floating(f1), Content::Integer(i2)) => Content::Floating(*f1 * (*i2 as f32)),
            (Content::Floating(f1), Content::Floating(f2)) => Content::Floating(*f1 * *f2),
            _ => {
                interpreter_error(op.line, binary_unsupported(&"*", &left, &right));
                return Err(());
            }
        };

        Ok(c)
    }

    fn binary_slash(&self, left: Content, right: Content, op: Token) -> Result<Content, ()> {
        let c: Content = match (&left, &right) {
            (Content::Integer(i1), Content::Integer(i2)) => Content::Integer(*i1 / *i2),
            (Content::Integer(i1), Content::Floating(f2)) => Content::Floating((*i1 as f32) / *f2),
            (Content::Floating(f1), Content::Integer(i2)) => Content::Floating(*f1 / (*i2 as f32)),
            (Content::Floating(f1), Content::Floating(f2)) => Content::Floating(*f1 / *f2),
            _ => {
                interpreter_error(op.line, binary_unsupported(&"/", &left, &right));
                return Err(());
            }
        };

        Ok(c)
    }

    fn binary_percentage(&self, left: Content, right: Content, op: Token) -> Result<Content, ()> {
        let c: Content = match (&left, &right) {
            (Content::Integer(i1), Content::Integer(i2)) => Content::Integer(*i1 % *i2),
            (Content::Integer(i1), Content::Floating(f2)) => Content::Floating((*i1 as f32) % *f2),
            (Content::Floating(f1), Content::Integer(i2)) => Content::Floating(*f1 % (*i2 as f32)),
            (Content::Floating(f1), Content::Floating(f2)) => Content::Floating(*f1 % *f2),
            _ => {
                interpreter_error(op.line, binary_unsupported(&"%", &left, &right));
                return Err(());
            }
        };

        Ok(c)
    }

    fn binary_starstar(&self, left: Content, right: Content, op: Token) -> Result<Content, ()> {
        let c: Content = match (&left, &right) {
            (Content::Integer(i1), Content::Integer(i2)) => {
                let i3: i32 = (*i1).pow(*i2 as u32);
                Content::Integer(i3)
            }
            (Content::Integer(i1), Content::Floating(f2)) => {
                let f3: f32 = (*i1 as f32).powf(*f2);
                Content::Floating(f3)
            }
            (Content::Floating(f1), Content::Integer(i2)) => {
                let f3: f32 = (*f1).powf(*i2 as f32);
                Content::Floating(f3)
            }
            (Content::Floating(f1), Content::Floating(f2)) => {
                let f3: f32 = (*f1).powf(*f2);
                Content::Floating(f3)
            }
            _ => {
                interpreter_error(op.line, binary_unsupported(&"**", &left, &right));
                return Err(());
            }
        };

        Ok(c)
    }
}
