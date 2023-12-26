use crate::error::error;
use crate::token::Content;
use crate::token::Token;
use crate::token::TokenType;

struct Scanner {
    source: String,
    tokens: Vec<Token>,

    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn new() -> Self {
        Scanner {
            source: "".to_string(),
            tokens: Vec::<Token>::new(),

            start: 0,
            current: 0,
            line: 1,
        }
    }

    fn is_at_end(self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(mut self) -> char {
        let c = self.source.chars().nth(self.current);
        self.current = self.current + 1;
        return c.unwrap();
    }

    fn add_token(mut self, token_type: TokenType) {
        self.tokens.push(Token {
            token_type,
            lexeme: "".to_string(),
            literal: Content {
                null: std::ptr::null(),
            },
            line: self.line,
        })
    }

    fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_at_end() {
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

        self.tokens
    }

    fn scan_token(mut self) {
        let c: char = self.advance();
        let token_type = match c {
            // Math
            '+' => self.add_token(if self.followed_by('=') {
                TokenType::PLUS
            } else {
                TokenType::PLUS_EQUAL
            }),
            '-' => self.add_token(if self.followed_by('=') {
                TokenType::MINUS
            } else {
                TokenType::MINUS_EQUAL
            }),
            '*' => self.add_token(if self.followed_by('=') {
                TokenType::STAR
            } else {
                TokenType::STAR_EQUAL
            }),
            '/' => self.add_token(if self.followed_by('=') {
                TokenType::SLASH
            } else {
                TokenType::SLASH_EQUAL
            }),
            '%' => self.add_token(if self.followed_by('=') {
                TokenType::PERCENTAGE
            } else {
                TokenType::PERCENTAGE_EQUAL
            }),
            // '**' => add_token(STAR_STAR),

            // Open-close
            '(' => self.add_token(TokenType::PARENTHESIS_OPEN),
            ')' => self.add_token(TokenType::PARENTHESIS_CLOSE),
            '[' => self.add_token(TokenType::BRACKET_OPEN),
            ']' => self.add_token(TokenType::BRACKET_CLOSE),
            '{' => self.add_token(TokenType::BRACE_OPEN),
            '}' => self.add_token(TokenType::BRACE_CLOSE),

            ////////////////
            '#' => {
                self.add_token(TokenType::COMMENT);

                while self.peek() != '\n' && !self.is_at_end() {
                    // Comments should be recorded for future usage.
                    self.advance();
                }
            }

            // Special
            '\n' => self.add_token(TokenType::NEWLINE),
            '\t' => self.add_token(TokenType::INDENT),

            // Ignored
            ' ' => (),
            '\r' => (),

            _ => error(self.line, "Unexpected character.".to_string()),
        };
    }

    fn followed_by(&self, c: char) -> bool {
        if self.is_at_end() {
            return false;
        };
        if self.source.chars().nth(self.current).unwrap() != c {
            return false;
        };

        self.current += 1;

        true
    }

    fn peek(self) -> char {
        if self.is_at_end() {
            return '\0';
        };

        self.source.chars().nth(self.current).unwrap()
    }

    fn string(mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1
            };
            self.advance();
        }

        if self.is_at_end() {
            error(self.line, "Unterminated string.".to_string());
            return;
        }

        self.advance();

        // let value: String = self.source.chars();
        // self.add_token(TokenType::STRING, value);
    }
}
