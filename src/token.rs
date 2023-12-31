use std::mem::ManuallyDrop;
use std::ptr::null;

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
    Tilde,
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

// https://doc.rust-lang.org/stable/std/mem/struct.ManuallyDrop.html
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
    // pub fn to_string(&self) -> String {
    //     format!(
    //         "{:#?} {} {}",
    //         self.token_type,
    //         self.lexeme,
    //         self.content_to_string()
    //     )
    // }

    pub fn content_to_string(&self) -> String {
        unsafe {
            match self.token_type {
                TokenType::True => "true".to_string(),
                TokenType::False => "false".to_string(),
                TokenType::Integer => self.content.integer.to_string(),
                TokenType::Floating => self.content.floating.to_string(),
                TokenType::Character => self.content.character.to_string(),
                TokenType::String => self.content.string.to_string(),
                TokenType::Null => "null".to_string(),
                _ => "".to_string(),
            }
        }
    }

    pub fn clone(&self) -> Token {
        let content: Content;

        unsafe {
            content = match self.token_type {
                TokenType::True => Content { boolean: true },
                TokenType::False => Content { boolean: false },
                TokenType::Integer => Content {
                    integer: self.content.integer,
                },
                TokenType::Floating => Content {
                    floating: self.content.floating,
                },
                TokenType::Character => Content {
                    character: self.content.character,
                },
                TokenType::String => Content {
                    string: self.content.string.clone(),
                },
                TokenType::Null => Content { null: null() },
                _ => Content { null: null() },
            };
        }

        Token {
            token_type: self.token_type,
            lexeme: self.lexeme.clone(),
            content,
            line: self.line,
        }
    }
}
