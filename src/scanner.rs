use crate::error::{
    error, UNFINISHED_CHARACTER, UNFINISHED_COMMENT, UNFINISHED_STRING, UNKNOW_CHAR,
    WRONG_CHAR_SIZE,
};
use crate::token::{Content, Token, TokenType, TokenType::*};
use std::collections::HashMap;

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
                ("if".to_string(), If),
                ("match".to_string(), Match),
                ("loop".to_string(), Loop),
                ("while".to_string(), While),
                ("for".to_string(), For),
                ("return".to_string(), Return),
                ("pass".to_string(), Pass),
                ("emit".to_string(), Emit),
                ("await".to_string(), Await),
                ("yield".to_string(), Yield),
                ("resume".to_string(), Resume),
                // Control Flow Modifier
                ("else".to_string(), Else),
                ("break".to_string(), Break),
                ("continue".to_string(), Continue),
                // Definition
                ("var".to_string(), Var),
                ("const".to_string(), Const),
                ("enum".to_string(), Enum),
                ("signal".to_string(), Signal),
                ("func".to_string(), Func),
                ("coroutine".to_string(), Coroutine),
                ("struct".to_string(), Struct),
                ("union".to_string(), Union),
                ("class".to_string(), Class),
                ("singleton".to_string(), Singleton),
                ("interface".to_string(), Interface),
                ("constructor".to_string(), Constructor),
                ("destructor".to_string(), Destructor),
                ("set".to_string(), Set),
                ("get".to_string(), Get),
                ("import".to_string(), Import),
                ("as".to_string(), As),
                // Definition Modifier
                ("static".to_string(), Static),
                ("public".to_string(), Public),
                ("extends".to_string(), Extends),
                ("implements".to_string(), Implements),
                ("from".to_string(), From),
                // Deisgn pattern
                ("in".to_string(), In),
                ("when".to_string(), When),
                // Literal
                ("true".to_string(), True),
                ("false".to_string(), False),
                ("null".to_string(), Null),
                // Logical
                ("not".to_string(), Not),
                ("and".to_string(), And),
                ("or".to_string(), Or),
                // Object-oriented
                ("self".to_string(), Self_),
                ("super".to_string(), Super),
                ("is".to_string(), Is),
                ("to".to_string(), To),
                // Test
                ("breakpoint".to_string(), Breakpoint),
                ("assert".to_string(), Assert),
                ("test".to_string(), Test),
                // Type
                ("bool".to_string(), Bool),
                ("int".to_string(), Int),
                ("float".to_string(), Float),
                ("char".to_string(), Char),
                ("str".to_string(), Str),
                // TODO: Classify
                ("where".to_string(), Where),
                ("with".to_string(), With),
            ]),
        }
    }

    pub fn scan_tokens(mut self) -> Vec<Token> {
        while !self.is_eof() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: Eof,
            lexeme: "".to_string(),
            content: Content::Null,
            line: self.line,
        });

        self.tokens
    }

    fn scan_token(&mut self) {
        let c: char = self.next();

        match c {
            // Assignment
            '>' if self.is_followed_by_both('>', '=') => self.add_token(GreaterGreaterEqual, ">>="),
            '<' if self.is_followed_by_both('<', '=') => self.add_token(LessLessEqual, "<<="),
            '*' if self.is_followed_by_both('*', '=') => self.add_token(StarStarEqual, "**="),

            // Assignment
            '+' if self.is_followed_by('=') => self.add_token(PlusEqual, "+="),
            '-' if self.is_followed_by('=') => self.add_token(MinusEqual, "-="),
            '*' if self.is_followed_by('=') => self.add_token(StarEqual, "*="),
            '/' if self.is_followed_by('=') => self.add_token(SlashEqual, "/="),
            '%' if self.is_followed_by('=') => self.add_token(PercentageEqual, "%="),
            '&' if self.is_followed_by('=') => self.add_token(AmpersandEqual, "&="),
            '|' if self.is_followed_by('=') => self.add_token(PipeEqual, "|="),
            '^' if self.is_followed_by('=') => self.add_token(CaretEqual, "^="),

            // Bitwise
            '>' if self.is_followed_by('>') => self.add_token(GreaterGreater, ">>"),
            '<' if self.is_followed_by('<') => self.add_token(LessLess, "<<"),

            // Comparassion
            '=' if self.is_followed_by('=') => self.add_token(EqualEqual, "=="),
            '!' if self.is_followed_by('=') => self.add_token(NotEqual, "!="),
            '>' if self.is_followed_by('=') => self.add_token(GreaterEqual, ">="),
            '<' if self.is_followed_by('=') => self.add_token(LessEqual, "<="),

            // Math
            '*' if self.is_followed_by('*') => self.add_token(StarStar, "**"),

            // TODO: Classify
            '.' if self.is_followed_by('.') => self.add_token(PeriodPeriod, ".."),
            '-' if self.is_followed_by('>') => self.add_token(ForwardArrow, "->"),

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
            '\t' => self.add_token(Indent, "\\t"),

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

    fn add_token(&mut self, token_type: TokenType, lexeme: &str) {
        self.tokens.push(Token {
            token_type,
            lexeme: lexeme.to_string(),
            content: Content::Null,
            line: self.line,
        });
    }

    fn add_comment_token(&mut self) {
        let string: String = self.consume_until('\n', UNFINISHED_COMMENT, false);

        self.tokens.push(Token {
            token_type: Comment,
            lexeme: "comment".to_string(),
            content: Content::String(string),
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
            token_type: String_,
            lexeme: "string".to_string(),
            content: Content::String(string),
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
            token_type: Character,
            lexeme: "character".to_string(),
            content: Content::Character(string.chars().nth(0).unwrap()),
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
                token_type: Floating,
                lexeme: "floating".to_string(),
                content: Content::Floating(string.parse::<f32>().unwrap()),
                line: self.line,
            });
        } else {
            self.tokens.push(Token {
                token_type: Integer,
                lexeme: "integer".to_string(),
                content: Content::Integer(string.parse::<i32>().unwrap()),
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
                    lexeme: string.to_string(),
                    content: match token_type {
                        True => Content::Boolean(true),
                        False => Content::Boolean(false),
                        _ => Content::Null,
                    },
                    line: self.line,
                });
            }
            _ => {
                self.tokens.push(Token {
                    token_type: Identifier,
                    lexeme: "identifier".to_string(),
                    content: Content::String(string),
                    line: self.line,
                });
            }
        };
    }

    fn add_newline_token(&mut self) {
        self.add_token(Newline, "\\n");
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
