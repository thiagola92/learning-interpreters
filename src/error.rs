pub const UNFINISHED_CHARACTER: &str = "Unfinished character.";
pub const UNFINISHED_COMMENT: &str = "Unfinished comment.";
pub const UNFINISHED_STRING: &str = "Unfinished string.";
pub const UNKNOW_CHAR: &str = "Unknow character.";
pub const WRONG_CHAR_SIZE: &str = "Single quotes should encapsulate exactly one character.";

pub static mut HAD_ERROR: bool = false;

pub fn error(line: usize, msg: &str) {
    report(line, "", &msg);
}

fn report(line: usize, whre: &str, msg: &str) {
    println!("[line {}] Error {}: {}", line, whre, &msg);

    unsafe {
        HAD_ERROR = true;
    }
}
