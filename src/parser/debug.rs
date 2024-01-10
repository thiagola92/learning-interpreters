use super::expression::Expression;

pub fn output_tree(tree: Vec<Expression>) -> String {
    let mut string: String = "\n".to_string();

    // Rename 'e' with generic value change.
    for e in tree {
        string.push_str(e.to_string().as_str());
        string.push('\n');
    }

    string
}
