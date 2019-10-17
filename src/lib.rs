mod parser;
mod scanner;

#[macro_use]
extern crate serde_derive;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(input: &str) -> JsValue {
    let result = scanner::scan(input.to_string());

    let result = parser::parse(result);

    JsValue::from_serde(&result).unwrap()
}

#[cfg(test)]
mod tests {

    use crate::parser;
    use crate::parser::Node;
    use crate::scanner;
    use crate::scanner::Token;
    use crate::scanner::TokenType;
    use std::collections::HashMap;

    #[test]
    fn doc_title() {
        let result = scanner::scan("#+TITLE: LifeRepo".to_string());

        let expected = vec![
            Token {
                token_type: TokenType::Title,
                lexeme: "#+TITLE:".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::String,
                lexeme: "LifeRepo".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                line: 1,
            },
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn heading() {
        let result = scanner::scan("*** projects".to_string());

        let expected = vec![
            Token {
                token_type: TokenType::Asterisk,
                lexeme: "***".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::String,
                lexeme: "projects".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                line: 1,
            },
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn date() {
        let result = scanner::scan("<2019-09-25 Wed>".to_string());

        let expected = vec![
            Token {
                token_type: TokenType::Date,
                lexeme: "<2019-09-25 Wed>".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                line: 1,
            },
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn bold() {
        let result = scanner::scan("*hotdogs*".to_string());

        let expected = vec![
            Token {
                token_type: TokenType::Bold,
                lexeme: "*hotdogs*".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                line: 1,
            },
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn timestamp() {
        let result = scanner::scan("[2019-09-19 Thu 10:40]".to_string());

        let expected = vec![
            Token {
                token_type: TokenType::Timestamp,
                lexeme: "[2019-09-19 Thu 10:40]".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                line: 1,
            },
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn link() {
        let result = scanner::scan(
            "[[https://orgmode.org/worg/dev/org-syntax.html][org-mode syntax]]".to_string(),
        );

        let expected = vec![
            Token {
                token_type: TokenType::Link,
                lexeme: "[[https://orgmode.org/worg/dev/org-syntax.html][org-mode syntax]]"
                    .to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                line: 1,
            },
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn italic() {
        let result = scanner::scan("/italic/".to_string());

        let expected = vec![
            Token {
                token_type: TokenType::Italic,
                lexeme: "/italic/".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                line: 1,
            },
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn underline() {
        let result = scanner::scan("_underline_".to_string());

        let expected = vec![
            Token {
                token_type: TokenType::Underline,
                lexeme: "_underline_".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                line: 1,
            },
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn strikethrough() {
        let result = scanner::scan("+strikethrough+".to_string());

        let expected = vec![
            Token {
                token_type: TokenType::Strikethrough,
                lexeme: "+strikethrough+".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                line: 1,
            },
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn underlined_title() {
        let result = scanner::scan("*** _Agenda_".to_string());

        let expected = vec![
            Token {
                token_type: TokenType::Asterisk,
                lexeme: "***".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Underline,
                lexeme: "_Agenda_".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                line: 1,
            },
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn logbook() {
        let result = scanner::scan(
            ":LOGBOOK:
    CLOCK: [2019-09-21 Sat 17:11]--[2019-09-21 Sat 18:24] =>  1:13
    CLOCK: [2019-09-21 Sat 16:26]--[2019-09-21 Sat 16:58] =>  0:32
    :END:"
                .to_string(),
        );

        let expected = vec![
            Token {
                token_type: TokenType::LogBook,
                lexeme: ":LOGBOOK:".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Clock,
                lexeme: "CLOCK:".to_string(),
                line: 2,
            },
            Token {
                token_type: TokenType::Timestamp,
                lexeme: "[2019-09-21 Sat 17:11]".to_string(),
                line: 2,
            },
            Token {
                token_type: TokenType::String,
                lexeme: "--".to_string(),
                line: 2,
            },
            Token {
                token_type: TokenType::Timestamp,
                lexeme: "[2019-09-21 Sat 18:24]".to_string(),
                line: 2,
            },
            Token {
                token_type: TokenType::String,
                lexeme: "=>".to_string(),
                line: 2,
            },
            Token {
                token_type: TokenType::Duration,
                lexeme: "1:13".to_string(),
                line: 2,
            },
            Token {
                token_type: TokenType::Clock,
                lexeme: "CLOCK:".to_string(),
                line: 3,
            },
            Token {
                token_type: TokenType::Timestamp,
                lexeme: "[2019-09-21 Sat 16:26]".to_string(),
                line: 3,
            },
            Token {
                token_type: TokenType::String,
                lexeme: "--".to_string(),
                line: 3,
            },
            Token {
                token_type: TokenType::Timestamp,
                lexeme: "[2019-09-21 Sat 16:58]".to_string(),
                line: 3,
            },
            Token {
                token_type: TokenType::String,
                lexeme: "=>".to_string(),
                line: 3,
            },
            Token {
                token_type: TokenType::Duration,
                lexeme: "0:32".to_string(),
                line: 3,
            },
            Token {
                token_type: TokenType::End,
                lexeme: ":END:".to_string(),
                line: 4,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                line: 4,
            },
        ];

        assert_eq!(result, expected);
    }

    #[test]
    fn multi_line() {
        let result = scanner::scan("** TODO Futurice \n *** brainstorming ".to_string());

        let expected = vec![
            Token {
                token_type: TokenType::Asterisk,
                lexeme: "**".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Todo,
                lexeme: "TODO".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::String,
                lexeme: "Futurice".to_string(),
                line: 1,
            },
            Token {
                token_type: TokenType::Asterisk,
                lexeme: "***".to_string(),
                line: 2,
            },
            Token {
                token_type: TokenType::String,
                lexeme: "brainstorming".to_string(),
                line: 2,
            },
            Token {
                token_type: TokenType::EOF,
                lexeme: "".to_string(),
                line: 2,
            },
        ];

        assert_eq!(result, expected);
    }
}
