use token::Token;

use crate::{lexer::Lexer, token::TokenKind};
mod lexer;
mod token;

fn main() {
    let mut l = Lexer::new("let five = 5;");
    let mut t: Token;

    loop {
        t = l.next_token();
        println!("token literal: {}, token kind: {}", t.literal, t.kind);

        if t.kind == TokenKind::ILLEGAL {
            println!("end");
            return;
        }
    }
}
