pub struct Lexer {
    position: usize,
    read_position: usize,
    ch: u8,
    input: Vec<u8>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            position: 0,
            read_position: 0,
            ch: 0,
            input: input.into_bytes(),
        };
        lexer.read_char();

        lexer
    }

    fn read_char(&mut self) {
        let l = self;
        if l.read_position >= l.input.len() {
            l.ch = 0;
        } else {
            l.ch = l.input[l.read_position];
        }

        l.position = l.read_position;
        l.read_position += 1;
    }
}
