#![allow(dead_code)]

use crate::{lexer::Lexer, token::Token};

pub struct Parser<'a> {
    lexer: Lexer<'a>,
    current_token: Token,
    peek_token: Token,
}

impl<'a> Parser<'a> {
    pub fn new(mut lexer: Lexer<'a>) -> Self {
        let cur = lexer.next_token();
        let next = lexer.next_token();

        Parser {
            lexer,
            current_token: cur,
            peek_token: next,
        }
    }

    /// Returns the next token of this [`Parser`].
    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }
}
