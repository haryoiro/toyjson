use std::{collections::HashMap, error::Error, str::Chars};

use crate::{
    error::JSONError,
    lexer::{self, Lexer},
    token::{Token, TokenType},
    value::Value,
};

#[derive(Debug)]
pub struct Parser<'a> {
    lexer: Lexer<Chars<'a>>,

    curr_tok: Token,
    peek_tok: Token,

    errors: Vec<JSONError>,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        let mut p = Parser {
            lexer: Lexer::<Chars>::new(source),
            curr_tok: Token::default(),
            peek_tok: Token::default(),
            errors: vec![],
        };
        p.next_token();
        p.next_token();
        p
    }

    pub fn next_token(&mut self) {
        self.curr_tok = self.peek_tok.clone();
        self.peek_tok = self.lexer.next_token();
    }

    pub fn parse(&mut self) -> Value {
        let value = self.parse_value();

        if self.peek_tok.token_type != TokenType::EOF {
            self.errors.push(JSONError::UnexpectedToken(
                self.peek_tok.clone(),
                self.peek_tok.location,
            ));
        }
        value
    }

    fn parse_value(&mut self) -> Value {
        match self.curr_tok.token_type {
            TokenType::String => self.parse_string(),
            TokenType::Number => self.parse_number(),
            TokenType::True => self.parse_boolean(true),
            TokenType::False => self.parse_boolean(false),
            TokenType::Null => self.parse_null(),
            TokenType::LeftBrace => self.parse_object(),
            TokenType::LeftBracket => self.parse_array(),
            _ => {
                self.errors.push(JSONError::UnexpectedToken(
                    self.curr_tok.clone(),
                    self.curr_tok.location,
                ));
                Value::Null
            }
        }
    }
    fn parse_string(&mut self) -> Value {
        let value = self.curr_tok.value.clone();
        if value.is_none() {
            self.errors.push(JSONError::StringError("".to_string()));
            return Value::Error(format!("-{}- {:?}", self.curr_tok, self.curr_tok.location));
        }
        self.next_token();
        Value::String(value.unwrap())
    }
    fn parse_number(&mut self) -> Value {
        let value = self.curr_tok.value.clone().unwrap();
        self.next_token();
        Value::Number(value.parse::<usize>().unwrap())
    }
    fn parse_boolean(&mut self, value: bool) -> Value {
        self.next_token();
        Value::Boolean(value)
    }
    fn parse_null(&mut self) -> Value {
        self.next_token();
        Value::Null
    }
    fn parse_object(&mut self) -> Value {
        self.next_token();
        let mut object = Vec::new();
        while self.curr_tok.token_type != TokenType::RightBrace {
            let key = self.parse_string().to_string();
            if !self.expect_token(TokenType::Colon) {
                self.emit_error(JSONError::LexcalError(
                    "Expected ':' after object key".to_string(),
                    self.curr_tok.location,
                ));
                break;
            }
            let value = self.parse_value();
            object.push((key, Box::new(value)));
            self.next_token();
            if self.expect_token(TokenType::Comma) {
                self.next_token();
            }

            if self.curr_tok.token_type == TokenType::EOF {
                self.errors.push(JSONError::UnexpectedToken(
                    self.curr_tok.clone(),
                    self.curr_tok.location,
                ));
                break;
            }
        }
        self.next_token();
        Value::Object(object)
    }

    fn parse_array(&mut self) -> Value {
        self.next_token();
        let mut array = Vec::new();
        while self.curr_tok.token_type != TokenType::RightBracket {
            let value = self.parse_value();
            array.push(value);
            if self.curr_tok.token_type == TokenType::Comma {
                self.next_token();
            }
            if self.curr_tok.token_type == TokenType::RightBracket {
                break;
            }
            if self.curr_tok.token_type == TokenType::EOF {
                self.errors.push(JSONError::UnexpectedToken(
                    self.curr_tok.clone(),
                    self.curr_tok.location,
                ));
                break;
            }
        }
        self.next_token();
        Value::Array(array)
    }

    fn emit_error(&mut self, e: JSONError) {
        self.errors.push(e);
    }
    fn expect_token(&mut self, token_type: TokenType) -> bool {
        if self.curr_tok.token_type == token_type {
            self.next_token();
            true
        } else {
            self.emit_error(JSONError::UnexpectedToken(
                self.curr_tok.clone(),
                self.curr_tok.location,
            ));
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::token::TokenType;

    use super::*;

    #[test]
    fn test_next_token() {
        let src = r#"
        {
            "key": key,
            "key2": key2,
            "arr": [
                "el1",
                "el2",
                "el3"
            ],
            "obj": {
                "inobj": obj,
            }
        }
        "#;

        let mut parser = Parser::new(src);
        loop {
            if parser.peek_tok.token_type == TokenType::EOF {
                break;
            }
            parser.next_token();
            println!("{:?}", parser.curr_tok);
        }
    }

    #[test]
    fn test_parse_value() {
        let src = r#"
        {
            "key": "key",
            "key2": "key2",
            "arr": [
                "el1",
                "el2",
                "el3"
            ],
            "obj": {
                "inobj": obj,
            }
        }
        "#;

        let mut parser = Parser::new(src);
        let value = parser.parse();
        println!("{:?}", value);
    }
}
