mod content;
mod environment;
mod error;
mod utility;

use crate::error::interpreter_error;
use crate::parser::expression::Expression;
use crate::parser::statement::Statement;
use crate::tokenizer::token::Token;
use crate::tokenizer::token_type::TokenType;
use content::Content;
use environment::Environment;
use error::*;
use utility::*;

pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: Vec<Statement>) {
        for s in statements {
            self.execute(s);
        }
    }

    // Analogue to evaluate() but for statements.
    fn execute(&mut self, stmt: Statement) {
        let _ = match stmt {
            Statement::Var { identifier } => self.var(identifier),
            Statement::VarAssign { identifier, expr } => self.var_assign(identifier, *expr),
            Statement::Print { expr } => self.print(*expr),
            Statement::Block { stmts, level: _ } => self.block(stmts),
            Statement::Expr { expr } => self.expression(*expr),
        };
    }

    fn var(&mut self, id: Token) {
        self.environment.define(&id, Content::Null);
    }

    fn var_assign(&mut self, id: Token, expr: Expression) {
        let expr = match self.evaluate(expr) {
            Ok(c) => c,
            _ => return,
        };

        self.environment.define(&id, expr);
    }

    fn print(&mut self, expr: Expression) {
        match self.evaluate(expr) {
            Ok(c) => println!("{}", c.to_string()),
            _ => (),
        }
    }

    fn block(&mut self, stmts: Vec<Statement>) {
        self.environment = Environment::from(self.environment.clone());

        for stmt in stmts {
            self.execute(stmt)
        }

        self.environment = match &self.environment.enclosing {
            Some(e) => *e.clone(),
            None => self.environment.clone(),
        }
    }

    fn expression(&mut self, expr: Expression) {
        match self.evaluate(expr) {
            _ => (),
        }
    }

    // Analogue to execute() but for expressions.
    fn evaluate(&mut self, expr: Expression) -> Result<Content, ()> {
        let c: Content = match expr {
            Expression::Literal { token } => Content::from(token.token_type)?,
            Expression::Variable { id } => self.environment.get(&id)?,
            Expression::Grouping { expr } => self.evaluate(*expr)?,
            Expression::Unary { op, right } => self.unary(op, *right)?,
            Expression::Binary { left, op, right } => self.binary(*left, op, *right)?,
            Expression::Assignment { id, op, right } => self.assignment(id, op, *right)?,
        };

        Ok(c)
    }

    fn unary(&mut self, op: Token, right: Expression) -> Result<Content, ()> {
        let content: Content = self.evaluate(right)?;

        let c: Content = match op.token_type {
            // Bitwise
            TokenType::ExclamationMark => self.unary_exclamation_mark(content, op)?,
            // Logical
            TokenType::Not => self.unary_not(content, op)?,
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
                interpreter_error(op.line, unary_unsupported(&op.lexeme, &content));
                return Err(());
            }
        };

        Ok(c)
    }

    fn unary_not(&self, content: Content, _op: Token) -> Result<Content, ()> {
        Ok(Content::Boolean(!is_true(content)))
    }

    fn unary_exclamation_mark(&self, content: Content, op: Token) -> Result<Content, ()> {
        let c: Content = match content {
            Content::Integer(i) => Content::Integer(!i),
            _ => {
                interpreter_error(op.line, unary_unsupported(&op.lexeme, &content));
                return Err(());
            }
        };

        Ok(c)
    }

    fn binary(&mut self, left: Expression, op: Token, right: Expression) -> Result<Content, ()> {
        let l_content: Content = self.evaluate(left)?;

        if is_logic_solved(&op.token_type, &l_content) {
            return Ok(l_content);
        }

        let r_content: Content = self.evaluate(right)?;

        let c2: Content = match op.token_type {
            // Bitwise
            TokenType::Ampersand => self.binary_ampersand(l_content, r_content, op)?,
            TokenType::Pipe => self.binary_pipe(l_content, r_content, op)?,
            TokenType::Caret => self.binary_caret(l_content, r_content, op)?,
            TokenType::GreaterGreater => self.binary_greater_greater(l_content, r_content, op)?,
            TokenType::LessLess => self.binary_less_less(l_content, r_content, op)?,
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

    fn binary_ampersand(&self, left: Content, right: Content, op: Token) -> Result<Content, ()> {
        let c: Content = match (&left, &right) {
            (Content::Integer(i1), Content::Integer(i2)) => Content::Integer(*i1 & *i2),
            _ => {
                interpreter_error(op.line, binary_unsupported(&op.lexeme, &left, &right));
                return Err(());
            }
        };

        Ok(c)
    }

    fn binary_pipe(&self, left: Content, right: Content, op: Token) -> Result<Content, ()> {
        let c: Content = match (&left, &right) {
            (Content::Integer(i1), Content::Integer(i2)) => Content::Integer(*i1 | *i2),
            _ => {
                interpreter_error(op.line, binary_unsupported(&op.lexeme, &left, &right));
                return Err(());
            }
        };

        Ok(c)
    }

    fn binary_caret(&self, left: Content, right: Content, op: Token) -> Result<Content, ()> {
        let c: Content = match (&left, &right) {
            (Content::Integer(i1), Content::Integer(i2)) => Content::Integer(*i1 ^ *i2),
            _ => {
                interpreter_error(op.line, binary_unsupported(&op.lexeme, &left, &right));
                return Err(());
            }
        };

        Ok(c)
    }

    fn binary_greater_greater(
        &self,
        left: Content,
        right: Content,
        op: Token,
    ) -> Result<Content, ()> {
        let c: Content = match (&left, &right) {
            (Content::Integer(i1), Content::Integer(i2)) => Content::Integer(*i1 >> *i2),
            _ => {
                interpreter_error(op.line, binary_unsupported(&op.lexeme, &left, &right));
                return Err(());
            }
        };

        Ok(c)
    }

    fn binary_less_less(&self, left: Content, right: Content, op: Token) -> Result<Content, ()> {
        let c: Content = match (&left, &right) {
            (Content::Integer(i1), Content::Integer(i2)) => Content::Integer(*i1 << *i2),
            _ => {
                interpreter_error(op.line, binary_unsupported(&op.lexeme, &left, &right));
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
                interpreter_error(op.line, binary_unsupported(&op.lexeme, &left, &right));
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
                interpreter_error(op.line, binary_unsupported(&op.lexeme, &left, &right));
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
                interpreter_error(op.line, binary_unsupported(&op.lexeme, &left, &right));
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
                interpreter_error(op.line, binary_unsupported(&op.lexeme, &left, &right));
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
                interpreter_error(op.line, binary_unsupported(&op.lexeme, &left, &right));
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
                interpreter_error(op.line, binary_unsupported(&op.lexeme, &left, &right));
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
                interpreter_error(op.line, binary_unsupported(&op.lexeme, &left, &right));
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
                interpreter_error(op.line, binary_unsupported(&op.lexeme, &left, &right));
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
                interpreter_error(op.line, binary_unsupported(&op.lexeme, &left, &right));
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
                interpreter_error(op.line, binary_unsupported(&op.lexeme, &left, &right));
                return Err(());
            }
        };

        Ok(c)
    }

    fn assignment(&mut self, id: Token, op: Token, right: Expression) -> Result<Content, ()> {
        let mut c: Content = self.evaluate(right)?;

        c = match op.token_type {
            TokenType::Equal => self.environment.assign(&id, c)?,
            TokenType::PlusEqual => {
                let left = self.environment.get(&id)?;
                c = self.binary_plus(left, c, op)?;
                self.environment.assign(&id, c)?
            }
            TokenType::MinusEqual => {
                let left = self.environment.get(&id)?;
                c = self.binary_minus(left, c, op)?;
                self.environment.assign(&id, c)?
            }
            TokenType::StarEqual => {
                let left = self.environment.get(&id)?;
                c = self.binary_star(left, c, op)?;
                self.environment.assign(&id, c)?
            }
            TokenType::SlashEqual => {
                let left = self.environment.get(&id)?;
                c = self.binary_slash(left, c, op)?;
                self.environment.assign(&id, c)?
            }
            TokenType::PercentageEqual => {
                let left = self.environment.get(&id)?;
                c = self.binary_percentage(left, c, op)?;
                self.environment.assign(&id, c)?
            }
            TokenType::StarStarEqual => {
                let left = self.environment.get(&id)?;
                c = self.binary_starstar(left, c, op)?;
                self.environment.assign(&id, c)?
            }
            TokenType::AmpersandEqual => {
                let left = self.environment.get(&id)?;
                c = self.binary_ampersand(left, c, op)?;
                self.environment.assign(&id, c)?
            }
            TokenType::PipeEqual => {
                let left = self.environment.get(&id)?;
                c = self.binary_pipe(left, c, op)?;
                self.environment.assign(&id, c)?
            }
            TokenType::CaretEqual => {
                let left = self.environment.get(&id)?;
                c = self.binary_caret(left, c, op)?;
                self.environment.assign(&id, c)?
            }
            TokenType::GreaterGreaterEqual => {
                let left = self.environment.get(&id)?;
                c = self.binary_greater_greater(left, c, op)?;
                self.environment.assign(&id, c)?
            }
            TokenType::LessLessEqual => {
                let left = self.environment.get(&id)?;
                c = self.binary_less_less(left, c, op)?;
                self.environment.assign(&id, c)?
            }
            _ => return Err(()),
        };

        Ok(c)
    }
}
