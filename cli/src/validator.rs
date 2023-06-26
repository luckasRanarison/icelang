use lexer::Lexer;
use parser::{error::ParsingErrorKind, Parser};
use reedline::{ValidationResult, Validator};

pub struct IceValidator {}

impl IceValidator {
    pub fn new() -> Self {
        Self {}
    }
}

impl Validator for IceValidator {
    fn validate(&self, line: &str) -> reedline::ValidationResult {
        let tokens = Lexer::new(line).tokenize();
        let tokens = match tokens {
            Ok(value) => value,
            Err(_) => {
                return ValidationResult::Complete;
            }
        };
        let nodes = Parser::new(&tokens).parse();
        match nodes {
            Ok(_) => ValidationResult::Complete,
            Err(err) => match err.kind {
                ParsingErrorKind::MissingParenthesis
                | ParsingErrorKind::MissingClosingBrace
                | ParsingErrorKind::ExpectedComma(_)
                | ParsingErrorKind::UnexpedtedEndOfInput => ValidationResult::Incomplete,
                _ => ValidationResult::Complete,
            },
        }
    }
}
