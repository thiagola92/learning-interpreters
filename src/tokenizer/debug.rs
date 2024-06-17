use super::token::Token;

pub fn output_tokens(tokens: &Vec<Token>) -> String {
    let mut string: String = "\n".to_string();

    for t in tokens {
        string.push_str(t.to_string().as_str());
        string.push('\n');
    }

    string
}
