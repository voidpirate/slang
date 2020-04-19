use std::cmp::PartialEq;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum TokenType {
    EOF,

    IDENT(String),
    INT(u64),

    // Operator
    ASSIGN(char),
    PLUS(char),
    MINUS(char),
    BANG(char),
    ASTERISK(char),
    SLASH(char),

    LT(char),
    GT(char),
    EQ([char; 2]),
    NOTEQ([char; 2]),

    COMMA(char),
    SEMICOLON(char),

    LPAREN(char),
    RPAREN(char),
    LBRACE(char),
    RBRACE(char),

    // Keywords
    FUNCTION(String),
    LET(String),
    TRUE,
    FALSE,
    IF(String),
    ELSE(String),
    RETURN(String),
}

impl TokenType {
    pub fn create_keyword(s: &str) -> Option<TokenType> {
        match s {
            "fn" => Some(TokenType::FUNCTION(s.to_string())),
            "let" => Some(TokenType::LET(s.to_string())),
            "true" => Some(TokenType::TRUE),
            "false" => Some(TokenType::FALSE),
            "if" => Some(TokenType::IF(s.to_string())),
            "else" => Some(TokenType::ELSE(s.to_string())),
            "return" => Some(TokenType::RETURN(s.to_string())),
            s => Some(TokenType::IDENT(s.to_string())),
        }
    }

    fn print_equality(&self, c: [char; 2]) -> String {
        format!("{}{}", c[0], c[1])
    }
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result {
        let tk = match self {
            TokenType::EOF => '\0'.to_string(),
            TokenType::IDENT(c) => c.to_string(),
            TokenType::INT(c) => c.to_string(),
            TokenType::ASSIGN(c) => c.to_string(),
            TokenType::PLUS(c) => c.to_string(),
            TokenType::MINUS(c) => c.to_string(),
            TokenType::BANG(c) => c.to_string(),
            TokenType::ASTERISK(c) => c.to_string(),
            TokenType::SLASH(c) => c.to_string(),
            TokenType::LT(c) => c.to_string(),
            TokenType::GT(c) => c.to_string(),
            TokenType::EQ(c) => self.print_equality(*c),
            TokenType::NOTEQ(c) => self.print_equality(*c),
            TokenType::COMMA(c) => c.to_string(),
            TokenType::SEMICOLON(c) => c.to_string(),
            TokenType::LPAREN(c) => c.to_string(),
            TokenType::RPAREN(c) => c.to_string(),
            TokenType::LBRACE(c) => c.to_string(),
            TokenType::RBRACE(c) => c.to_string(),
            TokenType::FUNCTION(c) => c.to_string(),
            TokenType::LET(c) => c.to_string(),
            TokenType::TRUE => "true".to_string(),
            TokenType::FALSE => "false".to_string(),
            TokenType::IF(c) => c.to_string(),
            TokenType::ELSE(c) => c.to_string(),
            TokenType::RETURN(c) => c.to_string(),
        };
        write!(f, "{}", tk)
    }
}
