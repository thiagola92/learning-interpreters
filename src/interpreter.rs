use crate::expression::Expression;
use crate::token::{Content, Token, TokenType};

pub struct Interpreter {}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {}
    }

    // Change to Result<Content, ()>
    pub fn evaluate(&self, exp: Expression) -> Content {
        match exp {
            Expression::Literal { content } => content,
            Expression::Grouping { exp } => self.evaluate(*exp),
            Expression::Unary { token, exp } => self.evaluate_unary(token, *exp),
            Expression::Binary { left, token, right } => self.evaluate_binary(*left, token, *right),
        }
    }

    fn evaluate_unary(&self, token: Token, exp: Expression) -> Content {
        let content = self.evaluate(exp);

        match token.token_type {
            TokenType::Minus => match content {
                Content::Integer(i) => Content::Integer(-i),
                Content::Floating(f) => Content::Floating(-f),
                _ => Content::Null,
            },
            TokenType::Not => Content::Boolean(!is_true(content)),
            TokenType::ExclamationMark => match content {
                Content::Integer(i) => Content::Integer(!i),
                _ => Content::Null,
            },
            _ => content,
        }
    }

    fn evaluate_binary(&self, left: Expression, token: Token, right: Expression) -> Content {
        let left_content = self.evaluate(left);
        let right_content = self.evaluate(right);

        match token.token_type {
            TokenType::Plus => add_contents(left_content, right_content),
            TokenType::Minus => sub_contents(left_content, right_content),
            TokenType::Ampersand => match left_content {
                Content::Integer(i1) => match right_content {
                    Content::Integer(i2) => Content::Integer(i1 & i2),
                    _ => Content::Null,
                },
                _ => Content::Null,
            },
            TokenType::Pipe => match left_content {
                Content::Integer(i1) => match right_content {
                    Content::Integer(i2) => Content::Integer(i1 | i2),
                    _ => Content::Null,
                },
                _ => Content::Null,
            },
            TokenType::Caret => match left_content {
                Content::Integer(i1) => match right_content {
                    Content::Integer(i2) => Content::Integer(i1 ^ i2),
                    _ => Content::Null,
                },
                _ => Content::Null,
            },
            TokenType::Star => mul_contents(left_content, right_content),
            TokenType::Slash => div_contents(left_content, right_content),
            TokenType::Percentage => rem_contents(left_content, right_content),
            TokenType::StarStar => pow_contents(left_content, right_content),
            TokenType::GreaterGreater => match left_content {
                Content::Integer(i1) => match right_content {
                    Content::Integer(i2) => Content::Integer(i1 >> i2),
                    _ => Content::Null,
                },
                _ => Content::Null,
            },
            TokenType::LessLess => match left_content {
                Content::Integer(i1) => match right_content {
                    Content::Integer(i2) => Content::Integer(i1 << i2),
                    _ => Content::Null,
                },
                _ => Content::Null,
            },
            TokenType::Greater => match left_content {
                Content::Integer(i1) => match right_content {
                    Content::Integer(i2) => Content::Boolean(i1 > i2),
                    Content::Floating(f2) => Content::Boolean((i1 as f32) > f2),
                    _ => Content::Null,
                },
                Content::Floating(f1) => match right_content {
                    Content::Integer(i2) => Content::Boolean(f1 > (i2 as f32)),
                    Content::Floating(f2) => Content::Boolean(f1 > f2),
                    _ => Content::Null,
                },
                Content::Character(c1) => match right_content {
                    Content::Character(c2) => Content::Boolean(c1 > c2),
                    _ => Content::Null,
                },
                _ => Content::Null,
            },
            TokenType::Less => match left_content {
                Content::Integer(i1) => match right_content {
                    Content::Integer(i2) => Content::Boolean(i1 < i2),
                    Content::Floating(f2) => Content::Boolean((i1 as f32) < f2),
                    _ => Content::Null,
                },
                Content::Floating(f1) => match right_content {
                    Content::Integer(i2) => Content::Boolean(f1 < (i2 as f32)),
                    Content::Floating(f2) => Content::Boolean(f1 < f2),
                    _ => Content::Null,
                },
                Content::Character(c1) => match right_content {
                    Content::Character(c2) => Content::Boolean(c1 < c2),
                    _ => Content::Null,
                },
                _ => Content::Null,
            },
            TokenType::GreaterEqual => match left_content {
                Content::Integer(i1) => match right_content {
                    Content::Integer(i2) => Content::Boolean(i1 >= i2),
                    Content::Floating(f2) => Content::Boolean((i1 as f32) >= f2),
                    _ => Content::Null,
                },
                Content::Floating(f1) => match right_content {
                    Content::Integer(i2) => Content::Boolean(f1 >= (i2 as f32)),
                    Content::Floating(f2) => Content::Boolean(f1 >= f2),
                    _ => Content::Null,
                },
                Content::Character(c1) => match right_content {
                    Content::Character(c2) => Content::Boolean(c1 >= c2),
                    _ => Content::Null,
                },
                _ => Content::Null,
            },
            TokenType::LessEqual => match left_content {
                Content::Integer(i1) => match right_content {
                    Content::Integer(i2) => Content::Boolean(i1 <= i2),
                    Content::Floating(f2) => Content::Boolean((i1 as f32) <= f2),
                    _ => Content::Null,
                },
                Content::Floating(f1) => match right_content {
                    Content::Integer(i2) => Content::Boolean(f1 <= (i2 as f32)),
                    Content::Floating(f2) => Content::Boolean(f1 <= f2),
                    _ => Content::Null,
                },
                Content::Character(c1) => match right_content {
                    Content::Character(c2) => Content::Boolean(c1 <= c2),
                    _ => Content::Null,
                },
                _ => Content::Null,
            },
            TokenType::EqualEqual => Content::Boolean(is_equal(left_content, right_content)),
            TokenType::NotEqual => Content::Boolean(!is_equal(left_content, right_content)),
            _ => Content::Null,
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

fn add_contents(left: Content, right: Content) -> Content {
    match left {
        Content::Integer(i1) => match right {
            Content::Integer(i2) => Content::Integer(i1 + i2),
            Content::Floating(f2) => Content::Floating(i1 as f32 + f2),
            _ => Content::Null,
        },
        Content::Floating(f1) => match right {
            Content::Integer(i2) => Content::Floating(f1 + i2 as f32),
            Content::Floating(f2) => Content::Floating(f1 + f2),
            _ => Content::Null,
        },
        Content::String(s1) => match right {
            Content::String(s2) => {
                let mut s3 = s1.clone();
                s3.push_str(s2.as_str());
                Content::String(s3)
            }
            _ => Content::Null,
        },
        _ => Content::Null,
    }
}

fn sub_contents(left: Content, right: Content) -> Content {
    add_contents(
        left,
        match right {
            Content::Integer(i) => Content::Integer(-i),
            Content::Floating(f) => Content::Floating(-f),
            _ => Content::Null,
        },
    )
}

fn mul_contents(left: Content, right: Content) -> Content {
    match left {
        Content::Integer(i1) => match right {
            Content::Integer(i2) => Content::Integer(i1 * i2),
            Content::Floating(f2) => Content::Floating(i1 as f32 * f2),
            Content::String(s2) => Content::String(s2.repeat(i1 as usize)),
            _ => Content::Null,
        },
        Content::Floating(f1) => match right {
            Content::Integer(i2) => Content::Floating(f1 * i2 as f32),
            Content::Floating(f2) => Content::Floating(f1 * f2),
            _ => Content::Null,
        },
        Content::String(s1) => match right {
            Content::Integer(i2) => Content::String(s1.repeat(i2 as usize)),
            _ => Content::Null,
        },
        _ => Content::Null,
    }
}

fn div_contents(left: Content, right: Content) -> Content {
    mul_contents(
        left,
        match right {
            Content::Integer(i) => Content::Floating(1.0 / i as f32),
            Content::Floating(f) => Content::Floating(1.0 / f),
            _ => Content::Null,
        },
    )
}

// Remainder
fn rem_contents(left: Content, right: Content) -> Content {
    match left {
        Content::Integer(i1) => match right {
            Content::Integer(i2) => Content::Integer(i1 % i2),
            Content::Floating(f2) => Content::Floating(i1 as f32 % f2),
            _ => Content::Null,
        },
        Content::Floating(f1) => match right {
            Content::Integer(i2) => Content::Floating(f1 % i2 as f32),
            Content::Floating(f2) => Content::Floating(f1 % f2),
            _ => Content::Null,
        },
        _ => Content::Null,
    }
}

fn pow_contents(left: Content, right: Content) -> Content {
    match left {
        Content::Integer(i1) => match right {
            Content::Integer(i2) => Content::Integer(i1.pow(i2 as u32)),
            Content::Floating(f2) => Content::Floating((i1 as f32).powf(f2)),
            _ => Content::Null,
        },
        Content::Floating(f1) => match right {
            Content::Integer(i2) => Content::Floating(f1.powf(i2 as f32)),
            Content::Floating(f2) => Content::Floating(f1.powf(f2)),
            _ => Content::Null,
        },
        _ => Content::Null,
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
