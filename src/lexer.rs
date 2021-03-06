use crate::token::TokenType;
use std::iter::Iterator;

#[derive(Debug)]
pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    line_number: u32,
    column: u32,
    ch: char,
}

impl Lexer {
    pub fn new(input: Option<&str>) -> Result<Lexer, &str> {
        if input.is_none() {
            return Err("Lexer: invalid source");
        }

        // Check that we can initialize the lexer to the first character from
        // the input string
        let input = input.unwrap();
        let first_ch = if let Some(c) = input.chars().next() {
            c
        } else {
            '\0'
        };

        let mut lex = Lexer {
            input: input.to_string(),
            position: 0,
            read_position: 0,
            line_number: 1,
            column: 1,
            ch: first_ch,
        };
        lex.read_char();
        Ok(lex)
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
            return;
        }

        if self.read_position > 0 {
            if let Some(c) = self.input.chars().nth(self.read_position) {
                self.ch = c
            } else {
                self.ch = '\0'
            }
        }

        if self.ch == '\n' {
            self.line_number += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }

        self.position = self.read_position;
        self.read_position += 1;
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

    fn get_number(&mut self) -> Option<i64> {
        let position = self.position;
        let mut read_sign = false;
        while self.is_digit() || self.ch == '-' {
            if self.ch == '-' && !read_sign {
                read_sign = true;
            } else if self.ch == '-' && read_sign {
                // Return None here to signal we have an invalid number. Later
                // we will add syntax errors, because this would be one.
                return None;
            }
            self.read_char();
        }
        let piece = &self.input[position..self.position];
        if let Ok(n) = piece.parse::<i64>() {
            return Some(n);
        }
        None
    }

    fn is_digit(&self) -> bool {
        self.ch.is_digit(10)
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
            '-' => {
                // Peek the next character to see if this is a signed number
                if self.peek_char().is_digit(10) {
                    if let Some(num) = self.get_number() {
                        return Some(TokenType::INT(num));
                    }
                }
                Some(TokenType::MINUS('-'))
            }
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
            '\0' => Some(TokenType::EOF),
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
        assert!(Lexer::new(None).is_err())
    }

    #[test]
    fn empty_input_source() {
        const INPUT_SRC: &str = "";
        let lexer = Lexer::new(Some(INPUT_SRC));
        assert!(lexer.is_ok());

        let tok = lexer.unwrap().next();
        assert!(tok.is_some());
        assert_eq!(TokenType::EOF, tok.unwrap())
    }

    #[test]
    fn next_token() {
        const INPUT: &str = "let five = 5;
let ten = 10;
let add = fn(x, y) {
    x + y;
}

let result = add(five, ten);
!-/*5;
5 < 10 > 5;

if (5 < 10) {
    return true;
} else {
    return false;
}

10 == 10;
10 != 9;
let n = -10;";

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
            TokenType::SEMICOLON(';'),
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
            TokenType::LET("let".to_string()),
            TokenType::IDENT("n".to_string()),
            TokenType::ASSIGN('='),
            TokenType::INT(-10),
            TokenType::SEMICOLON(';'),
            TokenType::EOF,
        ];

        match Lexer::new(Some(INPUT)) {
            Ok(mut lex) => {
                for (i, test_token) in tests.iter().enumerate() {
                    if let Some(tok) = lex.next() {
                        assert_eq!(
                            test_token,
                            &tok,
                            "{}",
                            get_lexer_test_error(i, test_token, Some(&tok), &lex)
                        )
                    } else {
                        panic!(get_lexer_test_error(i, test_token, None, &lex))
                    }
                }
            }
            Err(err) => panic!(err),
        }
    }

    fn get_line_number_and_column(lex: &Lexer) -> String {
        format!("Line: {}, Column: {}", lex.line_number, lex.column)
    }

    fn get_lexer_test_error(
        test_num: usize,
        test_token: &TokenType,
        token: Option<&TokenType>,
        lex: &Lexer,
    ) -> String {
        format!(
            "Test case ({}): Expected TokenType: {}, Got TokenType: {}. {}",
            test_num,
            test_token,
            if let Some(tk) = token {
                tk.to_string()
            } else {
                "None".to_string()
            },
            get_line_number_and_column(lex)
        )
    }
}
