use crate::error::error;
use crate::token::Content;
use crate::token::Token;
use crate::token::TokenType;
use std::collections::HashMap;
use std::mem::ManuallyDrop;

struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,

    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    fn new() -> Scanner {
        Scanner {
            source: "".to_string(),
            tokens: Vec::<Token>::new(),

            start: 0,
            current: 0,
            line: 1,

            keywords: HashMap::from([
                // Control Flow
                ("if".to_string(), TokenType::IF),
                ("match".to_string(), TokenType::MATCH),
                ("loop".to_string(), TokenType::LOOP),
                ("while".to_string(), TokenType::WHILE),
                ("for".to_string(), TokenType::FOR),
                ("return".to_string(), TokenType::RETURN),
                ("pass".to_string(), TokenType::PASS),
                ("await".to_string(), TokenType::AWAIT),
                ("else".to_string(), TokenType::ELSE),
                ("break".to_string(), TokenType::BREAK),
                ("continue".to_string(), TokenType::CONTINUE),
                // Definition
                ("var".to_string(), TokenType::VAR),
                ("const".to_string(), TokenType::CONST),
                ("enum".to_string(), TokenType::ENUM),
                ("signal".to_string(), TokenType::SIGNAL),
                ("func".to_string(), TokenType::FUNC),
                ("proc".to_string(), TokenType::PROC),
                ("struct".to_string(), TokenType::STRUCT),
                ("class".to_string(), TokenType::CLASS),
                ("constructor".to_string(), TokenType::CONSTRUCTOR),
                ("destructor".to_string(), TokenType::DESTRUCTOR),
                ("import".to_string(), TokenType::IMPORT),
                ("static".to_string(), TokenType::STATIC),
                ("public".to_string(), TokenType::PUBLIC),
                ("extends".to_string(), TokenType::EXTENDS),
                ("from".to_string(), TokenType::FROM),
                // Deisgn pattern
                ("in".to_string(), TokenType::IN),
                ("when".to_string(), TokenType::WHEN),
                // Literal
                ("true".to_string(), TokenType::TRUE),
                ("false".to_string(), TokenType::FALSE),
                ("null".to_string(), TokenType::NULL),
                // Logical
                ("not".to_string(), TokenType::NOT),
                ("and".to_string(), TokenType::AND),
                ("or".to_string(), TokenType::OR),
                // Object-oriented
                ("self".to_string(), TokenType::SELF),
                ("super".to_string(), TokenType::SUPER),
                ("is".to_string(), TokenType::IS),
                ("as".to_string(), TokenType::AS),
                // Test
                ("breakpoint".to_string(), TokenType::BREAKPOINT),
                ("assert".to_string(), TokenType::ASSERT),
                ("test".to_string(), TokenType::TEST),
                // Type
                ("bool".to_string(), TokenType::BOOL),
                ("int".to_string(), TokenType::INT),
                ("float".to_string(), TokenType::FLOAT),
                ("char".to_string(), TokenType::CHAR),
                ("str".to_string(), TokenType::STR),
                // TODO: Classify
                ("where".to_string(), TokenType::WHERE),
            ]),
        }
    }

    fn scan_tokens(&mut self) {
        while !self.is_eof() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            content: Content {
                null: std::ptr::null(),
            },
            line: self.line,
        });
    }

    fn scan_token(&mut self) {
        let c: char = self.next();

        match c {
            // Assignment
            '+' if self.is_followed_by('=') => self.add_token(TokenType::PLUS_EQUAL),
            '-' if self.is_followed_by('=') => self.add_token(TokenType::MINUS_EQUAL),
            '*' if self.is_followed_by('=') => self.add_token(TokenType::STAR_EQUAL),
            '/' if self.is_followed_by('=') => self.add_token(TokenType::SLASH_EQUAL),
            '%' if self.is_followed_by('=') => self.add_token(TokenType::PERCENTAGE_EQUAL),

            // Bitwise
            '>' if self.is_followed_by('>') => self.add_token(TokenType::GREATER_GREATER),
            '<' if self.is_followed_by('<') => self.add_token(TokenType::LESS_LESS),

            // Comparassion
            '=' if self.is_followed_by('=') => self.add_token(TokenType::EQUAL_EQUAL),
            '!' if self.is_followed_by('=') => self.add_token(TokenType::NOT_EQUAL),
            '>' if self.is_followed_by('=') => self.add_token(TokenType::GREATER_EQUAL),
            '<' if self.is_followed_by('=') => self.add_token(TokenType::LESS_EQUAL),

            // Math
            '*' if self.is_followed_by('*') => self.add_token(TokenType::STAR_STAR),

            // TODO: Classify
            '.' if self.is_followed_by('>') => self.add_token(TokenType::PERIOD_PERIOD),
            '-' if self.is_followed_by('>') => self.add_token(TokenType::FORWARD_ARROW),

            // Assignment
            '=' => self.add_token(TokenType::EQUAL),

            // Bitwise
            '&' => self.add_token(TokenType::AMPERSAND),
            '|' => self.add_token(TokenType::PIPE),
            '^' => self.add_token(TokenType::CARET),

            // Design pattern
            '@' => self.add_token(TokenType::AT_SIGN),

            // Comparassion
            '>' => self.add_token(TokenType::GREATER),
            '<' => self.add_token(TokenType::LESS),

            // Math
            '+' => self.add_token(TokenType::PLUS),
            '-' => self.add_token(TokenType::MINUS),
            '*' => self.add_token(TokenType::STAR),
            '/' => self.add_token(TokenType::SLASH),
            '%' => self.add_token(TokenType::PERCENTAGE),

            // Open-close
            '(' => self.add_token(TokenType::PARENTHESIS_OPEN),
            ')' => self.add_token(TokenType::PARENTHESIS_CLOSE),
            '[' => self.add_token(TokenType::BRACKET_OPEN),
            ']' => self.add_token(TokenType::BRACKET_CLOSE),
            '{' => self.add_token(TokenType::BRACE_OPEN),
            '}' => self.add_token(TokenType::BRACE_CLOSE),

            // Scope
            '\n' => self.add_newline_token(),
            '\t' => self.add_token(TokenType::INDENT),

            // TODO: Classify
            '"' => self.add_string_token(),
            '#' => self.add_comment_token(),
            '$' => self.add_token(TokenType::DOLLAR),
            '.' => self.add_token(TokenType::PERIOD),
            ',' => self.add_token(TokenType::COMMA),
            ':' => self.add_token(TokenType::COLON),
            ';' => self.add_token(TokenType::SEMICOLON),
            '!' => self.add_token(TokenType::EXCLAMATION_MARK),
            '?' => self.add_token(TokenType::QUESTION_MARK),
            '_' => self.add_token(TokenType::UNDERSCORE),

            // Ignored
            ' ' => (),
            '\r' => (),

            _ => {
                if self.is_digit(c) {
                    self.add_number_token(c);
                } else if self.is_alpha(c) {
                    self.add_identifier_token(c)
                } else {
                    error(self.line, "Unexpected character.".to_string())
                }
            }
        };
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token {
            token_type,
            lexeme: "".to_string(),
            content: Content {
                null: std::ptr::null(),
            },
            line: self.line,
        });
    }

    fn add_comment_token(&mut self) {
        let string: String = self.consume_until('\n');

        self.tokens.push(Token {
            token_type: TokenType::COMMENT,
            lexeme: "".to_string(),
            content: Content {
                string: ManuallyDrop::new(string),
            },
            line: self.line,
        });
    }

    fn add_string_token(&mut self) {
        let chars: Vec<char> = vec!['"', '\n'];
        let mut string: String = self.consume_until_one_of(&chars);
        let mut last_char: char = string.chars().last().unwrap();

        loop {
            // Refuse multi-line strings.
            if last_char != '"' {
                error(self.line, "Unterminated string.".to_string())
            }

            // User is escaping quote.
            if last_char == '\\' {
                string.push(self.next());
                string.push_str(self.consume_until_one_of(&chars).as_str());
                last_char = string.chars().last().unwrap();

                continue;
            }

            break;
        }
        self.next(); // Consume the closing quote.
        self.tokens.push(Token {
            token_type: TokenType::STRING,
            lexeme: "".to_string(),
            content: Content {
                string: ManuallyDrop::new(string),
            },
            line: self.line,
        });
    }

    fn add_number_token(&mut self, mut c: char) {
        let mut string: String = format!("{}", c);

        while self.is_digit(self.peek()) {
            string.push(self.next());
        }

        // Floating will have a dot with numbers after it.
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            string.push(self.next());

            while self.is_digit(self.peek()) {
                string.push(self.next());
            }

            self.tokens.push(Token {
                token_type: TokenType::FLOAT,
                lexeme: "".to_string(),
                content: Content {
                    floating: string.parse::<f32>().unwrap(),
                },
                line: self.line,
            });
        } else {
            self.tokens.push(Token {
                token_type: TokenType::INTEGER,
                lexeme: "".to_string(),
                content: Content {
                    integer: string.parse::<i32>().unwrap(),
                },
                line: self.line,
            });
        }
    }

    fn add_identifier_token(&mut self, mut c: char) {
        let mut string: String = format!("{}", c);

        while self.is_alpha_numeric(self.peek()) {
            string.push(self.next())
        }
    }

    fn add_newline_token(&mut self) {
        self.add_token(TokenType::NEWLINE);
        self.line += 1;
    }

    /*********************************************/
    /****************** Utility ******************/
    /*********************************************/

    fn is_eof(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_digit(c) || self.is_alpha(c)
    }

    fn is_next(&self, c: char) -> bool {
        !self.is_eof() && self.source.chars().nth(self.current).unwrap() == c
    }

    fn is_next_one_of(&mut self, chars: &Vec<char>) -> bool {
        match chars.into_iter().find(|&&c| self.is_next(c)) {
            Some(_) => true,
            _ => false,
        }
    }

    fn is_followed_by(&mut self, c: char) -> bool {
        if self.is_eof() {
            false
        } else if self.source.chars().nth(self.current).unwrap() != c {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn next(&mut self) -> char {
        let character = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        character
    }

    fn peek(&self) -> char {
        if self.is_eof() {
            '\0'
        } else {
            self.source.chars().nth(self.current).unwrap()
        }
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            '\0'
        } else {
            self.source.chars().nth(self.current + 1).unwrap()
        }
    }

    fn consume_until(&mut self, c: char) -> String {
        let mut string: String = "".to_string();

        while !self.is_next(c) && !self.is_eof() {
            if self.is_next('\n') {
                self.add_newline_token()
            };

            string.push(self.next());
        }

        if self.is_eof() {
            error(self.line, "Unterminated string.".to_string())
        }

        string
    }

    fn consume_until_one_of(&mut self, chars: &Vec<char>) -> String {
        let mut string: String = "".to_string();

        while !self.is_next_one_of(&chars) && !self.is_eof() {
            if self.is_next('\n') {
                self.add_newline_token()
            };

            string.push(self.next());
        }

        if self.is_eof() {
            error(self.line, "Unterminated string.".to_string())
        }

        string
    }
}
