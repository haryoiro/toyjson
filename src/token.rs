use std::fmt::{write, Display, Error, Formatter};

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

impl Display for Token {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self.token_type {
            TokenType::String => write!(f, "\"{}\"", self.value.as_ref().unwrap()),
            TokenType::Number => write!(f, "{}", self.value.as_ref().unwrap()),
            TokenType::Ident => write!(f, " {} ", self.value.as_ref().unwrap()),
            TokenType::True => write!(f, "true"),
            TokenType::False => write!(f, "false"),
            TokenType::Null => write!(f, "null"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::LeftBracket => write!(f, "["),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::RightBracket => write!(f, "]"),
            TokenType::Colon => write!(f, ":"),
            TokenType::Comma => write!(f, ","),
            TokenType::EOF => write!(f, "\n"),
            TokenType::ILLEGAL => write!(f, "ILLEGAL"),
        }
    }
}

impl TokenType {
    pub fn lookup_ident(ident: &str) -> TokenType {
        match ident {
            "true" => TokenType::True,
            "false" => TokenType::False,
            "null" => TokenType::Null,
            _ => TokenType::String,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenType {
    String,       // "string"
    Number,       // 12345 | 123.45 | 123.45e6 | 123.45e+6 | 123.45e-6
    True,         // true
    False,        // false
    Null,         // null
    Ident,        // ident
    LeftBrace,    // {
    RightBrace,   // }
    LeftBracket,  // [
    RightBracket, // ]
    Colon,        // :
    Comma,        // ,
    EOF,          // EOF
    ILLEGAL,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        match self {
            TokenType::String => write!(f, "string"),
            TokenType::Number => write!(f, "number"),
            TokenType::Ident => write!(f, "Ident"),
            TokenType::True => write!(f, "true"),
            TokenType::False => write!(f, "false"),
            TokenType::Null => write!(f, "null"),
            TokenType::LeftBrace => write!(f, "{{"),
            TokenType::RightBrace => write!(f, "}}"),
            TokenType::LeftBracket => write!(f, "["),
            TokenType::RightBracket => write!(f, "]"),
            TokenType::Colon => write!(f, ":"),
            TokenType::Comma => write!(f, ","),
            TokenType::EOF => write!(f, "EOF"),
            TokenType::ILLEGAL => write!(f, "ILLEGAL"),
        }
    }
}

impl From<char> for TokenType {
    fn from(src: char) -> Self {
        match src {
            '{' => TokenType::LeftBrace,
            '}' => TokenType::RightBrace,
            '[' => TokenType::LeftBracket,
            ']' => TokenType::RightBracket,
            ',' => TokenType::Comma,
            ':' => TokenType::Colon,
            _ => TokenType::ILLEGAL,
        }
    }
}

const IDENTIFIERS: &[&str] = &["true", "false", "null"];

pub fn lookup_ident(ident: &str) -> bool {
    IDENTIFIERS.contains(&ident)
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
