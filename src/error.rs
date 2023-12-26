pub static mut HAS_ERROR: bool = false;

pub fn error(line: usize, message: String) {
    report(line, "", message);
}

fn report(line: usize, whre: &str, message: String) {
    println!("[line {}] Error {}: {}", line, whre, message);

    unsafe {
        HAS_ERROR = true;
    }
}
