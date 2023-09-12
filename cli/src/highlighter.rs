use lexer::utils::{is_alphabetic, is_alphanumeric, is_quote, is_standard_symbol};
use nu_ansi_term::{Color, Style};
use reedline::{Highlighter, StyledText};

pub struct IceHighlighter {
    identifier: Style,
    keyword: Style,
    number: Style,
    string: Style,
    symbol: Style,
    method: Style,
    comment: Style,
}

impl IceHighlighter {
    pub fn new() -> Self {
        Self {
            identifier: Style::from(Color::White),
            keyword: Style::from(Color::LightBlue),
            method: Style::from(Color::Cyan),
            symbol: Style::from(Color::LightCyan),
            number: Style::from(Color::LightRed),
            string: Style::from(Color::LightGreen),
            comment: Style::from(Color::DarkGray),
        }
    }
}

impl Highlighter for IceHighlighter {
    fn highlight(&self, line: &str, _cursor: usize) -> StyledText {
        let mut tokens = line.chars().peekable();
        let mut buffer = vec![];

        while let Some(char) = tokens.next() {
            if is_standard_symbol(char) {
                if char == '-' {
                    buffer.push((self.symbol, char.to_string()));

                    if let Some(next) = tokens.peek() {
                        if *next == '-' {
                            buffer.last_mut().unwrap().0 = self.comment;

                            for next in tokens.by_ref() {
                                buffer.push((self.comment, next.to_string()));
                            }
                        }
                    }
                } else {
                    buffer.push((self.symbol, char.to_string()));
                }
            } else if is_quote(char) {
                let quote = char;
                buffer.push((self.string, char.to_string()));

                for char in tokens.by_ref() {
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

                let is_keyword = matches!(
                    current.as_str(),
                    "set"
                        | "true"
                        | "false"
                        | "null"
                        | "and"
                        | "or"
                        | "if"
                        | "else"
                        | "match"
                        | "for"
                        | "to"
                        | "while"
                        | "loop"
                        | "in"
                        | "break"
                        | "continue"
                        | "function"
                        | "lambda"
                        | "return"
                        | "self"
                );

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
