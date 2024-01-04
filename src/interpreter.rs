use crate::error::runtime_error;
use crate::expression::Expression;
use crate::token::{Content, Token, TokenType};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    pub fn interpret(&self, exp: Expression) {
        match self.evaluate(exp) {
            Ok(content) => println!("{}", content.to_string()),
            Err((t, s)) => {
                println!("{}", s);
                runtime_error(t, s);
            }
        }
    }

    fn evaluate(&self, exp: Expression) -> Result<Content, (Token, String)> {
        match exp {
            Expression::Literal { content } => Ok(content),
            Expression::Grouping { exp } => self.evaluate(*exp),
            Expression::Unary { token, exp } => self.evaluate_unary(token, *exp),
            Expression::Binary { left, token, right } => self.evaluate_binary(*left, token, *right),
        }
    }

    fn evaluate_unary(&self, token: Token, exp: Expression) -> Result<Content, (Token, String)> {
        let content = self.evaluate(exp)?;

        match token.token_type {
            TokenType::Minus => match content {
                Content::Integer(i) => Ok(Content::Integer(-i)),
                Content::Floating(f) => Ok(Content::Floating(-f)),
                _ => Err((
                    token.clone(),
                    format!("{:#?} must be followed by {}", token.lexeme, "int"),
                )),
            },
            TokenType::Not => Ok(Content::Boolean(!is_true(content))),
            TokenType::ExclamationMark => match content {
                Content::Integer(i) => Ok(Content::Integer(!i)),
                _ => Err((
                    token.clone(),
                    format!("{} must be followed by {}", token.lexeme, "int"),
                )),
            },
            _ => Ok(content),
        }
    }

    fn evaluate_binary(
        &self,
        left: Expression,
        token: Token,
        right: Expression,
    ) -> Result<Content, (Token, String)> {
        let left_content = self.evaluate(left)?;
        let right_content = self.evaluate(right)?;

        match token.token_type {
            TokenType::Plus => add_contents(left_content, right_content, token),
            TokenType::Minus => sub_contents(left_content, right_content, token),
            TokenType::Ampersand => match left_content {
                Content::Integer(i1) => match right_content {
                    Content::Integer(i2) => Ok(Content::Integer(i1 & i2)),
                    _ => Err((
                        token.clone(),
                        format!("{} must be followed by {}", token.lexeme, "int"),
                    )),
                },
                _ => Err((
                    token.clone(),
                    format!("{} must be preceded by {}", token.lexeme, "int"),
                )),
            },
            TokenType::Pipe => match left_content {
                Content::Integer(i1) => match right_content {
                    Content::Integer(i2) => Ok(Content::Integer(i1 | i2)),
                    _ => Err((
                        token.clone(),
                        format!("{} must be followed by {}", token.lexeme, "int"),
                    )),
                },
                _ => Err((
                    token.clone(),
                    format!("{} must be preceded by {}", token.lexeme, "int"),
                )),
            },
            TokenType::Caret => match left_content {
                Content::Integer(i1) => match right_content {
                    Content::Integer(i2) => Ok(Content::Integer(i1 ^ i2)),
                    _ => Err((
                        token.clone(),
                        format!("{} must be followed by {}", token.lexeme, "int"),
                    )),
                },
                _ => Err((
                    token.clone(),
                    format!("{} must be preceded by {}", token.lexeme, "int"),
                )),
            },
            TokenType::Star => mul_contents(left_content, right_content, token),
            TokenType::Slash => div_contents(left_content, right_content, token),
            TokenType::Percentage => rem_contents(left_content, right_content, token),
            TokenType::StarStar => pow_contents(left_content, right_content, token),
            TokenType::GreaterGreater => match left_content {
                Content::Integer(i1) => match right_content {
                    Content::Integer(i2) => Ok(Content::Integer(i1 >> i2)),
                    _ => Err((
                        token.clone(),
                        format!("{} must be followed by {}", token.lexeme, "int"),
                    )),
                },
                _ => Err((
                    token.clone(),
                    format!("{} must be preceded by {}", token.lexeme, "int"),
                )),
            },
            TokenType::LessLess => match left_content {
                Content::Integer(i1) => match right_content {
                    Content::Integer(i2) => Ok(Content::Integer(i1 << i2)),
                    _ => Err((
                        token.clone(),
                        format!("{} must be followed by {}", token.lexeme, "int"),
                    )),
                },
                _ => Err((
                    token.clone(),
                    format!("{} must be preceded by {}", token.lexeme, "int"),
                )),
            },
            TokenType::Greater => match left_content {
                Content::Integer(i1) => match right_content {
                    Content::Integer(i2) => Ok(Content::Boolean(i1 > i2)),
                    Content::Floating(f2) => Ok(Content::Boolean((i1 as f32) > f2)),
                    _ => Err((
                        token.clone(),
                        format!("{} must be followed by {}", token.lexeme, "int/float"),
                    )),
                },
                Content::Floating(f1) => match right_content {
                    Content::Integer(i2) => Ok(Content::Boolean(f1 > (i2 as f32))),
                    Content::Floating(f2) => Ok(Content::Boolean(f1 > f2)),
                    _ => Err((
                        token.clone(),
                        format!("{} must be followed by {}", token.lexeme, "int/float"),
                    )),
                },
                Content::Character(c1) => match right_content {
                    Content::Character(c2) => Ok(Content::Boolean(c1 > c2)),
                    _ => Err((
                        token.clone(),
                        format!("{} must be followed by {}", token.lexeme, "char"),
                    )),
                },
                _ => Err((
                    token.clone(),
                    format!("{} must be preceded by {}", token.lexeme, "int/float/char"),
                )),
            },
            TokenType::Less => match left_content {
                Content::Integer(i1) => match right_content {
                    Content::Integer(i2) => Ok(Content::Boolean(i1 < i2)),
                    Content::Floating(f2) => Ok(Content::Boolean((i1 as f32) < f2)),
                    _ => Err((
                        token.clone(),
                        format!("{} must be followed by {}", token.lexeme, "int/float"),
                    )),
                },
                Content::Floating(f1) => match right_content {
                    Content::Integer(i2) => Ok(Content::Boolean(f1 < (i2 as f32))),
                    Content::Floating(f2) => Ok(Content::Boolean(f1 < f2)),
                    _ => Err((
                        token.clone(),
                        format!("{} must be followed by {}", token.lexeme, "int/float"),
                    )),
                },
                Content::Character(c1) => match right_content {
                    Content::Character(c2) => Ok(Content::Boolean(c1 < c2)),
                    _ => Err((
                        token.clone(),
                        format!("{} must be followed by {}", token.lexeme, "char"),
                    )),
                },
                _ => Err((
                    token.clone(),
                    format!("{} must be preceded by {}", token.lexeme, "int/float/char"),
                )),
            },
            TokenType::GreaterEqual => match left_content {
                Content::Integer(i1) => match right_content {
                    Content::Integer(i2) => Ok(Content::Boolean(i1 >= i2)),
                    Content::Floating(f2) => Ok(Content::Boolean((i1 as f32) >= f2)),
                    _ => Err((
                        token.clone(),
                        format!("{} must be followed by {}", token.lexeme, "int/float"),
                    )),
                },
                Content::Floating(f1) => match right_content {
                    Content::Integer(i2) => Ok(Content::Boolean(f1 >= (i2 as f32))),
                    Content::Floating(f2) => Ok(Content::Boolean(f1 >= f2)),
                    _ => Err((
                        token.clone(),
                        format!("{} must be followed by {}", token.lexeme, "int/float"),
                    )),
                },
                Content::Character(c1) => match right_content {
                    Content::Character(c2) => Ok(Content::Boolean(c1 >= c2)),
                    _ => Err((
                        token.clone(),
                        format!("{} must be followed by {}", token.lexeme, "int"),
                    )),
                },
                _ => Err((
                    token.clone(),
                    format!("{} must be preceded by {}", token.lexeme, "int/float/char"),
                )),
            },
            TokenType::LessEqual => match left_content {
                Content::Integer(i1) => match right_content {
                    Content::Integer(i2) => Ok(Content::Boolean(i1 <= i2)),
                    Content::Floating(f2) => Ok(Content::Boolean((i1 as f32) <= f2)),
                    _ => Err((
                        token.clone(),
                        format!("{} must be followed by {}", token.lexeme, "int/float"),
                    )),
                },
                Content::Floating(f1) => match right_content {
                    Content::Integer(i2) => Ok(Content::Boolean(f1 <= (i2 as f32))),
                    Content::Floating(f2) => Ok(Content::Boolean(f1 <= f2)),
                    _ => Err((
                        token.clone(),
                        format!("{} must be followed by {}", token.lexeme, "int/float"),
                    )),
                },
                Content::Character(c1) => match right_content {
                    Content::Character(c2) => Ok(Content::Boolean(c1 <= c2)),
                    _ => Err((
                        token.clone(),
                        format!("{} must be followed by {}", token.lexeme, "int"),
                    )),
                },
                _ => Err((
                    token.clone(),
                    format!("{} must be preceded by {}", token.lexeme, "int/float/char"),
                )),
            },
            TokenType::EqualEqual => Ok(Content::Boolean(is_equal(left_content, right_content))),
            TokenType::NotEqual => Ok(Content::Boolean(!is_equal(left_content, right_content))),
            _ => Err((
                token.clone(),
                format!("Token '{}' is not an operand", token.lexeme),
            )),
        }
    }
}

