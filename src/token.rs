use std::mem::ManuallyDrop;

#[derive(Debug)]
pub enum TokenType {
    // Assignment
    EQUAL,
    PLUS_EQUAL,
    MINUS_EQUAL,
    STAR_EQUAL,
    SLASH_EQUAL,
    PERCENTAGE_EQUAL,
    STAR_STAR_EQUAL,
    AMPERSAND_EQUAL,
    PIPE_EQUAL,
    CARET_EQUAL,
    GREATER_GREATER_EQUAL,
    LESS_LESS_EQUAL,

    // Bitwise
    AMPERSAND,
    PIPE,
    CARET,
    GREATER_GREATER,
    LESS_LESS,

    // Comparassion
    GREATER,
    LESS,
    EQUAL_EQUAL,
    NOT_EQUAL,
    GREATER_EQUAL,
    LESS_EQUAL,

    // Control Flow
    IF,
    MATCH,
    LOOP,
    WHILE,
    FOR,
    RETURN,
    PASS,
    AWAIT,
    ELSE_IF,
    ELSE,
    BREAK,
    CONTINUE,

    // Definition
    VAR,
    CONST,
    ENUM,
    SIGNAL,
    FUNC,
    PROC,
    STRUCT,
    CLASS,
    CONSTRUCTOR,
    DESTRUCTOR,
    IMPORT,
    STATIC,
    PUBLIC,
    EXTENDS,
    FROM,

    // Literal
    TRUE,
    FALSE,
    INTEGER,
    FLOATING,
    CHARACTER,
    STRING,
    NULL,

    // Logical
    NOT,
    AND,
    OR,

    // Math
    PLUS,
    MINUS,
    STAR,
    SLASH,
    PERCENTAGE,
    STAR_STAR,

    // Object-oriented
    SELF,
    SUPER,
    IS,
    AS,

    // Open close
    PARENTHESIS_OPEN,
    PARENTHESIS_CLOSE,
    BRACKET_OPEN,
    BRACKET_CLOSE,
    BRACE_OPEN,
    BRACE_CLOSE,

    // Test
    BREAKPOINT,
    ASSERT,
    TEST,

    // Type
    BOOL,
    INT,
    FLOAT,
    CHAR,
    STR,

    // Deisgn pattern
    IN,
    WHEN,
    AT_SIGN,

    ////////////////
    WHERE,
    COMMENT,
    DOLLAR,
    PERIODO,
    COMMA,
    COLON,
    SEMICOLON,
    EXCLAMATION_MARK,
    QUESTION_MARK,
    UNDERSCORE,
    PERIOD_PERIOD,
    FORWARD_ARROW,

    // Special
    IDENTIFIER, // Name of variables, classes, functions, etc
    NEWLINE,
    INDENT,
    DEDENT,
    EOF,
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
    pub literal: Content,
    pub line: usize,
}

impl Token {
    pub fn to_string(&self) -> String {
        let token_type = &self.token_type;
        let lexeme = &self.lexeme;
        let literal = &self.literal;

        match token_type {
            TRUE => format!("{:#?} {} true", token_type, lexeme),
            FALSE => format!("{:#?} {} false", token_type, lexeme),
            INTEGER => format!("{:#?} {} {}", token_type, lexeme, literal.integer),
            FLOATING => format!("{:#?} {} {}", token_type, lexeme, literal.floating),
            CHARACTER => format!("{:#?} {} {}", token_type, lexeme, literal.character),
            STRING => format!("{:#?} {} {:#?}", token_type, lexeme, literal.string),
            _ => format!("{:#?} {}", token_type, lexeme),
        }
    }
}
