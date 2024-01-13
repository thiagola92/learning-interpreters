use super::content::Content;

pub fn variable_undefined(name: &str) -> String {
    format!("Undefined variable '{}'", name)
}

pub fn unary_unsupported(op: &str, c: &Content) -> String {
    format!("Unsupported operator '{}' for: {}", op, c.type_to_string())
}

pub fn binary_unsupported(op: &str, c1: &Content, c2: &Content) -> String {
    format!(
        "Unsupported operator '{}' for: {} and {}",
        op,
        c1.type_to_string(),
        c2.type_to_string()
    )
}
