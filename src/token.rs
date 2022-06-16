use std::fmt::{Display, Error, Formatter};

#[derive(Debug, Clone)]
pub struct Token {
    pub location: Location,
    pub token_type: TokenType,
    pub value: Option<String>,
}

impl Default for Token {
    fn default() -> Self {
        Token {
            location: Location::new(1, 1),
            token_type: TokenType::ILLEGAL,
            value: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    String,       // "string"
    Number,       // 12345 | 123.45 | 123.45e6 | 123.45e+6 | 123.45e-6
    Ident,        // null, true, false
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Colon,        // :
    Comma,        // ,
    Whitespace,   // \t | \n | \r | \f | \v
    EOF,          // EOF
    ILLEGAL,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            TokenType::String => write!(f, "string"),
            TokenType::Number => write!(f, "number"),
            TokenType::Ident => write!(f, "Ident"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::LeftBracket => write!(f, "["),
            TokenType::RightBracket => write!(f, "]"),
            TokenType::Colon => write!(f, ":"),
            TokenType::Comma => write!(f, ","),
            TokenType::Whitespace => write!(f, "whitespace"),
            TokenType::EOF => write!(f, "EOF"),
            TokenType::ILLEGAL => write!(f, "ILLEGAL"),
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Location {
        Location {
            line: line,
            column: column,
        }
    }
    pub fn next_column(&mut self) {
        self.column = self.column + 1;
    }
    pub fn next_line(&mut self) {
        self.line = self.line + 1;
        self.column = 1;
    }
}
