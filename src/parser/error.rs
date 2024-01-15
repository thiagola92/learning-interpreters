pub const EXPECT_CLOSE_PARENTHESIS: &str = "Expect ')' after expression.";
pub const EXPECT_EXPRESSION: &str = "Expect expression.";
pub const EXPECT_NEWLINE: &str = "Expect newline.";
pub const EXPECT_VAR_IDENTIFIER: &str = "Expect name after 'var'.";

pub fn invalid_var_on_assignment(op: &str) -> String {
    format!("Invalid variable before '{}'", op)
}