fn is_true(content: Content) -> bool {
    match content {
        Content::Boolean(b) => b,
        Content::Integer(i) => i != 0,
        Content::Floating(f) => f != 0.0,
        Content::String(s) => !s.is_empty(),
        _ => false,
    }
}

fn add_contents(left: Content, right: Content, token: Token) -> Result<Content, (Token, String)> {
    match left {
        Content::Integer(i1) => match right {
            Content::Integer(i2) => Ok(Content::Integer(i1 + i2)),
            Content::Floating(f2) => Ok(Content::Floating(i1 as f32 + f2)),
            _ => Err((
                token.clone(),
                format!("{} must be followed by {}", token.lexeme, "int/float"),
            )),
        },
        Content::Floating(f1) => match right {
            Content::Integer(i2) => Ok(Content::Floating(f1 + i2 as f32)),
            Content::Floating(f2) => Ok(Content::Floating(f1 + f2)),
            _ => Err((
                token.clone(),
                format!("{} must be followed by {}", token.lexeme, "int/float"),
            )),
        },
        Content::String(s1) => match right {
            Content::String(s2) => {
                let mut s3 = s1.clone();
                s3.push_str(s2.as_str());
                Ok(Content::String(s3))
            }
            _ => Err((
                token.clone(),
                format!("{} must be followed by {}", token.lexeme, "string"),
            )),
        },
        _ => Err((
            token.clone(),
            format!(
                "{} must be preceded by {}",
                token.lexeme, "int/float/string"
            ),
        )),
    }
}

