use nu_ansi_term::{Color, Style};
use reedline::{Highlighter, StyledText};

use crate::lexer::utils::{is_alphabetic, is_alphanumeric, is_quote, is_standard_symbol};

pub struct IceHighlighter {
    identifier: Style,
    keyword: Style,
    number: Style,
    string: Style,
    symbol: Style,
    method: Style,
}

impl IceHighlighter {
    pub fn new() -> Self {
        Self {
            identifier: Style::new().fg(Color::White),
            keyword: Style::new().fg(Color::LightBlue),
            method: Style::new().fg(Color::Cyan),
            symbol: Style::new().fg(Color::LightCyan),
            number: Style::new().fg(Color::LightRed),
            string: Style::new().fg(Color::LightGreen),
        }
    }
}

impl Highlighter for IceHighlighter {
    fn highlight(&self, line: &str, _cursor: usize) -> StyledText {
        let mut tokens = line.chars().peekable();
        let mut buffer = vec![];

        while let Some(char) = tokens.next() {
            if is_standard_symbol(char) {
                buffer.push((self.symbol, char.to_string()));
            } else if is_quote(char) {
                let quote = char;
                buffer.push((self.string, char.to_string()));

                while let Some(char) = tokens.next() {
                    buffer.push((self.string, char.to_string()));

                    if char == quote {
                        break;
                    }
                }
            } else if is_alphabetic(char) {
                let mut current = String::from(char);

                while let Some(char) = tokens.peek() {
                    if is_alphanumeric(*char) {
                        current += &char.to_string();
                        tokens.next();
                    } else {
                        break;
                    }
                }

                let is_keyword = match current.as_str() {
                    "set" | "true" | "false" | "null" | "and" | "or" | "if" | "else" | "match"
                    | "for" | "while" | "loop" | "in" | "break" | "continue" | "function"
                    | "lambda" | "return" => true,
                    _ => false,
                };

                let style = match is_keyword {
                    true => self.keyword,
                    false => match tokens.peek() {
                        Some(&'(') => self.method,
                        _ => self.identifier,
                    },
                };

                buffer.push((style, current));
            } else if char.is_ascii_digit() {
                buffer.push((self.number, char.to_string()));
            } else {
                buffer.push((Style::new(), char.to_string()))
            }
        }

        StyledText { buffer }
    }
}
