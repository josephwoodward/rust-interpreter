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
            input,
            position: 0,      // points to current position that points to ch
            read_position: 0, // points to next character in input
            ch: 0 as char,
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
            '"' => {
                let s = self.read_string();
                TokenKind::STRING(s)
            }
            '\u{0}' => TokenKind::EOF,
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
                    println!("char is '{}'", self.ch);
                    TokenKind::ILLEGAL
                }
            }
        };

        self.read_char();

        return Token {
            kind: t,
            literal: self.ch.to_string(),
        };
    }

    pub fn skip_whitespace(&mut self) {
        while self.ch.is_ascii_whitespace() {
            self.read_char();
        }
    }

    pub fn read_string(&mut self) -> String {
        let pos = self.position + 1;
        loop {
            self.read_char();
            if self.ch == '"' || self.ch == '\u{0}' {
                break;
            }
        }

        let x = self.input[pos..self.position].to_string();

        // TODO: I don't think this is necessary
        // consume the end "
        // if self.ch == '"' {
        //     println!("consuming until the end {}", self.ch);
        //     self.read_char();
        //     println!("next char is {}", self.ch);
        // }

        return x;
    }

    pub fn read_identifier(&mut self) -> String {
        let position = self.position;
        while is_letter(self.ch) {
            self.read_char();
        }

        let x = self.input[position..self.position].to_string();
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

    use insta::assert_snapshot;

    use super::Lexer;
    use crate::token::{Token, TokenKind};

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
        let input = r#"
        let five = 5;

        let six=6;
        let msg = "HelloWorld!";"#;
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
            TokenKind::LET,
            TokenKind::IDENTIFIER {
                name: "msg".to_string(),
            },
            TokenKind::ASSIGN,
            TokenKind::STRING("HelloWorld!".to_string()),
            TokenKind::SEMICOLON,
            TokenKind::EOF,
        ];

        for token in tokens {
            let next_token = lexer.next_token();
            println!("expected: {:?}, received {:?}", token, next_token.kind);
            assert_eq!(token, next_token.kind);
        }
    }

    // #[test]
    fn test_assignment_mixed_spaces_snapshot() {
        let input = r#"
        let five = 5;

        let six=6;
        let name = "Hello World!";
        "#;

        let lexer = Lexer::new(input.into());
        verify_snapshot("simple", lexer, input);
    }

    fn verify_snapshot(name: &str, mut l: Lexer, input: &str) {
        let mut tokens: Vec<Token> = vec![];
        loop {
            let t = l.next_token();
            if t.kind == TokenKind::EOF {
                tokens.push(t);
                break;
            } else {
                tokens.push(t);
            }
        }

        assert_snapshot!(name, serde_json::to_string_pretty(&tokens).unwrap(), input);
    }
}
