use reedline::{ValidationResult, Validator};

use crate::{
    lexer::Lexer,
    parser::{error::ParsingError, Parser},
};

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
            Err(err) => match err {
                ParsingError::MissingParenthesis(_)
                | ParsingError::MissingClosingBrace(_)
                | ParsingError::ExpectedComma(_)
                | ParsingError::UnexpedtedEndOfInput(_) => ValidationResult::Incomplete,
                _ => ValidationResult::Complete,
            },
        }
    }
}
