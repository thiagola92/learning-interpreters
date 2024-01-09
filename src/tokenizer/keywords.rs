use super::token_type::TokenType;
use super::token_type::TokenType::*;
use std::collections::HashMap;

pub fn get_keyword(string: &str) -> Option<TokenType> {
    let keywords: HashMap<&str, TokenType> = HashMap::from([
        // Control Flow
        ("if", If),
        ("match", Match),
        ("loop", Loop),
        ("while", While),
        ("for", For),
        ("return", Return),
        ("pass", Pass),
        ("emit", Emit),
        ("await", Await),
        ("yield", Yield),
        ("resume", Resume),
        ("where", Where),
        // Control Flow Modifier
        ("else", Else),
        ("break", Break),
        ("continue", Continue),
        // Declaration
        ("var", Var),
        ("const", Const),
        ("enum", Enum),
        ("signal", Signal),
        ("func", Func),
        ("coro", Coro),
        ("struct", Struct),
        ("class", Class),
        ("singleton", Singleton),
        ("interface", Interface),
        ("constructor", Constructor),
        ("destructor", Destructor),
        ("set", Set),
        ("get", Get),
        ("import", Import),
        ("as", As),
        // Declaration Modifier
        ("static", Static),
        ("public", Public),
        ("extends", Extends),
        ("implements", Implements),
        ("from", From),
        // Deisgn pattern
        ("in", In),
        ("when", When),
        // Literal
        ("true", Boolean { content: true }),
        ("false", Boolean { content: false }),
        ("null", Null),
        // Logical
        ("not", Not),
        ("and", And),
        ("or", Or),
        // Object-oriented
        ("self", Self_),
        ("super", Super),
        ("is", Is),
        // Test
        ("breakpoint", Breakpoint),
        ("assert", Assert),
        ("test", Test),
        // Type
        ("bool", Bool),
        ("int", Int),
        ("float", Float),
        ("char", Char),
        ("str", Str),
        ("void", Void),
        // TODO: Classify
        ("to", To),
        ("with", With),
        ("print", Print),
    ]);

    match keywords.get(string) {
        Some(k) => Some(k.clone()),
        _ => None,
    }
}
