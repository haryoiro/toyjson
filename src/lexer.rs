use std::str::Chars;

use crate::token::{Location, Token, TokenType};

#[derive(Debug)]
pub struct Lexer<T: Iterator<Item = char> + Clone> {
    source: T,
    location: Location,
    ch: char,
}

impl<T> Lexer<T>
where
    T: Iterator<Item = char> + Clone,
{
    pub fn new<'a>(source: &'a str) -> Lexer<Chars> {
        let mut lex = Lexer {
            source: source.chars(),
            location: Location::new(1, 1),
            ch: ' ',
        };

        lex.next();
        return lex;
    }

    pub fn new_token(&self, token_type: TokenType) -> Token {
        Token {
            location: self.location,
            token_type,
            value: None,
        }
    }
    pub fn new_literal(&self, token_type: TokenType, value: String) -> Token {
        Token {
            location: self.location,
            token_type,
            value: Some(value),
        }
    }

    pub fn next_char(&mut self) {
        self.ch = self.next().unwrap_or('\0');
    }

    pub fn next_token(&mut self) -> Token {
        self.ignore_space();

        match self.ch {
            '0'..='9' | '-' => self.number(),  // 0,1,2,3 ... -1,-2,-3
            't' | 'f' | 'n' => self.keyword(), // true | false | null
            '"' => self.string(),
            ':' => self.new_token(TokenType::Colon),
            ',' => self.new_token(TokenType::Comma),
            '[' => self.new_token(TokenType::LeftBracket),
            ']' => self.new_token(TokenType::RightBracket),
            '{' => self.new_token(TokenType::LeftBrace),
            '}' => self.new_token(TokenType::RightBrace),
            '\0' => self.new_token(TokenType::EOF),
            _ => self.new_token(TokenType::ILLEGAL),
        }
    }

    fn ignore_space(&mut self) {
        while self.ch == '\n' || self.ch == '\r' || self.ch == '\t' || self.ch == ' ' {
            match self.ch {
                '\n' => {
                    self.location.next_line();
                }
                '\r' => {
                    self.next_char();
                    if self.ch == '\n' {
                        self.location.next_line();
                    }
                }
                '\t' | ' ' => {
                    self.location.next_column();
                }
                _ => break,
            }
            self.next_char();
        }
    }

    fn string(&mut self) -> Token {
        unimplemented!()
    }
    fn number(&mut self) -> Token {
        unimplemented!()
    }
    fn keyword(&mut self) -> Token {
        unimplemented!()
    }
}

impl<T> Iterator for Lexer<T>
where
    T: Iterator<Item = char> + Clone,
{
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        self.source.next()
    }
}

#[cfg(test)]
mod tests {
    use crate::token::TokenType;

    use super::*;

    #[test]
    fn test_next_char() {
        let inputs = r#"


        {
            "hello": "world",
        }
12345
123
12    123
12345
        "#;

        let mut lex = Lexer::<Chars>::new(&inputs);
        println!("{:?}", &lex);

        for i in 0..inputs.len() - 1 {
            let ch = lex.next();
            println!("{:?} {}: {:?}", lex.location, i, ch);
        }
    }
    #[test]
    fn test_lexer() {
        let input = r#"
            {
                "foo": "bar",
                "baz": true,
                "qux": null,
                "quux": [
                    "corge",
                    "grault",
                    "garply"
                ]
                "waldo": {
                    "fred": 123,
                    "waldo": 12.3,
                    "thud": 123.456e-6,
                    "thud": 123.456e+6,
                }
            }
            [
                "foo",
                "bar",
            ]
            "foo" : "bar",
        "#;

        let expected = vec![
            (TokenType::LeftBrace, None),
            (TokenType::String, Some(String::from("foo"))),
            (TokenType::Colon, None),
            (TokenType::String, Some(String::from("bar"))),
            (TokenType::Comma, None),
            (TokenType::String, Some(String::from("baz"))),
            (TokenType::Colon, None),
            (TokenType::Boolean, Some(String::from("true"))),
            (TokenType::Comma, None),
            (TokenType::String, Some(String::from("qux"))),
            (TokenType::Colon, None),
            (TokenType::Null, None),
            (TokenType::Comma, None),
            (TokenType::String, Some(String::from("quux"))),
            (TokenType::LeftBracket, None),
            (TokenType::String, Some(String::from("corge"))),
            (TokenType::Comma, None),
            (TokenType::String, Some(String::from("grault"))),
            (TokenType::Comma, None),
            (TokenType::String, Some(String::from("garply"))),
            (TokenType::RightBracket, None),
            (TokenType::Comma, None),
            (TokenType::String, Some(String::from("waldo"))),
            (TokenType::LeftBrace, None),
            (TokenType::String, Some(String::from("fred"))),
            (TokenType::Colon, None),
            (TokenType::Number, Some(String::from("123"))),
            (TokenType::Comma, None),
            (TokenType::String, Some(String::from("waldo"))),
            (TokenType::Colon, None),
            (TokenType::Number, Some(String::from("12.3"))),
            (TokenType::Comma, None),
            (TokenType::String, Some(String::from("thud"))),
            (TokenType::Colon, None),
            (TokenType::Number, Some(String::from("123.456e-6"))),
            (TokenType::Comma, None),
            (TokenType::String, Some(String::from("thud"))),
            (TokenType::Colon, None),
            (TokenType::Number, Some(String::from("123.456e+6"))),
            (TokenType::RightBrace, None),
            (TokenType::Comma, None),
            (TokenType::LeftBracket, None),
            (TokenType::String, Some(String::from("foo"))),
            (TokenType::Comma, None),
            (TokenType::String, Some(String::from("bar"))),
            (TokenType::RightBracket, None),
            (TokenType::Comma, None),
            (TokenType::String, Some(String::from("foo"))),
            (TokenType::Colon, None),
            (TokenType::String, Some(String::from("bar"))),
            (TokenType::RightBrace, None),
            (TokenType::Comma, None),
            (TokenType::EOF, None),
        ];
    }
}
