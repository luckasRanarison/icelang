use crate::tokenizer::tokens::{Position, Token, TokenType};
use crate::tokenizer::utils::*;
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

    pub fn tokenize(&mut self) -> Vec<Token> {
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
                _ => Token::new(TokenType::Invalid, String::new(), self.current_pos),
            };

            tokens.push(token);

            self.current_lexeme = String::new();
            self.current_pos.start = self.current_pos.end + 1;
        }

        self.current_pos.end = self.current_pos.start;
        let eof_token = Token::new(TokenType::Eof, String::new(), self.current_pos);
        tokens.push(eof_token);

        tokens
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

    fn create_symbol_token(&mut self) -> Token {
        if let Some(next_char) = self.chars.peek() {
            if is_standard_symbol(*next_char) {
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
            _ => TokenType::Invalid,
        };

        let token = Token::new(token_type, self.current_lexeme.clone(), self.current_pos);

        token
    }

    fn create_string_token(&mut self, quote_char: char) -> Token {
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
            panic!("Unmatched quote")
        }

        let value = self.current_lexeme[1..self.current_lexeme.len() - 1].to_string();
        let token = Token::new(
            TokenType::String(value),
            self.current_lexeme.clone(),
            self.current_pos,
        );

        token
    }

    fn create_number_token(&mut self) -> Token {
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
                panic!("Invalid number value")
            }
        };

        token
    }

    fn create_keyword_or_identifer_token(&mut self) -> Token {
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

        token
    }
}
