#![allow(dead_code)]

use crate::{
    ast::{Program, Statement},
    lexer::Lexer,
    token::{Token, TokenKind},
};

type ParseError = String;

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

    fn current_token_is(&mut self, token: TokenKind) -> bool {
        self.current_token.kind == token
    }

    pub fn parse_statement(&self) -> Result<Statement, ParseError> {
        let s = Statement {};
        Ok(s)
    }

    pub fn parse_program(&mut self) -> Result<Program, ParseError> {
        let mut p = Program::new();
        while !self.current_token_is(TokenKind::EOF) {
            match self.parse_statement() {
                Ok(s) => p.statements.push(s),
                Err(_) => todo!(),
            }

            self.next_token();
        }

        Ok(p)
    }
}

#[cfg(test)]
mod tests {

    use super::Parser;
    use crate::lexer::Lexer;

    #[test]
    fn test_parser() {
        let input = r#"
        let five = 5;
        let six=6;
        let msg = "HelloWorld!";
        "#;

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse_program().expect("failed to parse program");

        assert_eq!(program.statements.len(), 3);
    }
}
