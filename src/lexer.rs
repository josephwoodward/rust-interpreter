#![allow(dead_code)]

use crate::token::{lookup_identifier, Token, TokenKind};

pub struct Lexer<'a> {
    position: usize,
    read_position: usize,
    ch: char,
    input: &'a str,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        let mut lexer = Lexer {
            position: 0,
            read_position: 0,
            ch: 0 as char,
            input,
        };
        lexer.read_char();

        lexer
    }

    fn peek_char(&mut self) -> char {
        if self.read_position >= self.input.len() {
            0 as char
        } else {
            if let Some(ch) = self.input.chars().nth(self.read_position) {
                ch
            } else {
                panic!("read out of range")
            }
        }
    }

    fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0 as char;
        } else {
            if let Some(ch) = self.input.chars().nth(self.read_position) {
                self.ch = ch;
            } else {
                panic!("read out of range")
            }
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    fn read_number(&mut self) -> usize {
        let pos = self.position;
        while is_digit(self.ch) {
            self.read_char();
        }

        self.input[pos..self.position].parse().unwrap()
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let t = match self.ch {
            ';' => TokenKind::SEMICOLON,
            '(' => TokenKind::LPAREN,
            ')' => TokenKind::RPAREN,
            '{' => TokenKind::LBRACE,
            '}' => TokenKind::RBRACE,
            ',' => TokenKind::COMMA,
            '+' => TokenKind::PLUS,
            '-' => TokenKind::MINUS,
            '=' => {
                if self.peek_char() == '=' {
                    self.read_char();
                    TokenKind::EQ
                } else {
                    TokenKind::ASSIGN
                }
            }
            '0' => TokenKind::EOF,
            _ => {
                if is_letter(self.ch) {
                    let ident = self.read_identifier();
                    return Token {
                        kind: lookup_identifier(&ident),
                        literal: ident,
                    };
                } else if is_digit(self.ch) {
                    let num = self.read_number();
                    return Token {
                        kind: TokenKind::INT(num.try_into().unwrap()),
                        literal: self.read_identifier().to_string(),
                    };
                } else {
                    TokenKind::ILLEGAL
                }
            }
        };

        self.read_char();

        return Token {
            kind: t,
            literal: "boo".to_string(),
        };
    }

    pub fn skip_whitespace(&mut self) {
        if self.ch == ' ' || self.ch == '\t' || self.ch == '\n' || self.ch == '\r' {
            self.read_char();
        }
    }

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }

        let x = self.input[position..self.position].to_string();

        // println!("chars {}", x);
        x
    }
}

pub fn is_letter(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

pub fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

#[cfg(test)]
mod tests {
    use crate::token::TokenKind;

    use super::Lexer;

    #[test]
    fn test_next_token_simple() {
        let input = "=+(){},;==";
        let mut lexer = Lexer::new(input.into());

        let tokens = vec![
            TokenKind::ASSIGN,
            TokenKind::PLUS,
            TokenKind::LPAREN,
            TokenKind::RPAREN,
            TokenKind::LBRACE,
            TokenKind::RBRACE,
            TokenKind::COMMA,
            TokenKind::SEMICOLON,
            TokenKind::EQ,
        ];

        for token in tokens {
            let next_token = lexer.next_token();
            println!("expected: {:?}, received {:?}", token, next_token.kind);
            assert_eq!(token, next_token.kind);
        }
    }

    #[test]
    fn test_assignment_mixed_spaces() {
        let input = r#"let five = 5;let six=6;"#;
        let mut lexer = Lexer::new(input.into());

        let tokens = vec![
            TokenKind::LET,
            TokenKind::IDENTIFIER {
                name: "five".to_string(),
            },
            TokenKind::ASSIGN,
            TokenKind::INT(5),
            TokenKind::SEMICOLON,
            TokenKind::LET,
            TokenKind::IDENTIFIER {
                name: "six".to_string(),
            },
            TokenKind::ASSIGN,
            TokenKind::INT(6),
            TokenKind::SEMICOLON,
        ];

        for token in tokens {
            let next_token = lexer.next_token();
            println!("expected: {:?}, received {:?}", token, next_token.kind);
            assert_eq!(token, next_token.kind);
        }
    }
}
