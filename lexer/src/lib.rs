pub mod errors;
pub mod tokens;
pub mod utils;

use std::{iter::Peekable, str::Chars};

use errors::LexicalError;

use self::{
    errors::LexicalErrorKind,
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
            current_pos: Position::new(0, 0, 0, 0),
            current_lexeme: String::new(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexicalError> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(ch) = self.chars.next() {
            self.current_pos.line_start = self.current_pos.line_end;
            self.current_pos.col_end = self.current_pos.col_start;

            if self.is_skipable(ch) {
                continue;
            }

            self.current_lexeme += &ch.to_string();

            tokens.push(self.create_token(ch)?);

            self.current_lexeme.clear();

            if !is_linebreak(ch) {
                self.current_pos.col_start = self.current_pos.col_end + 1;
            }
        }

        if self.current_pos.col_start > 0 {
            self.current_pos.col_start -= 1; // offstet of the last loop
        }

        self.current_pos.col_end = self.current_pos.col_start;

        let eof_token = Token::new(TokenType::Eof, String::new(), self.current_pos);

        tokens.push(eof_token);

        Ok(tokens)
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

        if ch.is_whitespace() && !is_linebreak(ch) {
            self.current_pos.col_start = self.current_pos.col_end + 1;
            return true;
        }

        return false;
    }

    fn skip_comment(&mut self) {
        while let Some(ch) = self.chars.next() {
            if ch == '\n' {
                self.current_pos.line_end += 1;
                self.current_pos.col_start = 0;
                break;
            }
        }
    }

    fn create_token(&mut self, ch: char) -> Result<Token, LexicalError> {
        match ch {
            ch if is_linebreak(ch) => self.create_linebreak(),
            ch if is_standard_symbol(ch) => self.create_symbol_token(),
            ch if is_quote(ch) => self.create_string_token(ch),
            ch if is_alphabetic(ch) => self.create_keyword_or_identifer_token(),
            ch if ch.is_ascii_digit() => self.create_number_token(),
            _ => {
                return Err(LexicalError::new(
                    LexicalErrorKind::UnexpectedCharacter(self.current_lexeme.clone()),
                    self.current_pos,
                ))
            }
        }
    }

    fn create_linebreak(&mut self) -> Result<Token, LexicalError> {
        let token = Token::new(TokenType::LineBreak, String::from("\n"), self.current_pos);

        self.current_pos.line_end += 1;
        self.current_pos.col_start = 0;

        Ok(token)
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
                return Err(LexicalError::new(
                    LexicalErrorKind::UnexpectedCharacter(self.current_lexeme.clone()),
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
                            return Err(LexicalError::new(
                                LexicalErrorKind::InvalidEscapeChar(current_escape_char),
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
            return Err(LexicalError::new(
                LexicalErrorKind::TrailingQuote(quote_char),
                self.current_pos,
            ));
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
                return Err(LexicalError::new(
                    LexicalErrorKind::InvalidFloat(self.current_lexeme.clone()),
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
                Token::new(
                    TokenType::Set,
                    String::from("set"),
                    Position::new(0, 0, 0, 2)
                ),
                Token::new(
                    TokenType::Identifier(String::from("s")),
                    String::from("s"),
                    Position::new(0, 4, 0, 4)
                ),
                Token::new(
                    TokenType::Equal,
                    String::from("="),
                    Position::new(0, 6, 0, 6)
                ),
                Token::new(
                    TokenType::String(String::from("Hello World")),
                    String::from("'Hello World'"),
                    Position::new(0, 8, 0, 20)
                ),
                Token::new(
                    TokenType::Semicolon,
                    String::from(";"),
                    Position::new(0, 21, 0, 21)
                ),
                Token::new(TokenType::Eof, String::new(), Position::new(0, 21, 0, 21))
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
                    Position::new(0, 0, 0, 7)
                ),
                Token::new(
                    TokenType::Identifier(String::from("hello")),
                    String::from("hello"),
                    Position::new(0, 9, 0, 13)
                ),
                Token::new(
                    TokenType::LeftParenthesis,
                    String::from("("),
                    Position::new(0, 14, 0, 14)
                ),
                Token::new(
                    TokenType::RighParenethesis,
                    String::from(")"),
                    Position::new(0, 15, 0, 15)
                ),
                Token::new(
                    TokenType::LeftBrace,
                    String::from("{"),
                    Position::new(0, 17, 0, 17)
                ),
                Token::new(
                    TokenType::LineBreak,
                    String::from("\n"),
                    Position::new(0, 18, 0, 18)
                ),
                Token::new(
                    TokenType::Return,
                    String::from("return"),
                    Position::new(1, 4, 1, 9)
                ),
                Token::new(
                    TokenType::String(String::from("Hello World")),
                    String::from("\"Hello World\""),
                    Position::new(1, 11, 1, 23)
                ),
                Token::new(
                    TokenType::Semicolon,
                    String::from(";"),
                    Position::new(1, 24, 1, 24)
                ),
                Token::new(
                    TokenType::LineBreak,
                    String::from("\n"),
                    Position::new(1, 25, 1, 25)
                ),
                Token::new(
                    TokenType::RightBrace,
                    String::from("}"),
                    Position::new(2, 0, 2, 0)
                ),
                Token::new(TokenType::Eof, String::new(), Position::new(2, 0, 2, 0)),
            ]
        )
    }
}
