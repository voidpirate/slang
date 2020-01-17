use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum TokenType {
    ILLEGAL(),
    EOF(char),

    IDENT(char),
    INT(u64),

    ASSIGN(char),
    PLUS(char),

    COMMA(char),
    SEMICOLON(char),

    LPAREN(char),
    RPAREN(char),
    LBRACE(char),
    RBRACE(char),

    FUNCTION(char),
    LET(char),
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let tk = match *self {
            TokenType::ILLEGAL() => "".to_string(),
            TokenType::EOF(c) => c.to_string(),
            TokenType::IDENT(c) => c.to_string(),
            TokenType::INT(c) => c.to_string(),
            TokenType::ASSIGN(c) => c.to_string(),
            TokenType::PLUS(c) => c.to_string(),
            TokenType::COMMA(c) => c.to_string(),
            TokenType::SEMICOLON(c) => c.to_string(),
            TokenType::LPAREN(c) => c.to_string(),
            TokenType::RPAREN(c) => c.to_string(),
            TokenType::LBRACE(c) => c.to_string(),
            TokenType::RBRACE(c) => c.to_string(),
            TokenType::FUNCTION(c) => c.to_string(),
            TokenType::LET(c) => c.to_string(),
        };
        write!(f, "{}", tk)
    }
}
