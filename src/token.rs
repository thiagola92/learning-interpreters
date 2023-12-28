use std::mem::ManuallyDrop;

#[derive(Debug, Clone, Copy)]
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
    Await,
    Emit,

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
    Proc,
    Struct,
    Union,
    Class,
    Constructor,
    Destructor,
    Interface,
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
    String,
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
    Self_,
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
    ExclamationMark,
    QuestionMark,
    PeriodPeriod,
    ForwardArrow,

    // Special
    Identifier, // Name of variables, classes, functions, etc
    Eof,
}

pub union Content {
    pub boolean: bool,
    pub integer: i32,
    pub floating: f32,
    pub character: char,
    pub string: ManuallyDrop<String>,
    pub null: *const i32,
}

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub content: Content,
    pub line: usize,
}

impl Token {
    pub fn to_string(&self) -> String {
        let t = self.token_type;
        let l = &self.lexeme;
        let c = &self.content;

        unsafe {
            match t {
                TokenType::True => format!("{:#?} {}", t, l),
                TokenType::False => format!("{:#?} {}", t, l),
                TokenType::Integer => format!("{:#?} {} {}", t, l, c.integer),
                TokenType::Floating => format!("{:#?} {} {}", t, l, c.floating),
                TokenType::Character => format!("{:#?} {} {}", t, l, c.character),
                TokenType::String => format!("{:#?} {} {:#?}", t, l, c.string),
                _ => format!("{:#?} {}", t, l),
            }
        }
    }
}
