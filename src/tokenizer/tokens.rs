use super::utils::Position;

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
    Comma,
    Semicolon,
    Dot,
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

#[derive(Debug, PartialEq)]
pub struct Token {
    pub value: TokenType,
    pub lexeme: String,
    pub pos: Position,
}

impl Token {
    pub fn new(value: TokenType, lexeme: String, pos: Position) -> Self {
        Self { value, lexeme, pos }
    }
}
