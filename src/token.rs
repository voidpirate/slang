use std::cmp::PartialEq;

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
