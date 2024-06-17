use super::statement::Statement;

pub fn output_tree(tree: &Vec<Statement>) -> String {
    let mut string: String = "\n".to_string();

    for s in tree {
        string.push_str(s.to_string().as_str());
        string.push('\n');
    }

    string
}