fn sub_contents(left: Content, right: Content, token: Token) -> Result<Content, (Token, String)> {
    add_contents(
        left,
        match right {
            Content::Integer(i) => Ok(Content::Integer(-i)),
            Content::Floating(f) => Ok(Content::Floating(-f)),
            _ => Err((
                token.clone(),
                format!("{} must be followed by {}", token.lexeme, "int/float"),
            )),
        }?,
        token,
    )
}

fn mul_contents(left: Content, right: Content, token: Token) -> Result<Content, (Token, String)> {
    match left {
        Content::Integer(i1) => match right {
            Content::Integer(i2) => Ok(Content::Integer(i1 * i2)),
            Content::Floating(f2) => Ok(Content::Floating(i1 as f32 * f2)),
            Content::String(s2) => Ok(Content::String(s2.repeat(i1 as usize))),
            _ => Err((
                token.clone(),
                format!(
                    "{} must be followed by {}",
                    token.lexeme, "int/float/string"
                ),
            )),
        },
        Content::Floating(f1) => match right {
            Content::Integer(i2) => Ok(Content::Floating(f1 * i2 as f32)),
            Content::Floating(f2) => Ok(Content::Floating(f1 * f2)),
            _ => Err((
                token.clone(),
                format!("{} must be followed by {}", token.lexeme, "int/float"),
            )),
        },
        Content::String(s1) => match right {
            Content::Integer(i2) => Ok(Content::String(s1.repeat(i2 as usize))),
            _ => Err((
                token.clone(),
                format!("{} must be followed by {}", token.lexeme, "int"),
            )),
        },
        _ => Err((
            token.clone(),
            format!(
                "{} must be preceded by {}",
                token.lexeme, "int/float/string"
            ),
        )),
    }
}

