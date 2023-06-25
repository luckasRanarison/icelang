pub mod errors;
pub mod tokens;
pub mod utils;

use std::{iter::Peekable, str::Chars};

use self::{
    errors::LexicalError,
    tokens::{Token, TokenType},
    utils::*,
};

#[derive(Debug)]
pub struct Lexer<'a> {
    pub chars: Peekable<Chars<'a>>,
    pub current_pos: Position,
    pub current_lexeme: String,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            chars: source.chars().peekable(),
            current_pos: Position::new(1, 1, 1),
            current_lexeme: String::new(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexicalError> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(ch) = self.chars.next() {
            self.current_pos.col_end = self.current_pos.col_start;

            if ch == '\n' {
                tokens.push(Token::new(
                    TokenType::LineBreak,
                    String::from("\n"),
                    self.current_pos,
                ));
            }

            if self.is_skipable(ch) {
                continue;
            }

            self.current_lexeme += &ch.to_string();

            tokens.push(self.create_token(ch)?);

            self.current_lexeme = String::new();
            self.current_pos.col_start = self.current_pos.col_end + 1;
        }

        if self.current_pos.col_start != 1 {
            self.current_pos.col_start -= 1; // offstet of the last loop
        }
        self.current_pos.col_end = self.current_pos.col_start;

        tokens.push(Token::new(TokenType::Eof, String::new(), self.current_pos));

        Ok(tokens)
    }

    pub fn tokenize_with_err(&mut self) -> (Vec<Token>, Vec<LexicalError>) {
        let mut tokens: Vec<Token> = Vec::new();
        let mut errors: Vec<LexicalError> = Vec::new();

        while let Some(ch) = self.chars.next() {
            self.current_pos.col_end = self.current_pos.col_start;

            if ch == '\n' {
                tokens.push(Token::new(
                    TokenType::LineBreak,
                    String::from("\n"),
                    self.current_pos,
                ));
            }

            if self.is_skipable(ch) {
                continue;
            }

            self.current_lexeme += &ch.to_string();

            match self.create_token(ch) {
                Ok(token) => tokens.push(token),
                Err(error) => {
                    while let Some(ch) = self.chars.next() {
                        self.current_pos.col_end += 1;
                        self.current_pos.col_start = self.current_pos.col_end;
                        if ch == '\n' {
                            match error {
                                LexicalError::TrailingQuote(..) => {
                                    self.current_pos.line += 1;
                                    self.current_pos.col_end = 1;
                                }
                                _ => {
                                    tokens.push(Token::new(
                                        TokenType::LineBreak,
                                        String::from("\n"),
                                        self.current_pos,
                                    ));
                                    self.current_pos.line += 1;
                                    self.current_pos.col_end = 0; // offset
                                    break;
                                }
                            }
                        }
                    }
                    errors.push(error);
                }
            }

            self.current_lexeme = String::new();
            self.current_pos.col_start = self.current_pos.col_end + 1;
        }

        if self.current_pos.col_start != 1 {
            self.current_pos.col_start -= 1; // offstet of the last loop
        }
        self.current_pos.col_end = self.current_pos.col_start;

        tokens.push(Token::new(TokenType::Eof, String::new(), self.current_pos));

        (tokens, errors)
    }

    fn advance(&mut self) {
        self.chars.next();
        self.current_pos.col_end += 1;
    }

    fn is_skipable(&mut self, ch: char) -> bool {
        if ch == '-' {
            if let Some(next_char) = self.chars.peek() {
                if *next_char == '-' {
                    self.skip_comment();
                    return true;
                }
            }
        }

        if ch.is_whitespace() {
            if ch == '\n' {
                self.current_pos.line += 1;
                self.current_pos.col_start = 1;
            } else {
                self.current_pos.col_start = self.current_pos.col_end + 1;
            }

            return true;
        }

        return false;
    }

    fn skip_comment(&mut self) {
        while let Some(ch) = self.chars.next() {
            if ch == '\n' {
                self.current_pos.line += 1;
                self.current_pos.col_start = 1;
                break;
            }
        }
    }

    fn create_token(&mut self, ch: char) -> Result<Token, LexicalError> {
        match ch {
            ch if is_standard_symbol(ch) => self.create_symbol_token(),
            ch if is_quote(ch) => self.create_string_token(ch),
            ch if is_alphabetic(ch) => self.create_keyword_or_identifer_token(),
            ch if ch.is_ascii_digit() => self.create_number_token(),
            _ => {
                return Err(LexicalError::UnexpectedCharacter(
                    self.current_lexeme.clone(),
                    self.current_pos,
                ))
            }
        }
    }

    fn create_symbol_token(&mut self) -> Result<Token, LexicalError> {
        if let Some(next_char) = self.chars.peek() {
            if *next_char == '=' {
                self.current_lexeme += &next_char.to_string();
                self.advance();
            }
        }

        let token_type = match self.current_lexeme.as_str() {
            "+" => TokenType::Plus,
            "-" => TokenType::Minus,
            "%" => TokenType::Modulo,
            "*" => TokenType::Asterix,
            "/" => TokenType::Slash,
            "(" => TokenType::LeftParenthesis,
            ")" => TokenType::RighParenethesis,
            "{" => TokenType::LeftBrace,
            "}" => TokenType::RightBrace,
            "[" => TokenType::LeftBracket,
            "]" => TokenType::RightBracket,
            "." => TokenType::Dot,
            "," => TokenType::Comma,
            ";" => TokenType::Semicolon,
            ":" => TokenType::Colon,
            "=" => TokenType::Equal,
            "+=" => TokenType::PlusEqual,
            "-=" => TokenType::MinusEqaul,
            "*=" => TokenType::AsterixEqual,
            "/=" => TokenType::SlashEqual,
            "%=" => TokenType::ModuloEqual,
            "==" => TokenType::EqualEqual,
            "!" => TokenType::Bang,
            "!=" => TokenType::BangEqual,
            ">" => TokenType::Greater,
            ">=" => TokenType::GreaterEqual,
            "<" => TokenType::Less,
            "<=" => TokenType::LessEqual,
            _ => {
                return Err(LexicalError::UnexpectedCharacter(
                    self.current_lexeme.clone(),
                    self.current_pos,
                ))
            }
        };

        let token = Token::new(token_type, self.current_lexeme.clone(), self.current_pos);

        Ok(token)
    }

    fn create_string_token(&mut self, quote_char: char) -> Result<Token, LexicalError> {
        let mut closed = false;

        while let Some(next_char) = self.chars.peek() {
            if *next_char == '\\' {
                self.advance();
                if let Some(next_next_char) = self.chars.peek() {
                    let current_escape_char: String = format!("\\{}", next_next_char);
                    let escape_char = match *next_next_char {
                        'n' => "\n",
                        't' => "\t",
                        'r' => "\r",
                        '\'' => "\'",
                        '\"' => "\"",
                        '\\' => "\\",
                        _ => {
                            return Err(LexicalError::InvalidEscapeChar(
                                current_escape_char,
                                self.current_pos,
                            ))
                        }
                    };
                    self.current_lexeme += escape_char;
                    self.advance();
                } else {
                    break;
                };
            } else if *next_char == '\n' {
                break;
            } else if *next_char == quote_char {
                closed = true;
                self.current_lexeme += &next_char.to_string();
                self.advance();
                break;
            } else {
                self.current_lexeme += &next_char.to_string();
                self.advance();
            }
        }

        if !closed {
            return Err(LexicalError::TrailingQuote(quote_char, self.current_pos));
        }

        let value = self.current_lexeme[1..self.current_lexeme.len() - 1].to_string();
        let token = Token::new(
            TokenType::String(value),
            self.current_lexeme.clone(),
            self.current_pos,
        );

        Ok(token)
    }

    fn create_number_token(&mut self) -> Result<Token, LexicalError> {
        while let Some(next_char) = self.chars.peek() {
            if next_char.is_ascii_digit() || *next_char == '.' {
                self.current_lexeme += &next_char.to_string();
                self.advance();
            } else {
                break;
            }
        }

        let token = match self.current_lexeme.parse::<f64>() {
            Ok(value) => Token::new(
                TokenType::Number(value),
                self.current_lexeme.clone(),
                self.current_pos,
            ),
            Err(_) => {
                return Err(LexicalError::InvalidFloat(
                    self.current_lexeme.clone(),
                    self.current_pos,
                ))
            }
        };

        Ok(token)
    }

    fn create_keyword_or_identifer_token(&mut self) -> Result<Token, LexicalError> {
        while let Some(ch) = self.chars.peek() {
            if is_alphanumeric(*ch) {
                self.current_lexeme += &ch.to_string();
                self.advance();
            } else {
                break;
            }
        }

        let token_type = match self.current_lexeme.as_str() {
            "set" => TokenType::Set,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "null" => TokenType::Null,
            "and" => TokenType::And,
            "or" => TokenType::Or,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "match" => TokenType::Match,
            "for" => TokenType::For,
            "to" => TokenType::To,
            "while" => TokenType::While,
            "loop" => TokenType::Loop,
            "foreach" => TokenType::Foreach,
            "in" => TokenType::In,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "function" => TokenType::Function,
            "lambda" => TokenType::Lambda,
            "return" => TokenType::Return,
            _ => TokenType::Identifier(self.current_lexeme.clone()),
        };

        let token = Token::new(token_type, self.current_lexeme.clone(), self.current_pos);

        Ok(token)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compare_single_line_tokens() {
        let s = "set s = 'Hello World';";
        let mut lex = Lexer::new(s);
        let tokens = lex.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::Set, String::from("set"), Position::new(1, 1, 3)),
                Token::new(
                    TokenType::Identifier(String::from("s")),
                    String::from("s"),
                    Position::new(1, 5, 5)
                ),
                Token::new(TokenType::Equal, String::from("="), Position::new(1, 7, 7)),
                Token::new(
                    TokenType::String(String::from("Hello World")),
                    String::from("'Hello World'"),
                    Position::new(1, 9, 21)
                ),
                Token::new(
                    TokenType::Semicolon,
                    String::from(";"),
                    Position::new(1, 22, 22)
                ),
                Token::new(TokenType::Eof, String::new(), Position::new(1, 22, 22))
            ]
        )
    }

    #[test]
    fn compare_multiple_line_tokens() {
        let s = r#"function hello() {
    return "Hello World";
}"#;
        let mut lex = Lexer::new(s);
        let tokens = lex.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::new(
                    TokenType::Function,
                    String::from("function"),
                    Position::new(1, 1, 8)
                ),
                Token::new(
                    TokenType::Identifier(String::from("hello")),
                    String::from("hello"),
                    Position::new(1, 10, 14)
                ),
                Token::new(
                    TokenType::LeftParenthesis,
                    String::from("("),
                    Position::new(1, 15, 15)
                ),
                Token::new(
                    TokenType::RighParenethesis,
                    String::from(")"),
                    Position::new(1, 16, 16)
                ),
                Token::new(
                    TokenType::LeftBrace,
                    String::from("{"),
                    Position::new(1, 18, 18)
                ),
                Token::new(
                    TokenType::LineBreak,
                    String::from("\n"),
                    Position::new(1, 19, 19)
                ),
                Token::new(
                    TokenType::Return,
                    String::from("return"),
                    Position::new(2, 5, 10)
                ),
                Token::new(
                    TokenType::String(String::from("Hello World")),
                    String::from("\"Hello World\""),
                    Position::new(2, 12, 24)
                ),
                Token::new(
                    TokenType::Semicolon,
                    String::from(";"),
                    Position::new(2, 25, 25)
                ),
                Token::new(
                    TokenType::LineBreak,
                    String::from("\n"),
                    Position::new(2, 26, 26)
                ),
                Token::new(
                    TokenType::RightBrace,
                    String::from("}"),
                    Position::new(3, 1, 1)
                ),
                Token::new(TokenType::Eof, String::new(), Position::new(3, 1, 1)),
            ]
        )
    }

    #[test]
    fn test_tokenize_with_err() {
        let s = "12 + 1..2
^ unreachable
idk
'hi";
        let (tokens, errors) = Lexer::new(s).tokenize_with_err();
        assert_eq!(
            tokens,
            vec![
                Token::new(
                    TokenType::Number(12.0),
                    String::from("12"),
                    Position::new(1, 1, 2)
                ),
                Token::new(TokenType::Plus, String::from("+"), Position::new(1, 4, 4)),
                Token::new(
                    TokenType::LineBreak,
                    String::from("\n"),
                    Position::new(1, 10, 10)
                ),
                Token::new(
                    TokenType::LineBreak,
                    String::from("\n"),
                    Position::new(2, 14, 14)
                ),
                Token::new(
                    TokenType::Identifier(String::from("idk")),
                    String::from("idk"),
                    Position::new(3, 1, 3)
                ),
                Token::new(
                    TokenType::LineBreak,
                    String::from("\n"),
                    Position::new(3, 4, 4)
                ),
                Token::new(TokenType::Eof, String::new(), Position::new(4, 3, 3))
            ]
        );
        assert_eq!(
            errors,
            vec![
                LexicalError::InvalidFloat(String::from("1..2"), Position::new(1, 6, 9)),
                LexicalError::UnexpectedCharacter(String::from("^"), Position::new(2, 1, 1)),
                LexicalError::TrailingQuote('\'', Position::new(4, 1, 3))
            ]
        )
    }
}
