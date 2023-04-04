#[derive(Debug, PartialEq)]
pub enum TokenType {
    Number(f64),
    String(String),
    Boolean(bool),

    Identifier(String),

    Var,
    Null,
    True,
    False,
    And,
    Or,
    If,
    Else,
    For,
    While,
    Fun,
    Return,

    Plus,
    Minus,
    Asterix,
    Slash,
    Quote,
    DoubleQuote,
    Comma,
    Semicolon,
    Dot,
    Comment,
    LeftParenthese,
    RighParenethese,
    LeftBrace,
    RightBrace,
    Bang,

    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Eof,
    Invalid,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Position {
    pub line: usize,
    pub start: usize,
    pub end: usize,
}

impl Position {
    pub fn new(line: usize, start: usize, end: usize) -> Self {
        Self { line, start, end }
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub value: TokenType,
    pub pos: Position,
}

impl Token {
    pub fn new(value: TokenType, pos: Position) -> Self {
        Self { value, pos }
    }
}
