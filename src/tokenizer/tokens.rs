use super::utils::Position;

#[derive(Debug, PartialEq, Clone)]
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

impl TokenType {
    pub fn is_equality(&self) -> bool {
        match self {
            TokenType::EqualEqual | TokenType::BangEqual => true,
            _ => false,
        }
    }

    pub fn is_comparaison(&self) -> bool {
        match self {
            TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual => true,
            _ => false,
        }
    }

    pub fn is_unary(&self) -> bool {
        match self {
            TokenType::Bang | TokenType::Minus => true,
            _ => false,
        }
    }

    pub fn is_literal(&self) -> bool {
        match self {
            TokenType::String(_)
            | TokenType::Number(_)
            | TokenType::True
            | TokenType::False
            | TokenType::Null => true,
            _ => false,
        }
    }

    pub fn is_plus_min(&self) -> bool {
        match self {
            TokenType::Plus | TokenType::Minus => true,
            _ => false,
        }
    }

    pub fn is_mutl_div(&self) -> bool {
        match self {
            TokenType::Asterix | TokenType::Slash => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
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
