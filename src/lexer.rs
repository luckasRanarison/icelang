use crate::tokens::{Position, Token};

#[derive(Debug)]
pub struct Lexer {
    pub source: Vec<char>,
    pub current_pos: Position,
    pub idx: usize,
}

impl Lexer {
    pub fn new(source: String) -> Self {
        Self {
            source: source.chars().collect(),
            current_pos: Position::new(1, 0, 0),
            idx: 0,
        }
    }

    pub fn tokenize(&mut self) -> Vec<Token> {
        todo!()
    }
}
