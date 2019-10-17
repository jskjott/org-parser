extern crate wasm_bindgen;
use regex::Regex;
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum TokenType {
    // Repeat-character tokens.
    Asterisk,

    // Literals.
    String,

    // Keywords.
    Title,
    Author,
    InitiationDate,
    LogBook,
    Bold,
    Italic,
    Underline,
    Strikethrough,
    Link,
    Clock,
    End,
    Duration,
    Date,
    Timestamp,
    Scheduled,
    Deadline,
    Todo,
    Done,

    // End of line
    EOF,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub line: usize,
}

pub fn scan(source: String) -> Vec<Token> {
    Scanner {
        source: source,
        tokens: vec![],
        start: 0,
        current: 0,
        line: 1,
    }
    .scan_source()
}

struct Scanner {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

impl Scanner {
    fn scan_source(mut self) -> Vec<Token> {
        while !self.is_at_end() {
            {
                self.start = self.current;
            }
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            line: self.line,
        });

        self.tokens
    }

    fn scan_token(&mut self) {
        let c: char = self.advance();

        match c {
            '*' => self.asterisk(),
            '<' => self.angle_bracket(),
            '[' => self.square_bracket(),
            '/' => self.forward_slash(),
            '_' => self.underline(),
            '+' => self.plus(),
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => self.number(),
            ' ' => (),
            '\r' => (),
            '\t' => (),
            '\n' => self.line = self.line + 1,
            _ => {
                if is_alpha(c) {
                    self.identifier();
                } else {
                    panic!(format!(
                        "line {:?}, Unexpected character: {:?}",
                        self.line, c
                    ));
                }
            }
        }
    }

    fn number(&mut self) {
        while self.peek().is_numeric() || self.peek() == ':' {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].to_string();

        let duration = Regex::new(r"^[\d]*:[\d]*$").unwrap();

        let token_type: TokenType;

        if duration.is_match(&text) {
            token_type = TokenType::Duration
        } else {
            token_type = TokenType::String
        };

        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text,
            line: self.line,
        })
    }

    fn asterisk(&mut self) {
        while is_alpha(self.peek()) | self.peek().is_alphanumeric() {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].to_string();

        let star_only = Regex::new(r"^[*]*$").unwrap();
        let bold = Regex::new(r"^[*].*[*]$").unwrap();

        let token_type: TokenType;

        if star_only.is_match(&text) {
            token_type = TokenType::Asterisk
        } else if bold.is_match(&text) {
            token_type = TokenType::Bold
        } else {
            token_type = TokenType::String
        };

        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text,
            line: self.line,
        })
    }

    fn angle_bracket(&mut self) {
        while is_alpha(self.peek()) | self.peek().is_alphanumeric() {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].to_string();

        let date_format = Regex::new(r"^<\d{4}-\d{2}-\d{2}$").unwrap();

        if !date_format.is_match(&text) {
            self.tokens.push(Token {
                token_type: TokenType::String,
                lexeme: text,
                line: self.line,
            });
        } else {
            while '>' != self.peek() {
                self.advance();
            }

            self.advance();

            let text: String = self.source[self.start..self.current].to_string();

            self.tokens.push(Token {
                token_type: TokenType::Date,
                lexeme: text,
                line: self.line,
            });
        }
    }

    fn square_bracket(&mut self) {
        let mut within_par: bool = true;
        let mut nesting: usize = 1;

        while within_par {
            if self.peek() == '[' {
                nesting = nesting + 1;
            } else if self.peek() == ']' {
                nesting = nesting - 1;
            }

            self.advance();

            if self.peek() == ']' && nesting == 1 {
                within_par = false;
            }
        }

        self.advance();

        let text: String = self.source[self.start..self.current].to_string();

        let timestamp = Regex::new(r"^\[\d{4}-\d{2}-\d{2} \w{3} \d{2}:\d{2}]$").unwrap();
        let link = Regex::new(r"^\[{2}.*[\]|\[]{2}.*]{2}$").unwrap();

        let token_type: TokenType;

        if timestamp.is_match(&text) {
            token_type = TokenType::Timestamp
        } else if link.is_match(&text) {
            token_type = TokenType::Link
        } else {
            token_type = TokenType::String
        };

        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text,
            line: self.line,
        })
    }

    fn forward_slash(&mut self) {
        while is_alpha(self.peek()) | self.peek().is_alphanumeric() {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].to_string();

        let italic = Regex::new(r"^[/].*[/]$").unwrap();

        let token_type: TokenType;

        if italic.is_match(&text) {
            token_type = TokenType::Italic
        } else {
            token_type = TokenType::String
        };

        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text,
            line: self.line,
        })
    }

    fn underline(&mut self) {
        while is_alpha(self.peek()) | self.peek().is_alphanumeric() {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].to_string();

        let underline = Regex::new(r"^[_].*[_]$").unwrap();

        let token_type: TokenType;

        if underline.is_match(&text) {
            token_type = TokenType::Underline
        } else {
            token_type = TokenType::String
        };

        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text,
            line: self.line,
        })
    }

    fn plus(&mut self) {
        while is_alpha(self.peek()) | self.peek().is_alphanumeric() {
            self.advance();
        }

        let text: String = self.source[self.start..self.current].to_string();

        let strikethrough = Regex::new(r"^[+].*[+]$").unwrap();

        let token_type: TokenType;

        if strikethrough.is_match(&text) {
            token_type = TokenType::Strikethrough
        } else {
            token_type = TokenType::String
        };

        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text,
            line: self.line,
        })
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> char {
        self.current = self.current + 1;
        self.source.chars().nth(self.current - 1).unwrap()
    }

    fn add_token(&mut self, token_type: TokenType) {
        let text: String = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            token_type: token_type,
            lexeme: text,
            line: self.line,
        })
    }

    fn peek(&mut self) -> char {
        let c: char;

        if self.is_at_end() {
            c = '\0'
        } else {
            c = self.source.chars().nth(self.current).unwrap()
        }

        c
    }

    fn identifier(&mut self) {
        while is_alpha(self.peek()) {
            self.advance();
        }

        let mut keywords = HashMap::new();

        keywords.insert("#+TITLE:".to_string(), TokenType::Title);
        keywords.insert("#+AUTHOR:".to_string(), TokenType::Author);
        keywords.insert("#+DATE:".to_string(), TokenType::InitiationDate);
        keywords.insert(":LOGBOOK:".to_string(), TokenType::LogBook);
        keywords.insert("CLOCK:".to_string(), TokenType::Clock);
        keywords.insert(":END:".to_string(), TokenType::End);
        keywords.insert("SCHEDULED:".to_string(), TokenType::Scheduled);
        keywords.insert("DEADLINE:".to_string(), TokenType::Deadline);
        keywords.insert("TODO".to_string(), TokenType::Todo);
        keywords.insert("DONE".to_string(), TokenType::Done);

        let text: String = self.source[self.start..self.current].to_string();

        let identifier = keywords.get(&text);

        let token_type = match identifier {
            Some(x) => x.clone(),
            None => TokenType::String,
        };

        self.add_token(token_type)
    }
}

fn is_alpha(c: char) -> bool {
    c.is_alphabetic()
        || c == ':'
        || c == '#'
        || c == '+'
        || c == '*'
        || c == '_'
        || c == '-'
        || c == '.'
        || c == ','
        || c == '/'
        || c == '='
        || c == '>'
        || c == '~'
        || c == '^'
        || c == '?'
        || c == '!'
        || c == '\u{27}'
        || c == '('
        || c == ')'
}
