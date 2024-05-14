use crate::lexer::Lexer;
mod lexer;
mod token;

fn main() {
    let l = Lexer::new("hello".to_string());
}
