use crate::tokens::{Position, Token, TokenType};
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

            if ch.is_whitespace() {
                self.current_pos.start = self.current_pos.end + 1;

                if ch == '\n' {
                    self.current_pos.line += 1;
                    self.current_pos.start = 0;
                }

                continue;
            }

            self.current_lexeme += &ch.to_string();

            if is_standard_symbol(ch) {
                let token = self.create_symbol_token();
                tokens.push(token);
            }

            self.current_lexeme = String::new();
            self.current_pos.start = self.current_pos.end + 1;
        }

        let eof_token = Token::new(TokenType::Eof, String::new(), self.current_pos);
        tokens.push(eof_token);

        tokens
    }

    fn advance(&mut self) {
        self.chars.next();
        self.current_pos.end += 1;
    }

    fn create_symbol_token(&mut self) -> Token {
        if let Some(next_char) = self.chars.peek() {
            if is_standard_symbol(*next_char) {
                self.current_lexeme += &next_char.to_string();
                self.advance();
            }
        }

        let lexeme = self.current_lexeme.clone();
        let token_type = match lexeme.as_str() {
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

        let token = Token::new(token_type, lexeme, self.current_pos);

        token
    }
}

fn is_standard_symbol(ch: char) -> bool {
    let symbols = "+-*/(){}.,;!<>=";
    symbols.contains(ch)
}
