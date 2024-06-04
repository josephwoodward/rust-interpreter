#![allow(dead_code)]

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Program { statements: vec![] }
    }
}

pub struct Statement {}
