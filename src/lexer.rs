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

    fn peek_char(&mut self) -> char {
        if self.read_position > self.input.len() {
            return '\0';
        }
        if let Some(c) = self.input.chars().nth(self.read_position) {
            return c;
        }
        '\0'
    }

    fn get_identifier(&mut self) -> &str {
        let position = self.position;
        while self.is_letter() {
            self.read_char()
        }
        &self.input[position..self.position]
    }

    fn is_letter(&self) -> bool {
        'a' <= self.ch && self.ch <= 'z' || 'A' <= self.ch && self.ch <= 'Z' || self.ch == '_'
    }

    fn get_number(&mut self) -> Option<u64> {
        let position = self.position;
        while self.is_digit() {
            self.read_char()
        }
        let piece = &self.input[position..self.position];
        if let Ok(n) = piece.parse::<u64>() {
            return Some(n);
        }
        None
    }

    fn is_digit(&self) -> bool {
        if let Some(_) = self.ch.to_digit(10) {
            return true;
        }
        return false;
    }

    fn skip_whitespace(&mut self) {
        while self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char()
        }
    }
}

impl Iterator for Lexer {
    type Item = TokenType;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespace();
        let tok = match self.ch {
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Some(TokenType::EQ(['=', '=']))
                } else {
                    Some(TokenType::ASSIGN('='))
                }
            }
            '+' => Some(TokenType::PLUS('+')),
            '-' => Some(TokenType::MINUS('-')),
            '!' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    Some(TokenType::NOTEQ(['!', '=']))
                } else {
                    Some(TokenType::BANG('!'))
                }
            }
            '*' => Some(TokenType::ASTERISK('*')),
            '/' => Some(TokenType::SLASH('/')),
            '>' => Some(TokenType::GT('>')),
            '<' => Some(TokenType::LT('<')),
            '(' => Some(TokenType::LPAREN('(')),
            ')' => Some(TokenType::RPAREN(')')),
            '{' => Some(TokenType::LBRACE('{')),
            '}' => Some(TokenType::RBRACE('}')),
            ',' => Some(TokenType::COMMA(',')),
            ';' => Some(TokenType::SEMICOLON(';')),
            '\0' => Some(TokenType::EOF('\0')),
            _ => {
                if self.is_letter() {
                    let ident = self.get_identifier();
                    if let Some(tk) = TokenType::create_keyword(ident) {
                        return Some(tk);
                    }
                } else if self.is_digit() {
                    if let Some(n) = self.get_number() {
                        return Some(TokenType::INT(n));
                    }
                } else {
                    return None;
                }
                None
            }
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
        const INPUT: &str = "let five = 5;
let ten = 10;
let add = fn(x, y) {
    x + y;
}

let result = add(five, ten);
!-/*5
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;";

        println!("{}", INPUT);
        let tests = vec![
            TokenType::LET("let".to_string()),
            TokenType::IDENT("five".to_string()),
            TokenType::ASSIGN('='),
            TokenType::INT(5),
            TokenType::SEMICOLON(';'),
            TokenType::LET("let".to_string()),
            TokenType::IDENT("ten".to_string()),
            TokenType::ASSIGN('='),
            TokenType::INT(10),
            TokenType::SEMICOLON(';'),
            TokenType::LET("let".to_string()),
            TokenType::IDENT("add".to_string()),
            TokenType::ASSIGN('='),
            TokenType::FUNCTION("fn".to_string()),
            TokenType::LPAREN('('),
            TokenType::IDENT("x".to_string()),
            TokenType::COMMA(','),
            TokenType::IDENT("y".to_string()),
            TokenType::RPAREN(')'),
            TokenType::LBRACE('{'),
            TokenType::IDENT("x".to_string()),
            TokenType::PLUS('+'),
            TokenType::IDENT("y".to_string()),
            TokenType::SEMICOLON(';'),
            TokenType::RBRACE('}'),
            TokenType::LET("let".to_string()),
            TokenType::IDENT("result".to_string()),
            TokenType::ASSIGN('='),
            TokenType::IDENT("add".to_string()),
            TokenType::LPAREN('('),
            TokenType::IDENT("five".to_string()),
            TokenType::COMMA(','),
            TokenType::IDENT("ten".to_string()),
            TokenType::RPAREN(')'),
            TokenType::SEMICOLON(';'),
            TokenType::BANG('!'),
            TokenType::MINUS('-'),
            TokenType::SLASH('/'),
            TokenType::ASTERISK('*'),
            TokenType::INT(5),
            TokenType::INT(5),
            TokenType::LT('<'),
            TokenType::INT(10),
            TokenType::GT('>'),
            TokenType::INT(5),
            TokenType::SEMICOLON(';'),
            TokenType::IF("if".to_string()),
            TokenType::LPAREN('('),
            TokenType::INT(5),
            TokenType::LT('<'),
            TokenType::INT(10),
            TokenType::RPAREN(')'),
            TokenType::LBRACE('{'),
            TokenType::RETURN("return".to_string()),
            TokenType::TRUE,
            TokenType::SEMICOLON(';'),
            TokenType::RBRACE('}'),
            TokenType::ELSE("else".to_string()),
            TokenType::LBRACE('{'),
            TokenType::RETURN("return".to_string()),
            TokenType::FALSE,
            TokenType::SEMICOLON(';'),
            TokenType::RBRACE('}'),
            TokenType::INT(10),
            TokenType::EQ(['=', '=']),
            TokenType::INT(10),
            TokenType::SEMICOLON(';'),
            TokenType::INT(10),
            TokenType::NOTEQ(['!', '=']),
            TokenType::INT(9),
            TokenType::SEMICOLON(';'),
            TokenType::EOF('\0'),
        ];

        match Lexer::new(Some(INPUT)) {
            Ok(mut lex) => {
                for (i, test_token) in tests.iter().enumerate() {
                    if let Some(tok) = lex.next() {
                        assert_eq!(
                            test_token, &tok,
                            "Test {} failed, expected token: {}",
                            i, test_token
                        )
                    } else {
                        panic!("Test {} is not expected to be None", i)
                    }
                }
            }
            Err(err) => panic!(err),
        }
    }
}
