use super::content::Content;

pub fn is_true(content: &Content) -> bool {
    match content {
        Content::Boolean(b) => *b,
        Content::Integer(i) => *i != 0,
        Content::Floating(f) => *f != 0.0,
        Content::Character(c) => *c != '\0',
        Content::String_(s) => s.len() != 0,
        Content::Null => false,
    }
}

pub fn concat_strings(s1: &String, s2: &String) -> String {
    let mut s3 = s1.clone();
    s3.push_str(s2.as_str());
    s3
}
