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
            println!("setting as empty");
            self.ch = 0 as char;
        } else {
            println!("not setting as empty");
            if let Some(ch) = self.input.chars().nth(self.read_position) {
                self.ch = ch;
            } else {
                panic!("read out of range")
            }
        }

        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitespace();

        let t = match self.ch {
            ';' => TokenKind::SEMICOLON,
            '(' => TokenKind::LPAREN,
            ')' => TokenKind::RPAREN,
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
                    println!("is a letter, {}", self.ch);
                    return Token {
                        literal: self.read_identifier().to_string(),
                        token: lookup_identifier(&self.ch.to_string()),
                    };
                }

                println!("returning illegal");
                return Token {
                    token: TokenKind::ILLEGAL,
                    literal: "".to_string(),
                };
            }
        };

        return Token {
            token: t,
            literal: self.ch.to_string(),
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

        self.input[position..self.position].to_string()
    }
}

pub fn is_letter(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_next_token() {
        let _ = "let five = 5".to_string();
        // assert_eq!(
        //     generate_hash("dGhlIHNhbXBsZSBub25jZQ==".to_string()),
        //     "s3pPLMBiTxaQ9kYGzzhZRbK+xOo="
        // )
    }
}
