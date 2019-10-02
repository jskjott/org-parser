mod scanner;

#[macro_use]
extern crate serde_derive;
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn parse(input: &str) -> JsValue {
    let result = scanner::scan(input.to_string());

    JsValue::from_serde(&result).unwrap()
}

#[cfg(test)]
mod tests {

    use crate::scanner;
    use crate::scanner::Token;
    use crate::scanner::TokenType;

    #[test]
    fn doc_title() {
        let result = scanner::scan("#+TITLE LifeRepo".to_string());

        let expected = vec![
            Token {
                token_type: TokenType::Title,
                lexeme: "#+TITLE".to_string(),
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
                token_type: TokenType::Star,
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
    fn multi_line() {
        let result = scanner::scan("** TODO Futurice \n *** brainstorming ".to_string());

        let expected = vec![
            Token {
                token_type: TokenType::Star,
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
                token_type: TokenType::Star,
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
