use crate::token::TokenType;
use std::iter::Iterator;

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: Option<&str>) -> Result<Lexer, &str> {
        if input.is_none() {
            return Err("Lexer: invalid source");
        }

        let mut lex = Lexer {
            input: input.unwrap().to_string(),
            position: 0,
            read_position: 0,
            ch: input.unwrap().chars().nth(0).unwrap(),
        };
        lex.read_char();
        Ok(lex)
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            if let Some(c) = self.input.chars().nth(self.read_position) {
                self.ch = c;
            }
        }
        self.position = self.read_position;
        self.read_position += 1
    }
}

impl Iterator for Lexer {
    type Item = TokenType;

    fn next(&mut self) -> Option<Self::Item> {
        let tok = match self.ch {
            '=' => Some(TokenType::ASSIGN('=')),
            '+' => Some(TokenType::PLUS('+')),
            '(' => Some(TokenType::LPAREN('(')),
            ')' => Some(TokenType::RPAREN(')')),
            '{' => Some(TokenType::LBRACE('{')),
            '}' => Some(TokenType::RBRACE('}')),
            ',' => Some(TokenType::COMMA(',')),
            ';' => Some(TokenType::SEMICOLON(';')),
            '\0' => Some(TokenType::EOF('\0')),
            _ => None,
        };
        self.read_char();
        tok
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::token::TokenType;

    #[test]
    fn none_input_source() {
        if let Err(e) = Lexer::new(None) {
            assert_eq!(e, "Lexer: invalid source")
        } else {
            panic!("Lexer None case failed")
        }
    }

    #[test]
    fn next_token() {
        const INPUT: &str = "=+(){},;";
        let tests = vec![
            TokenType::ASSIGN('='),
            TokenType::PLUS('+'),
            TokenType::LPAREN('('),
            TokenType::RPAREN(')'),
            TokenType::LBRACE('{'),
            TokenType::RBRACE('}'),
            TokenType::COMMA(','),
            TokenType::SEMICOLON(';'),
            TokenType::EOF('\0'),
        ];

        match Lexer::new(Some(INPUT)) {
            Ok(mut lex) => {
                for (i, test_token) in tests.iter().enumerate() {
                    if let Some(tok) = lex.next() {
                        assert_eq!(test_token, &tok, "Test {} failed", i)
                    } else {
                        panic!("Test {} is not expected to be None", i)
                    }
                }

                // After running the lexer across all chars in input string,
                // verify that EOF token is set properly.
                if let Some(eof) = lex.next() {
                    assert_eq!(eof, TokenType::EOF('\0'))
                } else {
                    panic!("Expected EOF got None")
                }
            }
            Err(err) => panic!(err),
        }
    }
}
