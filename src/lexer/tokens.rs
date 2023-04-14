use super::utils::Position;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Number(f64),
    String(String),

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
    Modulo,
    Asterix,
    Slash,
    Comma,
    Semicolon,
    Dot,
    LeftParenthesis,
    RighParenethesis,
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

    LineBreak,
    Eof,
}

impl TokenType {
    pub fn is_line_break(&self) -> bool {
        matches!(self, Self::LineBreak)
    }

    pub fn is_skipable(&self) -> bool {
        matches!(self, TokenType::LineBreak | TokenType::Semicolon)
    }

    pub fn is_and(&self) -> bool {
        matches!(self, TokenType::And)
    }

    pub fn is_or(&self) -> bool {
        matches!(self, TokenType::Or)
    }

    pub fn is_eof(&self) -> bool {
        matches!(self, TokenType::Eof)
    }

    pub fn is_equality(&self) -> bool {
        matches!(self, TokenType::EqualEqual | TokenType::BangEqual)
    }

    pub fn is_plus_min_mod(&self) -> bool {
        matches!(self, TokenType::Plus | TokenType::Minus | TokenType::Modulo)
    }

    pub fn is_mutl_div(&self) -> bool {
        matches!(self, TokenType::Asterix | TokenType::Slash)
    }

    pub fn is_binary_operator(&self) -> bool {
        self.is_comparaison() || self.is_mutl_div() || self.is_plus_min_mod()
    }

    pub fn is_unary(&self) -> bool {
        matches!(self, TokenType::Bang | TokenType::Minus)
    }

    pub fn is_identifier(&self) -> bool {
        matches!(self, TokenType::Identifier(_))
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
