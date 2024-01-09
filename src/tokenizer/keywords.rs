use super::token_type::TokenType;
use super::token_type::TokenType::*;
use std::collections::HashMap;

// This is stupid but Rust doesn't support const/static HashMaps.
pub fn get_keywords() -> HashMap<String, TokenType> {
    HashMap::from([
        // Control Flow
        ("if".to_string(), If),
        ("match".to_string(), Match),
        ("loop".to_string(), Loop),
        ("while".to_string(), While),
        ("for".to_string(), For),
        ("return".to_string(), Return),
        ("pass".to_string(), Pass),
        ("emit".to_string(), Emit),
        ("await".to_string(), Await),
        ("yield".to_string(), Yield),
        ("resume".to_string(), Resume),
        ("where".to_string(), Where),
        // Control Flow Modifier
        ("else".to_string(), Else),
        ("break".to_string(), Break),
        ("continue".to_string(), Continue),
        // Declaration
        ("var".to_string(), Var),
        ("const".to_string(), Const),
        ("enum".to_string(), Enum),
        ("signal".to_string(), Signal),
        ("func".to_string(), Func),
        ("coro".to_string(), Coro),
        ("struct".to_string(), Struct),
        ("class".to_string(), Class),
        ("singleton".to_string(), Singleton),
        ("interface".to_string(), Interface),
        ("constructor".to_string(), Constructor),
        ("destructor".to_string(), Destructor),
        ("set".to_string(), Set),
        ("get".to_string(), Get),
        ("import".to_string(), Import),
        ("as".to_string(), As),
        // Declaration Modifier
        ("static".to_string(), Static),
        ("public".to_string(), Public),
        ("extends".to_string(), Extends),
        ("implements".to_string(), Implements),
        ("from".to_string(), From),
        // Deisgn pattern
        ("in".to_string(), In),
        ("when".to_string(), When),
        // Literal
        ("true".to_string(), Boolean { content: true }),
        ("false".to_string(), Boolean { content: false }),
        ("null".to_string(), Null),
        // Logical
        ("not".to_string(), Not),
        ("and".to_string(), And),
        ("or".to_string(), Or),
        // Object-oriented
        ("self".to_string(), Self_),
        ("super".to_string(), Super),
        ("is".to_string(), Is),
        // Test
        ("breakpoint".to_string(), Breakpoint),
        ("assert".to_string(), Assert),
        ("test".to_string(), Test),
        // Type
        ("bool".to_string(), Bool),
        ("int".to_string(), Int),
        ("float".to_string(), Float),
        ("char".to_string(), Char),
        ("str".to_string(), Str),
        ("void".to_string(), Void),
        // TODO: Classify
        ("to".to_string(), To),
        ("with".to_string(), With),
        ("print".to_string(), Print),
    ])
}
