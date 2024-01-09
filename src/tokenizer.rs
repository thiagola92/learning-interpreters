pub mod debug;
pub mod error;
pub mod keywords;
pub mod token;
pub mod token_type;
pub mod utility;

use super::error::tokenizer_error;
use error::*;
use keywords::get_keyword;
use token::Token;
use token_type::TokenType;
use token_type::TokenType::*;
use utility::*;

pub struct Tokenizer {
    source: String,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,
}

impl Tokenizer {
    pub fn new(code: String) -> Tokenizer {
        Tokenizer {
            source: code,
            tokens: Vec::<Token>::new(),

            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn tokenize(mut self) -> Vec<Token> {
        self.tokens.push(Token {
            token_type: Indent { level: 0 },
            lexeme: "".to_string(),
            line: self.line,
        });

        while !self.is_nth_eof(0) {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: Eof,
            lexeme: "".to_string(),
            line: self.line,
        });

        self.tokens
    }

    pub fn scan_token(&mut self) {
        let c: char = self.advance_n(1);

        match c {
            // Assignment
            '>' if self.is_followed_by(">=") => self.add_token(GreaterGreaterEqual, ">>="),
            '<' if self.is_followed_by("<=") => self.add_token(LessLessEqual, "<<="),
            '*' if self.is_followed_by("*=") => self.add_token(StarStarEqual, "**="),

            // Assignment
            '+' if self.is_followed_by("=") => self.add_token(PlusEqual, "+="),
            '-' if self.is_followed_by("=") => self.add_token(MinusEqual, "-="),
            '*' if self.is_followed_by("=") => self.add_token(StarEqual, "*="),
            '/' if self.is_followed_by("=") => self.add_token(SlashEqual, "/="),
            '%' if self.is_followed_by("=") => self.add_token(PercentageEqual, "%="),
            '&' if self.is_followed_by("=") => self.add_token(AmpersandEqual, "&="),
            '|' if self.is_followed_by("=") => self.add_token(PipeEqual, "|="),
            '^' if self.is_followed_by("=") => self.add_token(CaretEqual, "^="),

            // Bitwise
            '>' if self.is_followed_by(">") => self.add_token(GreaterGreater, ">>"),
            '<' if self.is_followed_by("<") => self.add_token(LessLess, "<<"),

            // Comparassion
            '=' if self.is_followed_by("=") => self.add_token(EqualEqual, "=="),
            '!' if self.is_followed_by("=") => self.add_token(NotEqual, "!="),
            '>' if self.is_followed_by("=") => self.add_token(GreaterEqual, ">="),
            '<' if self.is_followed_by("=") => self.add_token(LessEqual, "<="),

            // Math
            '*' if self.is_followed_by("*") => self.add_token(StarStar, "**"),

            // TODO: Classify
            '.' if self.is_followed_by(".") => self.add_token(PeriodPeriod, ".."),
            '-' if self.is_followed_by(">") => self.add_token(ForwardArrow, "->"),

            // Assignment
            '=' => self.add_token(Equal, "="),

            // Bitwise
            '&' => self.add_token(Ampersand, "&"),
            '|' => self.add_token(Pipe, "|"),
            '^' => self.add_token(Caret, "^"),
            '!' => self.add_token(ExclamationMark, "!"),

            // Design pattern
            '@' => self.add_token(AtSign, "@"),

            // Comparassion
            '>' => self.add_token(Greater, ">"),
            '<' => self.add_token(Less, "<"),

            // Math
            '+' => self.add_token(Plus, "+"),
            '-' => self.add_token(Minus, "-"),
            '*' => self.add_token(Star, "*"),
            '/' => self.add_token(Slash, "/"),
            '%' => self.add_token(Percentage, "%"),

            // Open-close
            '(' => self.add_token(ParenthesisOpen, "("),
            ')' => self.add_token(ParenthesisClose, ")"),
            '[' => self.add_token(BracketOpen, "["),
            ']' => self.add_token(BracketClose, "]"),
            '{' => self.add_token(BraceOpen, "{"),
            '}' => self.add_token(BraceClose, "}"),

            // Scope
            '\n' => self.add_newline_token(),
            '\t' => self.add_indent_token(),

            // TODO: Classify
            '\'' => self.add_character_token(),
            '"' => self.add_string_token(),
            '#' => self.add_comment_token(),
            '$' => self.add_token(Dollar, "$"),
            '.' => self.add_token(Period, "."),
            ',' => self.add_token(Comma, ","),
            ':' => self.add_token(Colon, ":"),
            ';' => self.add_token(Semicolon, ";"),
            '?' => self.add_token(QuestionMark, "?"),
            '_' => self.add_token(Underscore, "_"),

            // Ignored
            ' ' => (),
            '\r' => (),

            _ => {
                if is_digit(c) {
                    self.add_number_token(c);
                } else if is_alpha(c) {
                    self.add_identifier_token(c)
                } else {
                    return tokenizer_error(self.line, INVALID_CHAR.to_string());
                }
            }
        }
    }

    fn add_token(&mut self, token_type: TokenType, lexeme: &str) {
        self.tokens.push(Token {
            token_type,
            lexeme: lexeme.to_string(),
            line: self.line,
        });
    }

    fn add_newline_token(&mut self) {
        self.add_token(Newline, "\n");
        self.add_token(Indent { level: 0 }, "");
        self.line += 1;
    }

    fn add_indent_token(&mut self) {
        if self.tokens.is_empty() {
            return self.add_token(Indent { level: 1 }, "\t");
        }

        let last: usize = self.tokens.len() - 1;
        let previous: Token = self.tokens[last].clone();

        match previous.token_type {
            Indent { level } => {
                self.tokens[last] = Token {
                    token_type: Indent { level: level + 1 },
                    lexeme: format!("{}\t", previous.lexeme),
                    line: previous.line,
                };
            }
            Newline => self.add_token(Indent { level: 1 }, "\t"),
            _ => (),
        }
    }

    fn add_comment_token(&mut self) {
        match self.advance_until_one_of("\n\0", false) {
            Ok(string) => {
                self.tokens.push(Token {
                    token_type: Comment {
                        content: string.clone(),
                    },
                    lexeme: format!("#{}", string),
                    line: self.line,
                });
            }
            _ => (),
        }
    }

    fn add_string_token(&mut self) {
        // Strings shouldn't leave the line and should end with double quotation mark.
        match self.advance_until_one_of("\n\"", true) {
            Ok(string) => {
                if self.peek_nth(0) != '"' {
                    return tokenizer_error(self.line, UNFINISHED_STRING.to_string());
                }

                // Consume quotation.
                self.advance_n(1);

                self.tokens.push(Token {
                    token_type: String_ {
                        content: string.clone(),
                    },
                    lexeme: format!("\"{}\"", string),
                    line: self.line,
                })
            }
            _ => tokenizer_error(self.line, UNFINISHED_STRING.to_string()),
        }
    }

    fn add_character_token(&mut self) {
        // Characters shouldn't leave the line and should end with single quotation mark.
        match self.advance_until_one_of("\n'", true) {
            Ok(string) => {
                if self.peek_nth(0) != '\'' {
                    return tokenizer_error(self.line, UNFINISHED_CHARACTER.to_string());
                }

                // Consume quotation.
                self.advance_n(1);

                if string.chars().count() != 1 {
                    return tokenizer_error(self.line, WRONG_CHAR_SIZE.to_string());
                }

                let character: char = string.chars().nth(0).unwrap();

                self.tokens.push(Token {
                    token_type: Character { content: character },
                    lexeme: format!("'{}'", character),
                    line: self.line,
                })
            }
            _ => tokenizer_error(self.line, UNFINISHED_CHARACTER.to_string()),
        }
    }

    fn add_number_token(&mut self, c: char) {
        let mut string: String = format!("{}", c);

        while is_digit(self.peek_nth(0)) {
            string.push(self.advance_n(1));
        }

        if self.peek_nth(0) == '.' && is_digit(self.peek_nth(1)) {
            string.push(self.advance_n(1));

            while is_digit(self.peek_nth(0)) {
                string.push(self.advance_n(1));
            }

            self.tokens.push(Token {
                token_type: Floating {
                    content: string.parse::<f64>().unwrap(),
                },
                lexeme: string,
                line: self.line,
            })
        } else {
            self.tokens.push(Token {
                token_type: Integer {
                    content: string.parse::<i64>().unwrap(),
                },
                lexeme: string,
                line: self.line,
            })
        }
    }

    fn add_identifier_token(&mut self, c: char) {
        let mut string: String = format!("{}", c);

        while is_alpha_numeric(self.peek_nth(0)) {
            string.push(self.advance_n(0))
        }

        match get_keyword(string.as_str()) {
            Some(token_type) => {
                self.tokens.push(Token {
                    token_type,
                    lexeme: string,
                    line: self.line,
                });
            }
            _ => {
                self.tokens.push(Token {
                    token_type: Identifier {
                        name: string.clone(),
                    },
                    lexeme: string,
                    line: self.line,
                });
            }
        };
    }

    // Check if the next nth char reached/passed EOF.
    fn is_nth_eof(&self, n: usize) -> bool {
        self.current + n >= self.source.len()
    }

    // Check if the next chars are exactly that.
    fn is_followed_by(&mut self, chars: &str) -> bool {
        for (i, expected_char) in chars.chars().enumerate() {
            if self.current + i >= self.source.len() {
                return false;
            }

            match self.source.chars().nth(self.current + i) {
                Some(current_char) => {
                    if expected_char != current_char {
                        return false;
                    }
                }
                None => return false,
            }
        }

        self.current += chars.len();

        true
    }

    // Return the current char and advance n chars.
    fn advance_n(&mut self, n: usize) -> char {
        match self.source.chars().nth(self.current) {
            Some(c) => {
                self.current += n;
                c
            }
            _ => '\0',
        }
    }

    // Get the next nth char value.
    fn peek_nth(&self, n: usize) -> char {
        match self.source.chars().nth(self.current + n) {
            Some(c) => c,
            _ => '\0',
        }
    }

    // Keep advancing until meet one of the chars.
    fn advance_until_one_of(&mut self, chars: &str, escape: bool) -> Result<String, ()> {
        let mut string: String = "".to_string();
        let chrs: Vec<char> = chars.chars().collect();

        while !chrs.contains(&self.peek_nth(0)) {
            if self.is_nth_eof(0) {
                return Err(());
            }

            // Remember to keep counting lines.
            if self.peek_nth(0) == '\n' {
                self.line += 1
            };

            if escape {
                string.push(self.escape());
            } else {
                string.push(self.advance_n(1));
            }
        }

        Ok(string)
    }

    // Advance and convert escape characters.
    fn escape(&mut self) -> char {
        if self.peek_nth(0) != '\\' {
            return self.advance_n(1);
        }

        // Current char is backslash, next one can be an special char.
        // In this case you have to skip 2 characters.
        match self.peek_nth(1) {
            '0' => {
                self.advance_n(2);
                '\0'
            }
            'n' => {
                self.advance_n(2);
                '\n'
            }
            'r' => {
                self.advance_n(2);
                '\r'
            }
            't' => {
                self.advance_n(2);
                '\t'
            }
            '\\' => {
                self.advance_n(2);
                '\\'
            }
            '"' => {
                self.advance_n(2);
                '"'
            }
            '\'' => {
                self.advance_n(2);
                '\''
            }
            _ => self.advance_n(1),
        }
    }
}
