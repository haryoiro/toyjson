use std::{
    str::{Bytes, Chars},
    string::ParseError,
};

use crate::{
    error::JSONError,
    token::{Location, Token, TokenType},
};

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

    pub fn next_token(&mut self) -> Result<Token, JSONError> {
        let token = match self.ch {
            '0'..='9' | '-' => self.number(), // 0,1,2,3 ... -1,-2,-3
            't' => match self.ident("true".chars()) {
                Ok(t) => t,
                Err(e) => return Err(e),
            },
            'f' => match self.ident("false".chars()) {
                Ok(t) => t,
                Err(e) => return Err(e),
            },
            'n' => match self.ident("null".chars()) {
                Ok(t) => t,
                Err(e) => return Err(e),
            },
            '"' => self.string(),
            ':' => self.new_token(TokenType::Colon),
            ',' => self.new_token(TokenType::Comma),
            '[' => self.new_token(TokenType::LeftBracket),
            ']' => self.new_token(TokenType::RightBracket),
            '{' => self.new_token(TokenType::LeftBrace),
            '}' => self.new_token(TokenType::RightBrace),
            '\0' => self.new_token(TokenType::EOF),
            _ => {
                return Err(JSONError::LexcalError(
                    format!("invalid character: {}", self.ch),
                    self.location,
                ));
            }
        };
        self.next();
        Ok(token)
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

    fn string(&mut self) -> Token {
        let mut value = String::new();
        self.next_with_space();

        while self.ch != '\"' {
            value.push(self.ch);
            self.next_with_space();
        }

        self.new_literal(TokenType::String, value)
    }
    // 12345 | 123.45 | 123.45e6 | 123.45e+6 | 123.45e-6
    fn number(&mut self) -> Token {
        let mut value = String::new();

        if self.ch == '-' {
            value.push(self.ch);
            self.next();
        }

        if !matches!(self.ch, '0'..='9') {
            return self.new_token(TokenType::ILLEGAL);
        }
        if matches!(self.ch, '0') {
            value.push(self.ch);
            self.next();
            if matches!(self.ch, '1'..='9') {
                return self.new_token(TokenType::ILLEGAL);
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
                return self.new_token(TokenType::ILLEGAL);
            }

            value.push(self.ch);
            self.next();

            while matches!(self.ch, '0'..='9') {
                value.push(self.ch);
                self.next();
            }
        }

        return self.new_literal(TokenType::Number, value);
    }

    fn ident(&mut self, expects: Chars) -> Result<Token, JSONError> {
        let mut res = String::new();
        let mut expects = expects;
        for _ in 0..expects.clone().count() - 1 {
            let c = &expects.next();
            match c {
                Some(cc) => {
                    if cc != &self.ch {
                        return Err(JSONError::LexcalError(
                            format!("Invalid Identifier {}", res),
                            self.location,
                        ));
                    }
                    res.push(self.ch);
                    self.next();
                }
                None => break,
            }
        }
        res.push(self.ch);
        println!("{}", res);
        Ok(self.new_literal(TokenType::Ident, res))
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
            let t = lex.next_token().unwrap();
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
            assert_eq!(input, token.unwrap_or_default().value.unwrap());
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
            match token {
                Err(_) => return,
                Ok(_) => panic!("must be an error"),
            }
        }
    }

    #[test]
    fn test_parse_keyword() {
        let inputs = "false \n true \n null ";

        let mut lex = Lexer::<Chars>::new(inputs);
        loop {
            let a = lex.next_token();
            println!("{:?}", a);
            match &a.unwrap().token_type {
                TokenType::Ident => {
                    println!("ok");
                }
                TokenType::EOF => {
                    break;
                }
                _ => panic!("must be a null or boolean value"),
            }
        }
    }

    #[test]
    fn test_parse_invalid_keyword() {
        let inputs = vec!["folse", "torue", "nuli"];

        for input in inputs {
            let mut lex = Lexer::<Chars>::new(input);
            let token = lex.next_token().unwrap();
            match token.token_type {
                TokenType::ILLEGAL => {}
                _ => panic!("must be an illegal token!"),
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
            (TokenType::Ident, Some(String::from("true"))),
            (TokenType::Comma, None),
            (TokenType::String, Some(String::from("qux"))),
            (TokenType::Colon, None),
            (TokenType::Ident, Some(String::from("null"))),
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
            let t = lex.next_token().unwrap();
            println!("{:?}\n", t);
            assert_eq!(e, &(t.token_type, t.value));
        }
    }
}
