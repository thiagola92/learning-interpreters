// Any logic that helps the tokenizer.

pub fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

pub fn is_alpha(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
}

pub fn is_alpha_numeric(c: char) -> bool {
    is_digit(c) || is_alpha(c)
}
