use std::str::Chars;

use crate::{
    lexer::{self, Lexer},
    token::Token,
};

pub struct Parser<'a> {
    lexer: Lexer<Chars<'a>>,
    curr_tok: Token,
    peek_tok: Token,
}

impl<'a> Parser<'a> {
    pub fn new(source: &'a str) -> Self {
        Parser {
            lexer: Lexer::<Chars>::new(source),
            curr_tok: Token::default(),
            peek_tok: Token::default(),
        }
    }
}
