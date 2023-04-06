use super::utils::Position;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Number(f64),
    String(String),
    Boolean(bool),

    Identifier(String),

    Set,
    Freeze,
    Null,
    True,
    False,
    And,
    Or,
    If,
    Else,
    For,
    To,
    While,
    Loop,
    Foreach,
    In,
    Break,
    Continue,
    Function,
    Return,
    Expose,
    Import,
    Export,

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
    LeftBracket,
    RightBracket,
    Bang,

    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    Eof,
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
