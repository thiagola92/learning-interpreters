use crate::error::error;
use crate::error::UNFINISHED_CHARACTER;
use crate::error::UNFINISHED_COMMENT;
use crate::error::UNFINISHED_STRING;
use crate::error::UNKNOW_CHAR;
use crate::error::WRONG_CHAR_SIZE;
use crate::token::Content;
use crate::token::Token;
use crate::token::TokenType;
use std::collections::HashMap;
use std::mem::ManuallyDrop;

pub struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,

    keywords: HashMap<String, TokenType>,
}

impl Scanner {
    pub fn new(code: String) -> Scanner {
        Scanner {
            source: code,
            tokens: Vec::<Token>::new(),

            start: 0,
            current: 0,
            line: 1,

            keywords: HashMap::from([
                // Control Flow
                ("if".to_string(), TokenType::If),
                ("match".to_string(), TokenType::Match),
                ("loop".to_string(), TokenType::Loop),
                ("while".to_string(), TokenType::While),
                ("for".to_string(), TokenType::For),
                ("return".to_string(), TokenType::Return),
                ("pass".to_string(), TokenType::Pass),
                ("await".to_string(), TokenType::Await),
                ("else".to_string(), TokenType::Else),
                ("break".to_string(), TokenType::Break),
                ("continue".to_string(), TokenType::Continue),
                // Definition
                ("var".to_string(), TokenType::Var),
                ("const".to_string(), TokenType::Const),
                ("enum".to_string(), TokenType::Enum),
                ("signal".to_string(), TokenType::Signal),
                ("func".to_string(), TokenType::Func),
                ("proc".to_string(), TokenType::Proc),
                ("struct".to_string(), TokenType::Struct),
                ("class".to_string(), TokenType::Class),
                ("constructor".to_string(), TokenType::Constructor),
                ("destructor".to_string(), TokenType::Destructor),
                ("import".to_string(), TokenType::Import),
                ("static".to_string(), TokenType::Static),
                ("public".to_string(), TokenType::Public),
                ("extends".to_string(), TokenType::Extends),
                ("from".to_string(), TokenType::From),
                // Deisgn pattern
                ("in".to_string(), TokenType::In),
                ("when".to_string(), TokenType::When),
                // Literal
                ("true".to_string(), TokenType::True),
                ("false".to_string(), TokenType::False),
                ("null".to_string(), TokenType::Null),
                // Logical
                ("not".to_string(), TokenType::Not),
                ("and".to_string(), TokenType::And),
                ("or".to_string(), TokenType::Or),
                // Object-oriented
                ("self".to_string(), TokenType::Self_),
                ("super".to_string(), TokenType::Super),
                ("is".to_string(), TokenType::Is),
                ("as".to_string(), TokenType::As),
                // Test
                ("breakpoint".to_string(), TokenType::Breakpoint),
                ("assert".to_string(), TokenType::Assert),
                ("test".to_string(), TokenType::Test),
                // Type
                ("bool".to_string(), TokenType::Bool),
                ("int".to_string(), TokenType::Int),
                ("float".to_string(), TokenType::Float),
                ("char".to_string(), TokenType::Char),
                ("str".to_string(), TokenType::Str),
                // TODO: Classify
                ("where".to_string(), TokenType::Where),
            ]),
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_eof() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_string(),
            content: Content {
                null: std::ptr::null(),
            },
            line: self.line,
        });

