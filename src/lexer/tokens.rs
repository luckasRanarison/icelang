use super::utils::Position;

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Number(f64),
    String(String),

    Identifier(String),

    Set,
    Null,
    True,
    False,
    And,
    Or,
    If,
    Else,
    Match,
    For,
    To,
    While,
    Loop,
    Foreach,
    In,
    Break,
    Continue,
    Function,
    Lambda,
    Return,

    Plus,
    Minus,
    Modulo,
    Asterix,
    Slash,
    Comma,
    Semicolon,
    Colon,
    Dot,
    LeftParenthesis,
    RighParenethesis,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Bang,
    Equal,
    PlusEqual,
    MinusEqaul,
    AsterixEqual,
    SlashEqual,
    ModuloEqual,

    BangEqual,
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

    pub fn is_assignment(&self) -> bool {
        match self {
            TokenType::Equal
            | TokenType::PlusEqual
            | TokenType::MinusEqaul
            | TokenType::AsterixEqual
            | TokenType::SlashEqual
            | TokenType::ModuloEqual => true,
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

    pub fn is_keyword(&self) -> bool {
        match self {
            TokenType::Set
            | TokenType::Null
            | TokenType::True
            | TokenType::False
            | TokenType::And
            | TokenType::Or
            | TokenType::If
            | TokenType::Else
            | TokenType::For
            | TokenType::To
            | TokenType::While
            | TokenType::Loop
            | TokenType::Foreach
            | TokenType::In
            | TokenType::Break
            | TokenType::Continue
            | TokenType::Function
            | TokenType::Return => true,
            _ => false,
        }
    }

    pub fn is_symbol(&self) -> bool {
        !self.is_identifier()
            && !self.is_keyword()
            && !self.is_line_break()
            && !self.is_eof()
            && !matches!(self, TokenType::String(_) | TokenType::Number(_))
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
