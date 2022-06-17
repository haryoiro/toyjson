use std::{
    str::{Bytes, Chars},
    string::ParseError,
};

use crate::{
    error::JSONError,
    token::{lookup_ident, Location, Token, TokenType},
};

#[derive(Debug, Clone)]
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

    fn new_token(&self, token_type: TokenType) -> Token {
        Token {
            location: self.location.clone(),
            token_type,
            value: None,
        }
    }

    fn new_literal(&self, token_type: TokenType, value: String) -> Token {
        Token {
            location: self.location.clone(),
            token_type,
            value: Some(value),
        }
    }

    pub fn next_token(&mut self) -> Token {
        let token = match self.ch {
            '"' => {
                let val = self.string();
                self.new_literal(TokenType::String, val)
            }
            ':' => self.new_token(TokenType::Colon),
            ',' => self.new_token(TokenType::Comma),
            '[' => self.new_token(TokenType::LeftBracket),
            ']' => self.new_token(TokenType::RightBracket),
            '{' => self.new_token(TokenType::LeftBrace),
            '}' => self.new_token(TokenType::RightBrace),
            '\0' => self.new_token(TokenType::EOF),
            _ => {
                if self.is_letter() {
                    let ident = self.ident();
                    let token_type = TokenType::lookup_ident(&ident);
                    self.new_literal(token_type, ident)
                } else if self.is_digit(true) {
                    let num = self.number();
                    self.new_literal(TokenType::Number, num)
                } else {
                    self.new_token(TokenType::ILLEGAL)
                }
            }
        };
        self.next();
        token
    }

    fn ignore_space(&mut self) {
        while self.ch == '\n' || self.ch == '\r' || self.ch == '\t' || self.ch == ' ' {
            match self.ch {
                '\n' => {
                    self.location.next_line();
                    self.source.next();
                }
                '\r' => {
                    self.next();
                    self.source.next();
                    if self.ch == '\n' {
                        self.location.next_line();
                        self.source.next();
                    }
                }
                '\t' | ' ' => {
                    self.location.next_column();
                }
                _ => break,
            }
            self.next();
        }
    }

    fn string(&mut self) -> String {
        let mut value = String::new();
        loop {
            self.next_with_space();
            match self.ch {
                '"' => break,
                _ => {
                    value.push(self.ch);
                }
            }
        }
        value
    }
    // 12345 | 123.45 | 123.45e6 | 123.45e+6 | 123.45e-6
    fn number(&mut self) -> String {
        let mut value = String::new();

        if self.ch == '-' {
            value.push(self.ch);
            self.next();
        }

        if !matches!(self.ch, '0'..='9') {
            return value;
        }
        if matches!(self.ch, '0') {
            value.push(self.ch);
            self.next();
            if matches!(self.ch, '1'..='9') {
                return value;
            }
        } else if matches!(self.ch, '1'..='9') {
            value.push(self.ch);
            self.next();
            while matches!(self.ch, '0'..='9') {
                value.push(self.ch);
                self.next();
            }
        }

        if matches!(self.ch, '.') {
            value.push(self.ch);
            self.next();
            while matches!(self.ch, '0'..='9') {
                value.push(self.ch);
                self.next();
            }
        }

        if matches!(self.ch, 'e' | 'E') {
            value.push(self.ch);
            self.next();

            if !matches!(self.ch, '-' | '+') {
                return value;
            }

            value.push(self.ch);
            self.next();

            while matches!(self.ch, '0'..='9') {
                value.push(self.ch);
                self.next();
            }
        }
        return value;
    }

    fn ident(&mut self) -> String {
        let mut name = String::new();
        name.push(self.ch);
        let mut peek = self.clone().peekable();

        loop {
            let p = peek.peek().unwrap();
            if matches!(p, ',' | ';' | ':' | ']' | '}' | ' ') {
                break;
            }
            name.push(p.clone());
            peek.next();
            self.next();
        }

        return name;
    }

    fn next_with_space(&mut self) -> Option<char> {
        let res = self.source.next();
        self.ch = res.unwrap_or('\0');
        self.location.next_column();
        res
    }

    fn error(&self) -> Token {
        return self.new_token(TokenType::ILLEGAL);
    }

    fn is_letter(&self) -> bool {
        matches!(self.ch, 'a'..='z' | 'A'..='Z' | '_')
    }
    fn is_digit(&self, zero: bool) -> bool {
        if zero {
            matches!(self.ch, '0'..='9' | '-')
        } else {
            matches!(self.ch, '1'..='9' | '-')
        }
    }
}