        self.tokens
    }

    fn scan_token(&mut self) {
        let c: char = self.next();

        match c {
            // Assignment
            '>' if self.is_followed_by_both('>', '=') => {
                self.add_token(TokenType::GreaterGreaterEqual)
            }
            '<' if self.is_followed_by_both('<', '=') => self.add_token(TokenType::LessLessEqual),
            '*' if self.is_followed_by_both('*', '=') => self.add_token(TokenType::StarStarEqual),

            // Assignment
            '+' if self.is_followed_by('=') => self.add_token(TokenType::PlusEqual),
            '-' if self.is_followed_by('=') => self.add_token(TokenType::MinusEqual),
            '*' if self.is_followed_by('=') => self.add_token(TokenType::StarEqual),
            '/' if self.is_followed_by('=') => self.add_token(TokenType::SlashEqual),
            '%' if self.is_followed_by('=') => self.add_token(TokenType::PercentageEqual),
            '&' if self.is_followed_by('=') => self.add_token(TokenType::AmpersandEqual),
            '|' if self.is_followed_by('=') => self.add_token(TokenType::PipeEqual),
            '^' if self.is_followed_by('=') => self.add_token(TokenType::CaretEqual),

            // Bitwise
            '>' if self.is_followed_by('>') => self.add_token(TokenType::GreaterGreater),
            '<' if self.is_followed_by('<') => self.add_token(TokenType::LessLess),

            // Comparassion
            '=' if self.is_followed_by('=') => self.add_token(TokenType::EqualEqual),
            '!' if self.is_followed_by('=') => self.add_token(TokenType::NotEqual),
            '>' if self.is_followed_by('=') => self.add_token(TokenType::GreaterEqual),
            '<' if self.is_followed_by('=') => self.add_token(TokenType::LessEqual),

            // Math
            '*' if self.is_followed_by('*') => self.add_token(TokenType::StarStar),

            // TODO: Classify
            '.' if self.is_followed_by('>') => self.add_token(TokenType::PeriodPeriod),
            '-' if self.is_followed_by('>') => self.add_token(TokenType::ForwardArrow),

            // Assignment
            '=' => self.add_token(TokenType::Equal),

            // Bitwise
            '&' => self.add_token(TokenType::Ampersand),
            '|' => self.add_token(TokenType::Pipe),
            '^' => self.add_token(TokenType::Caret),

            // Design pattern
            '@' => self.add_token(TokenType::AtSign),

            // Comparassion
            '>' => self.add_token(TokenType::Greater),
            '<' => self.add_token(TokenType::Less),

            // Math
            '+' => self.add_token(TokenType::Plus),
            '-' => self.add_token(TokenType::Minus),
            '*' => self.add_token(TokenType::Star),
            '/' => self.add_token(TokenType::Slash),
            '%' => self.add_token(TokenType::Percentage),

            // Open-close
            '(' => self.add_token(TokenType::ParenthesisOpen),
            ')' => self.add_token(TokenType::ParenthesisClose),
            '[' => self.add_token(TokenType::BracketOpen),
            ']' => self.add_token(TokenType::BracketClose),
            '{' => self.add_token(TokenType::BraceOpen),
            '}' => self.add_token(TokenType::BraceClose),

            // Scope
            '\n' => self.add_newline_token(),
            '\t' => self.add_token(TokenType::Indent),

            // TODO: Classify
            '\'' => self.add_character_token(),
            '"' => self.add_string_token(),
            '#' => self.add_comment_token(),
            '$' => self.add_token(TokenType::Dollar),
            '.' => self.add_token(TokenType::Period),
            ',' => self.add_token(TokenType::Comma),
            ':' => self.add_token(TokenType::Colon),
            ';' => self.add_token(TokenType::Semicolon),
            '!' => self.add_token(TokenType::ExclamationMark),
            '?' => self.add_token(TokenType::QuestionMark),
            '_' => self.add_token(TokenType::Underscore),

            // Ignored
            ' ' => (),
            '\r' => (),

            _ => {
                if self.is_digit(c) {
                    self.add_number_token(c);
                } else if self.is_alpha(c) {
                    self.add_identifier_token(c)
                } else {
                    error(self.line, UNKNOW_CHAR)
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
        let string: String = self.consume_until('\n', UNFINISHED_COMMENT, false);

        self.tokens.push(Token {
            token_type: TokenType::Comment,
            lexeme: "".to_string(),
            content: Content {
                string: ManuallyDrop::new(string),
            },
            line: self.line,
        });
    }

    fn add_string_token(&mut self) {
        let chars: Vec<char> = vec!['"', '\n'];
        let string: String = self.consume_until_one_of(&chars, UNFINISHED_STRING, true);

        // Refuse multi-line strings.
        if self.peek() != '"' {
            error(self.line, UNFINISHED_STRING);
            return;
        }

        // Consume the closing quote.
        self.next();

        self.tokens.push(Token {
            token_type: TokenType::String,
            lexeme: "".to_string(),
            content: Content {
                string: ManuallyDrop::new(string),
            },
            line: self.line,
        });
    }

    fn add_character_token(&mut self) {
        let chars: Vec<char> = vec!['\'', '\n'];
        let string: String = self.consume_until_one_of(&chars, UNFINISHED_CHARACTER, true);

        if self.peek() != '\'' {
            error(self.line, UNFINISHED_CHARACTER);
            return;
        }

        // Consume the closing quote.
        self.next();

        if string.chars().count() != 1 {
            error(self.line, WRONG_CHAR_SIZE);
            return;
        }

        self.tokens.push(Token {
            token_type: TokenType::Character,
            lexeme: "".to_string(),
            content: Content {
                character: string.chars().nth(0).unwrap(),
            },
            line: self.line,
        });
    }

    fn add_number_token(&mut self, c: char) {
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
                token_type: TokenType::Floating,
                lexeme: "".to_string(),
                content: Content {
                    floating: string.parse::<f32>().unwrap(),
                },
                line: self.line,
            });
        } else {
            self.tokens.push(Token {
                token_type: TokenType::Integer,
                lexeme: "".to_string(),
                content: Content {
                    integer: string.parse::<i32>().unwrap(),
                },
                line: self.line,
            });
        }
    }

    fn add_identifier_token(&mut self, c: char) {
        let mut string: String = format!("{}", c);

        while self.is_alpha_numeric(self.peek()) {
            string.push(self.next())
        }

        match self.keywords.get(&string) {
            Some(&token_type) => {
                self.tokens.push(Token {
                    token_type,
                    lexeme: "".to_string(),
                    content: Content {
                        null: std::ptr::null(),
                    },
                    line: self.line,
                });
            }
            _ => {
                self.tokens.push(Token {
                    token_type: TokenType::Identifier,
                    lexeme: "".to_string(),
                    content: Content {
                        string: ManuallyDrop::new(string),
                    },
                    line: self.line,
                });
            }
        };
    }

    fn add_newline_token(&mut self) {
        self.add_token(TokenType::Newline);
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

    fn is_next_one_of(&mut self, chars: &Vec<char>) -> bool {
        match chars.into_iter().find(|&&c| self.peek() == c) {
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

    fn is_followed_by_both(&mut self, c1: char, c2: char) -> bool {
        if self.is_eof() || self.current + 1 >= self.source.len() {
            false
        } else if self.source.chars().nth(self.current).unwrap() != c1 {
            false
        } else if self.source.chars().nth(self.current + 1).unwrap() != c2 {
            false
        } else {
            self.current += 2;
            true
        }
    }

    fn next(&mut self) -> char {
        let character: char = self.source.chars().nth(self.current).unwrap();
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

    fn consume_until(&mut self, c: char, msg: &str, escape: bool) -> String {
        let mut string: String = "".to_string();

        while self.peek() != c {
            if self.is_eof() {
                error(self.line, &msg);
                break;
            }

            if self.peek() == '\n' {
                self.add_newline_token()
            };

            if escape && self.peek() == '\\' {
                string.push(self.escape());
            } else {
                string.push(self.next());
            }
        }

        string
    }

    fn consume_until_one_of(&mut self, chars: &Vec<char>, msg: &str, escape: bool) -> String {
        let mut string: String = "".to_string();

        while !self.is_next_one_of(&chars) {
            if self.is_eof() {
                error(self.line, &msg);
                break;
            }

            if self.peek() == '\n' {
                self.add_newline_token()
            };

            if escape && self.peek() == '\\' {
                string.push(self.escape());
            } else {
                string.push(self.next());
            }
        }

        string
    }

    fn escape(&mut self) -> char {
        // Current char is backslash, next one can be an special char.
        // In this case you have to skip 2 characters.
        match self.peek_next() {
            '0' => {
                self.current += 2;
                '\0'
            }
            'n' => {
                self.current += 2;
                '\n'
            }
            'r' => {
                self.current += 2;
                '\r'
            }
            't' => {
                self.current += 2;
                '\t'
            }
            '\\' => {
                self.current += 2;
                '\\'
            }
            '"' => {
                self.current += 2;
                '"'
            }
            '\'' => {
                self.current += 2;
                '\''
            }
            _ => self.next(),
        }
    }
}
