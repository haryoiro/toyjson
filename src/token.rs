use std::fmt::{Display, Error, Formatter};

pub struct Token {
    pub location: Location,
    pub token_type: TokenType,
    pub value: Option<String>,
}
pub enum TokenType {
    String,       // "string"
    Number,       // 12345 | 123.45 | 123.45e6 | 123.45e+6 | 123.45e-6
    Boolean,      // true | false
    Null,         // null
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
            TokenType::Boolean => write!(f, "boolean"),
            TokenType::Null => write!(f, "null"),
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
    line: usize,
    column: usize,
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
