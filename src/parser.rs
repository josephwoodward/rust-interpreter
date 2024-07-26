#![allow(dead_code)]

use crate::{
    ast::{Let, Program, ReturnStatement, Statement},
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

    pub fn parse_statement(&mut self) -> Result<Statement, ParseError> {
        match self.current_token.kind {
            TokenKind::LET => self.parse_let_statement(),
            TokenKind::RETURN => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
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

    fn parse_let_statement(&self) -> Result<Statement, ParseError> {
        let _name = self.current_token.clone();

        // let mut ident = "".to_string();
        match &self.peek_token.kind {
            TokenKind::IDENTIFIER { name } => {
                println!("ident is {}", name);
                // let ident = name.to_string();
            }
            _ => return Err("not something".to_string()),
        }

        // let x = self.next_token();

        Ok(Statement::Let(Let {
            identifier: self.peek_token.clone(),
        }))
    }

    fn parse_return_statement(&mut self) -> Result<Statement, ParseError> {
        let curr_token = self.current_token.clone();
        let st = Statement::Return(ReturnStatement {
            identifier: Token {
                kind: curr_token.kind,
                literal: curr_token.literal.to_string(),
            },
        });

        // TODO: Revisit to parse expression
        while !self.current_token_is(TokenKind::SEMICOLON) {
            self.next_token();
        }

        Ok(st)
    }

    fn parse_expression_statement(&self) -> Result<Statement, ParseError> {
        Ok(Statement::Identifier {
            name: "Nothing".to_string(),
        })
    }
}

#[cfg(test)]
mod tests {

    use super::Parser;
    use crate::{
        ast::{Let, ReturnStatement, Statement},
        token::{Token, TokenKind},
        Lexer,
    };

    #[test]
    fn test_let_statement() {
        let input = r#"
        let five = 5;
        "#;

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse_program().expect("failed to parse program");

        assert_eq!(program.statements.len(), 5);

        // TokenKind::LET,
        // TokenKind::IDENTIFIER {
        //     name: "five".to_string(),
        // },
        // TokenKind::ASSIGN,
        // TokenKind::INT(5),
        // TokenKind::SEMICOLON,

        let exp1 = Statement::Let(Let {
            identifier: Token {
                kind: TokenKind::IDENTIFIER {
                    name: "five".to_string(),
                },
                literal: "five".to_string(),
            },
        });
        // let exp2 = Statement::Let(Let {
        //     identifier: Token {
        //         kind: TokenKind::IDENTIFIER {
        //             name: "x".to_string(),
        //         },
        //         literal: "five".to_string(),
        //     },
        // });
        assert_eq!(program.statements[0], exp1);
        // assert_eq!(program.statements[1], exp2);
    }

    #[test]
    fn test_return_statement() {
        let input = r#"
        return 5;
        "#;

        let mut parser = Parser::new(Lexer::new(input));
        let program = parser.parse_program().expect("failed to parse program");

        assert_eq!(program.statements.len(), 1);

        let exp = Statement::Return(ReturnStatement {
            identifier: Token {
                kind: TokenKind::RETURN,
                literal: "return".to_string(),
            },
        });
        assert_eq!(program.statements[0], exp);
    }
}
