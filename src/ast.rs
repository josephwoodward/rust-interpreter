#![allow(dead_code)]

use crate::token::Token;
use serde::{Deserialize, Serialize};

pub struct Program {
    pub statements: Vec<Statement>,
}

impl Program {
    pub fn new() -> Self {
        Program { statements: vec![] }
    }
}

#[derive(Clone, Debug, Eq, Serialize, Deserialize, Hash, PartialEq)]
#[serde(untagged)]
pub enum Statement {
    Let(Let),
    Identifier { name: String },
}

#[derive(Clone, Debug, Eq, Serialize, Deserialize, Hash, PartialEq)]
#[serde(tag = "type")]
pub struct Let {
    pub identifier: Token, // rust can't do precise type with enum
}
