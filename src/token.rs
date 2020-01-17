use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum TokenType {
    EOF(char),

    IDENT(String),
    INT(u64),

    ASSIGN(char),
    PLUS(char),

    COMMA(char),
    SEMICOLON(char),

    LPAREN(char),
    RPAREN(char),
    LBRACE(char),
    RBRACE(char),

    FUNCTION(String),
    LET(String),
}

impl TokenType {
    pub fn create(s: &str) -> Option<TokenType> {
        match s {
            "fn" => Some(TokenType::FUNCTION(s.to_string())),
            "let" => Some(TokenType::LET(s.to_string())),
            s => Some(TokenType::IDENT(s.to_string())),
        }
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let tk = match &*self {
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
