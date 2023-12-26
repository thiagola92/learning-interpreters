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

    // Deisgn pattern
    IN,
    WHEN,
    AT_SIGN,

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

    // Scope
    NEWLINE,
    INDENT,
    DEDENT,

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

    // TODO: Classify
    WHERE,
    COMMENT,
    DOLLAR,
    PERIOD,
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
    pub content: Content,
    pub line: usize,
}

impl Token {
    pub fn to_string(&self) -> String {
        let t = &self.token_type;
        let l = &self.lexeme;
        let c = &self.content;

        unsafe {
            match t {
                TokenType::TRUE => format!("{:#?} {} true", t, l),
                TokenType::FALSE => format!("{:#?} {} false", t, l),
                TokenType::INTEGER => format!("{:#?} {} {}", t, l, c.integer),
                TokenType::FLOATING => format!("{:#?} {} {}", t, l, c.floating),
                TokenType::CHARACTER => format!("{:#?} {} {}", t, l, c.character),
                TokenType::STRING => format!("{:#?} {} {:#?}", t, l, c.string),
                _ => format!("{:#?} {}", t, l),
            }
        }
    }
}