impl<T> Iterator for Lexer<T>
where
    T: Iterator<Item = char> + Clone,
{
    type Item = char;
    fn next(&mut self) -> Option<Self::Item> {
        let res = self.source.next();
        self.ch = res.unwrap_or('\0');
        self.ignore_space();
        self.location.next_column();
        res
    }
}

#[cfg(test)]
mod tests {
    use crate::token::TokenType;

    use super::*;

    #[test]
    fn test_parse_string_token() {
        let input = r#"
        "key":"value"
        "#;

        let mut lex = Lexer::<Chars>::new(input);
        loop {
            let t = lex.next_token();
            if t.clone().token_type == TokenType::EOF {
                break;
            }
            println!("{:?}", t);
        }
    }

    #[test]
    fn test_parse_number_token() {
        let inputs = vec![
            "123",
            "-123",
            "123.123",
            "-123.123",
            "123.123e-123",
            "123.123e+123",
        ];

        for input in inputs {
            let mut lex = Lexer::<Chars>::new(input);
            let token = lex.next_token();
            assert_eq!(input, token.value.unwrap());
        }
    }

    #[test]
    fn test_parse_invalid_number() {
        let inputs = vec![
            "+123", "01", "-0.e", "-0.0e0", "-0.0ee-0", ".123", "123e", "123.e",
        ];

        for input in inputs {
            let mut lex = Lexer::<Chars>::new(input);
            let token = lex.next_token();
        }
    }

    #[test]
    fn test_parse_keyword() {
        let inputs = "false \n true \n null ";

        let mut lex = Lexer::<Chars>::new(inputs);
        loop {
            let a = lex.next_token();
            println!("{:?}", a);
            match &a.token_type {
                TokenType::False | TokenType::True | TokenType::Null => {
                    continue;
                }
                TokenType::EOF => {
                    break;
                }
                _ => panic!("must be a null or boolean value"),
            }
        }
    }

    #[test]
    fn test_lexer() {
        let input = r#"
            {
                "foo": "bar",
                "baz": true,
                "qux": null,
                "obj": {
                    "inner": "inner text",
                    "innerArray": [
                        "array1",
                        "array2",
                        "array3"
                    ]
                }
            }
        "#;

        let expected = vec![
            (TokenType::LeftBrace, None),
            (TokenType::String, Some(String::from("foo"))),
            (TokenType::Colon, None),
            (TokenType::String, Some(String::from("bar"))),
            (TokenType::Comma, None),
            (TokenType::String, Some(String::from("baz"))),
            (TokenType::Colon, None),
            (TokenType::True, Some(String::from("true"))),
            (TokenType::Comma, None),
            (TokenType::String, Some(String::from("qux"))),
            (TokenType::Colon, None),
            (TokenType::Null, Some(String::from("null"))),
            (TokenType::Comma, None),
            (TokenType::String, Some(String::from("obj"))),
            // : {
            //     "inner": "inner text",
            //     "innerArray": [
            //         "array1",
            //         "array2",
            //         "array3"
            //     ]
            // }
            (TokenType::Colon, None),
            (TokenType::LeftBrace, None),
            (TokenType::String, Some(String::from("inner"))),
            (TokenType::Colon, None),
            (TokenType::String, Some(String::from("inner text"))),
            (TokenType::Comma, None),
            (TokenType::String, Some(String::from("innerArray"))),
            (TokenType::Colon, None),
            (TokenType::LeftBracket, None),
            (TokenType::String, Some(String::from("array1"))),
            (TokenType::Comma, None),
            (TokenType::String, Some(String::from("array2"))),
            (TokenType::Comma, None),
            (TokenType::String, Some(String::from("array3"))),
            (TokenType::RightBracket, None),
            (TokenType::RightBrace, None),
            (TokenType::RightBrace, None),
            (TokenType::EOF, None),
        ];

        let mut lex = Lexer::<Chars>::new(input);
        println!("{:?}", lex);

        for (i, e) in expected.iter().enumerate() {
            let t = lex.next_token();
            println!("{:?} {:?}\n", t, e);
            assert_eq!(e, &(t.token_type, t.value));
        }
    }
}
