#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TokenType {
    // Assignment
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

    // Bitwise
    Ampersand,
    Pipe,
    Caret,
    ExclamationMark,
    GreaterGreater,
    LessLess,

    // Comparassion
    Greater,
    Less,
    EqualEqual,
    NotEqual,
    GreaterEqual,
    LessEqual,

    // Control Flow
    If,
    Match,
    Loop,
    While,
    For,
    Return,
    Pass,
    Emit,
    Await,
    Yield,
    Resume,

    // Control Flow Modifier
    Else,
    Break,
    Continue,

    // Definition
    Var,
    Const,
    Enum,
    Signal,
    Func,
    Coroutine,
    Struct,
    Union,
    Class,
    Singleton,
    Interface,
    Constructor,
    Destructor,
    Set,
    Get,
    Import,
    As,

    // Definition Modifier
    Static,
    Public,
    Extends,
    Implements,
    From,

    // Deisgn Pattern
    In,
    When,
    AtSign,

    // Literal
    True,
    False,
    Integer,
    Floating,
    Character,
    String_, // Escape conflict with String
    Null,

    // Logical
    Not,
    And,
    Or,

    // Math
    Plus,
    Minus,
    Star,
    Slash,
    Percentage,
    StarStar,

    // Object-Oriented
    Self_, // Escape conflict with Self
    Super,
    Is,
    To,

    // Open Close
    ParenthesisOpen,
    ParenthesisClose,
    BracketOpen,
    BracketClose,
    BraceOpen,
    BraceClose,

    // Scope
    Newline,
    Indent,

    // Test
    Breakpoint,
    Assert,
    Test,

    // Type
    Bool,
    Int,
    Float,
    Char,
    Str,

    // TODO: Classify
    Where,
    With,
    Comment,
    Dollar,
    Period,
    Comma,
    Colon,
    Semicolon,
    Underscore,
    QuestionMark,
    PeriodPeriod,
    ForwardArrow,

    // Special
    Identifier, // Name of variables, classes, functions, etc
    Eof,
}

#[derive(Debug, Clone)]
pub enum Content {
    Boolean(bool),
    Integer(i32),
    Floating(f32),
    Character(char),
    String(String),
    Null,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub content: Content,
    pub line: usize,
}

impl Content {
    pub fn to_string(&self) -> String {
        match &self {
            Content::Boolean(v) => v.to_string(),
            Content::Integer(v) => v.to_string(),
            Content::Floating(v) => v.to_string(),
            Content::Character(v) => v.to_string(),
            Content::String(v) => v.clone(),
            Content::Null => "null".to_string(),
        }
    }
}