fn div_contents(left: Content, right: Content, token: Token) -> Result<Content, (Token, String)> {
    mul_contents(
        left,
        match right {
            Content::Integer(i) => Ok(Content::Floating(1.0 / i as f32)),
            Content::Floating(f) => Ok(Content::Floating(1.0 / f)),
            _ => Err((
                token.clone(),
                format!("{} must be followed by {}", token.lexeme, "int/float"),
            )),
        }?,
        token,
    )
}

// Remainder
fn rem_contents(left: Content, right: Content, token: Token) -> Result<Content, (Token, String)> {
    match left {
        Content::Integer(i1) => match right {
            Content::Integer(i2) => Ok(Content::Integer(i1 % i2)),
            Content::Floating(f2) => Ok(Content::Floating(i1 as f32 % f2)),
            _ => Err((
                token.clone(),
                format!("{} must be followed by {}", token.lexeme, "int/float"),
            )),
        },
        Content::Floating(f1) => match right {
            Content::Integer(i2) => Ok(Content::Floating(f1 % i2 as f32)),
            Content::Floating(f2) => Ok(Content::Floating(f1 % f2)),
            _ => Err((
                token.clone(),
                format!("{} must be followed by {}", token.lexeme, "int/float"),
            )),
        },
        _ => Err((
            token.clone(),
            format!("{} must be preceded by {}", token.lexeme, "int/float"),
        )),
    }
}

fn pow_contents(left: Content, right: Content, token: Token) -> Result<Content, (Token, String)> {
    match left {
        Content::Integer(i1) => match right {
            Content::Integer(i2) => Ok(Content::Integer(i1.pow(i2 as u32))),
            Content::Floating(f2) => Ok(Content::Floating((i1 as f32).powf(f2))),
            _ => Err((
                token.clone(),
                format!("{} must be followed by {}", token.lexeme, "int/float"),
            )),
        },
        Content::Floating(f1) => match right {
            Content::Integer(i2) => Ok(Content::Floating(f1.powf(i2 as f32))),
            Content::Floating(f2) => Ok(Content::Floating(f1.powf(f2))),
            _ => Err((
                token.clone(),
                format!("{} must be followed by {}", token.lexeme, "int/float"),
            )),
        },
        _ => Err((
            token.clone(),
            format!("{} must be preceded by {}", token.lexeme, "int/float"),
        )),
    }
}

fn is_equal(left: Content, right: Content) -> bool {
    match left {
        Content::Boolean(b1) => match right {
            Content::Boolean(b2) => b1 == b2,
            _ => false,
        },
        Content::Integer(i1) => match right {
            Content::Integer(i2) => i1 == i2,
            Content::Floating(f2) => (i1 as f32) == f2,
            _ => false,
        },
        Content::Floating(f1) => match right {
            Content::Integer(i2) => f1 == (i2 as f32),
            Content::Floating(f2) => f1 == f2,
            _ => false,
        },
        Content::Character(c1) => match right {
            Content::Character(c2) => c1 == c2,
            _ => false,
        },
        Content::String(s1) => match right {
            Content::String(s2) => s1 == s2,
            _ => false,
        },
        Content::Null => match right {
            Content::Null => true,
            _ => false,
        },
    }
}
