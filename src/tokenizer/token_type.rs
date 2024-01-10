#[derive(Debug, Clone, PartialEq)]
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

    // Comment
    Comment { content: String },

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
    Where,

    // Control Flow Modifier
    Else,
    Break,
    Continue,

    // Declaration
    Var,
    Const,
    Enum,
    Signal,
    Func,
    Coro,
    Struct,
    Class,
    Singleton,
    Interface,
    Constructor,
    Destructor,
    Set,
    Get,
    Import,
    As,

    // Declaration Modifier
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
    Boolean { content: bool },
    Integer { content: i64 },
    Floating { content: f64 },
    Character { content: char },
    String_ { content: String }, // Escape conflict with String
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

    // Open Close
    ParenthesisOpen,
    ParenthesisClose,
    BracketOpen,
    BracketClose,
    BraceOpen,
    BraceClose,

    // Scope
    Newline,
    Indent { level: i8 },

    // Test
    Breakpoint,
    Assert,
    Test,
    Print,

    // Type
    Bool,
    Int,
    Float,
    Char,
    Str,
    Void,

    // Typecasting
    To,

    // SPECIAL
    Identifier { name: String },
    Eof,

    // RESERVED
    With,
    Dollar,
    Period,
    Comma,
    Colon,
    Semicolon,
    Underscore,
    QuestionMark,
    PeriodPeriod,
    ForwardArrow,
}
