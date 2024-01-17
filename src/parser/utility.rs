use crate::tokenizer::token_type::TokenType;
use crate::tokenizer::token_type::TokenType::*;

pub const IDENTIFIER: TokenType = Identifier(String::new());

pub const INDENT: TokenType = Indent(0);

pub const ASSIGNMENTS: [TokenType; 12] = [
    Equal,
    PlusEqual,
    MinusEqual,
    StarEqual,
    SlashEqual,
    PercentageEqual,
    StarStarEqual,
    AmpersandEqual,
    PipeEqual,
    CaretEqual,
    GreaterGreaterEqual,
    LessLessEqual,
];

pub const EQUALITIES: [TokenType; 2] = [EqualEqual, NotEqual];

pub const COMPARASIONS: [TokenType; 4] = [Greater, Less, GreaterEqual, LessEqual];

pub const TERMS: [TokenType; 5] = [Plus, Minus, Ampersand, Pipe, Caret];

pub const FACTORIZATIONS: [TokenType; 6] =
    [Star, Slash, Percentage, StarStar, GreaterGreater, LessLess];

pub const UNARIES: [TokenType; 3] = [Minus, Not, ExclamationMark];

pub const LITERALS: [TokenType; 6] = [
    Boolean(false),
    Integer(0),
    Floating(0.0),
    Character('\0'),
    String_(String::new()),
    Null,
];

pub fn is_statement(token_type: &TokenType) -> bool {
    match token_type {
        Var | Const | Enum | Signal | Func | Coro | Struct | Class | Singleton | Interface
        | Constructor | Destructor | Import | Static | Public | When | AtSign | Breakpoint
        | Assert | Test | Print => true,
        _ => false,
    }
}
