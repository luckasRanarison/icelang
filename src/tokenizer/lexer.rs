use super::errors::LexicalError;
use super::tokens::{Token, TokenType};
use super::utils::*;
use std::{iter::Peekable, str::Chars};

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
            current_pos: Position::new(1, 0, 0),
            current_lexeme: String::new(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<Token>, LexicalError> {
        let mut tokens: Vec<Token> = Vec::new();

        while let Some(ch) = self.chars.next() {
            self.current_pos.end = self.current_pos.start;

            if ch == '-' {
                if let Some(next_char) = self.chars.peek() {
                    if *next_char == '-' {
                        self.skip_comment();
                        continue;
                    }
                }
            }

            if ch.is_whitespace() {
                self.current_pos.start = self.current_pos.end + 1;

                if ch == '\n' {
                    self.current_pos.line += 1;
                    self.current_pos.start = 0;
                }

                continue;
            }

            self.current_lexeme += &ch.to_string();

            let token = match ch {
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
            };

            tokens.push(token?);

            self.current_lexeme = String::new();
            self.current_pos.start = self.current_pos.end + 1;
        }

        if self.current_pos.start != 0 {
            self.current_pos.start -= 1; // offstet of the last loop
        }
        self.current_pos.end = self.current_pos.start;

        let eof_token = Token::new(TokenType::Eof, String::new(), self.current_pos);
        tokens.push(eof_token);

        Ok(tokens)
    }

    fn advance(&mut self) {
        self.chars.next();
        self.current_pos.end += 1;
    }

    fn skip_comment(&mut self) {
        while let Some(ch) = self.chars.next() {
            if ch == '\n' {
                self.current_pos.line += 1;
                self.current_pos.start = 0;
                break;
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
            "*" => TokenType::Asterix,
            "/" => TokenType::Slash,
            "(" => TokenType::LeftParenthese,
            ")" => TokenType::RighParenethese,
            "{" => TokenType::LeftBrace,
            "}" => TokenType::RightBrace,
            "." => TokenType::Dot,
            "," => TokenType::Comma,
            ";" => TokenType::Semicolon,
            "=" => TokenType::Equal,
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
            if *next_char == '\n' {
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
            return Err(LexicalError::MismatchedParentheses(self.current_pos));
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
            "var" => TokenType::Var,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "null" => TokenType::Null,
            "and" => TokenType::And,
            "or" => TokenType::Or,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "for" => TokenType::For,
            "while" => TokenType::While,
            "fun" => TokenType::Fun,
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
        let s = "var s = 'Hello World';";
        let mut lex = Lexer::new(s);
        let tokens = lex.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::Var, String::from("var"), Position::new(1, 0, 2)),
                Token::new(
                    TokenType::Identifier(String::from("s")),
                    String::from("s"),
                    Position::new(1, 4, 4)
                ),
                Token::new(TokenType::Equal, String::from("="), Position::new(1, 6, 6)),
                Token::new(
                    TokenType::String(String::from("Hello World")),
                    String::from("'Hello World'"),
                    Position::new(1, 8, 20)
                ),
                Token::new(
                    TokenType::Semicolon,
                    String::from(";"),
                    Position::new(1, 21, 21)
                ),
                Token::new(TokenType::Eof, String::new(), Position::new(1, 21, 21))
            ]
        )
    }

    #[test]
    fn compare_multiple_line_tokens() {
        let s = r#"fun hello() {
    return "Hello World";
}"#;
        let mut lex = Lexer::new(s);
        let tokens = lex.tokenize().unwrap();

        assert_eq!(
            tokens,
            vec![
                Token::new(TokenType::Fun, String::from("fun"), Position::new(1, 0, 2)),
                Token::new(
                    TokenType::Identifier(String::from("hello")),
                    String::from("hello"),
                    Position::new(1, 4, 8)
                ),
                Token::new(
                    TokenType::LeftParenthese,
                    String::from("("),
                    Position::new(1, 9, 9)
                ),
                Token::new(
                    TokenType::RighParenethese,
                    String::from(")"),
                    Position::new(1, 10, 10)
                ),
                Token::new(
                    TokenType::LeftBrace,
                    String::from("{"),
                    Position::new(1, 12, 12)
                ),
                Token::new(
                    TokenType::Return,
                    String::from("return"),
                    Position::new(2, 4, 9)
                ),
                Token::new(
                    TokenType::String(String::from("Hello World")),
                    String::from("\"Hello World\""),
                    Position::new(2, 11, 23)
                ),
                Token::new(
                    TokenType::Semicolon,
                    String::from(";"),
                    Position::new(2, 24, 24)
                ),
                Token::new(
                    TokenType::RightBrace,
                    String::from("}"),
                    Position::new(3, 0, 0)
                ),
                Token::new(TokenType::Eof, String::new(), Position::new(3, 0, 0)),
            ]
        )
    }
}
