use crate::token::{Token, TokenType};

pub const UNFINISHED_CHARACTER: &str = "Unfinished character.";
pub const UNFINISHED_COMMENT: &str = "Unfinished comment.";
pub const UNFINISHED_STRING: &str = "Unfinished string.";
pub const UNKNOW_CHAR: &str = "Unknow character.";
pub const WRONG_CHAR_SIZE: &str = "Single quotes should encapsulate exactly one character.";
pub const EXPECT_CLOSE_PARENTHESIS: &str = "Expect ')' after expression.";
pub const EXPECT_EXPRESSION: &str = "Expect expression.";

pub static mut HAD_ERROR: bool = false;

pub fn parser_error(token: &Token, msg: &str) {
    if token.token_type == TokenType::Eof {
        report(token.line, " at end", msg);
    } else {
        report(token.line, format!("at '{}'", token.lexeme).as_str(), msg);
    }
}

pub fn error(line: usize, msg: &str) {
    report(line, "", &msg);
}

fn report(line: usize, whre: &str, msg: &str) {
    println!("[line {}] Error {}: {}", line, whre, &msg);

    unsafe {
        HAD_ERROR = true;
    }
}
