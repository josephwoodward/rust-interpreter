use token::Token;

use crate::{lexer::Lexer, token::TokenKind};
mod lexer;
mod token;

fn main() {
    let mut l = Lexer::new("let x = 1");
    let mut t: Token;

    loop {
        t = l.next_token();
        println!("token literal: {}, token: {}", t.literal, t.token);
        if t.token == TokenKind::ILLEGAL {
            // break;
            println!("illegal");
            return;
        }
    }
}
