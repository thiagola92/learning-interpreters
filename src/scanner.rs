use crate::error::error;
use crate::token::Content;
use crate::token::Token;
use crate::token::TokenType;
use std::mem::ManuallyDrop;
use std::ptr::drop_in_place;

struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn scan_tokens(&mut self) {
        while !self.is_eof() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            literal: Content {
                null: std::ptr::null(),
            },
            line: self.line,
        });
    }

    fn scan_token(&mut self) {
        let c: char = self.next();

        // Longer tokens needs priority.
        match c {
            // Assignment
            '+' if self.followed_by('=') => self.add_token(TokenType::PLUS_EQUAL),
            '-' if self.followed_by('=') => self.add_token(TokenType::MINUS_EQUAL),
            '*' if self.followed_by('=') => self.add_token(TokenType::STAR_EQUAL),
            '/' if self.followed_by('=') => self.add_token(TokenType::SLASH_EQUAL),
            '%' if self.followed_by('=') => self.add_token(TokenType::PERCENTAGE_EQUAL),

            // Math
            '*' if self.followed_by('*') => self.add_token(TokenType::STAR_STAR),
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

            ////////////////
            '"' => self.add_string_token(),
            '#' => self.add_comment_token(),

            // Special
            '\n' => self.add_newline_token(),
            '\t' => self.add_token(TokenType::INDENT),

            // Ignored
            ' ' => (),
            '\r' => (),

            _ => error(self.line, "Unexpected character.".to_string()),
        };
    }

    fn add_token(&mut self, token_type: TokenType) {
        self.tokens.push(Token {
            token_type,
            lexeme: "".to_string(),
            literal: Content {
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
            literal: Content {
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
            literal: Content {
                string: ManuallyDrop::new(string),
            },
            line: self.line,
        });
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

    fn next(&mut self) -> char {
        let character = self.source.chars().nth(self.current).unwrap();
        self.current += 1;
        character
    }

    fn followed_by(&mut self, c: char) -> bool {
        if self.is_eof() {
            false
        } else if self.source.chars().nth(self.current).unwrap() != c {
            false
        } else {
            self.current += 1;
            true
        }
    }

    fn is_next(&self, c: char) -> bool {
        !self.is_eof() && self.source.chars().nth(self.current).unwrap() == c
    }

    fn consume_until(&mut self, c: char) -> String {
        let mut string: String = "".to_string();

        while self.is_next(c) && !self.is_eof() {
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

        while self.is_next_one_of(&chars) && !self.is_eof() {
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

    fn is_next_one_of(&mut self, chars: &Vec<char>) -> bool {
        match chars.into_iter().find(|&&c| self.is_next(c)) {
            Some(_) => true,
            _ => false,
        }
    }
}
