use std::{iter::Peekable, slice::Iter};

use super::{ast::Expression, error::ParsingError};
use crate::tokenizer::tokens::{Token, TokenType};

pub struct Parser<'a> {
    pub tokens: Peekable<Iter<'a, Token>>,
    pub current_token: &'a Token,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens: tokens.iter().peekable(),
            current_token: tokens.first().unwrap(),
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Expression>, ParsingError> {
        let mut nodes: Vec<Expression> = Vec::new();

        while let Some(_) = self.tokens.next() {
            if self.current_token.value == TokenType::Eof {
                break;
            }

            let expr = self.parse_expression();

            nodes.push(expr?);
        }

        Ok(nodes)
    }

    fn clone_token(&self) -> Token {
        self.current_token.clone()
    }

    fn advance(&mut self) {
        self.current_token = self.tokens.next().unwrap();
    }

    fn parse_expression(&mut self) -> Result<Expression, ParsingError> {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Result<Expression, ParsingError> {
        let expr = self.parse_comparaison()?;

        match self.current_token.value {
            TokenType::BangEqual | TokenType::EqualEqual => {
                let operator = self.clone_token();
                self.advance();
                let right = self.parse_comparaison()?;

                Ok(Expression::BinaryExpression {
                    left: Box::new(expr),
                    operator,
                    right: Box::new(right),
                })
            }
            _ => Ok(expr),
        }
    }

    fn parse_comparaison(&mut self) -> Result<Expression, ParsingError> {
        let expr = self.parse_term()?;

        match self.current_token.value {
            TokenType::Greater
            | TokenType::GreaterEqual
            | TokenType::Less
            | TokenType::LessEqual => {
                let operator = self.clone_token();
                self.advance();
                let right = self.parse_term()?;

                Ok(Expression::BinaryExpression {
                    left: Box::new(expr),
                    operator,
                    right: Box::new(right),
                })
            }
            _ => Ok(expr),
        }
    }

    fn parse_term(&mut self) -> Result<Expression, ParsingError> {
        let expr = self.parse_factor()?;

        match self.current_token.value {
            TokenType::Plus | TokenType::Minus => {
                let operator = self.clone_token();
                self.advance();
                let right = self.parse_factor()?;

                Ok(Expression::BinaryExpression {
                    left: Box::new(expr),
                    operator,
                    right: Box::new(right),
                })
            }
            _ => Ok(expr),
        }
    }

    fn parse_factor(&mut self) -> Result<Expression, ParsingError> {
        let expr = self.parse_unary()?;

        match self.current_token.value {
            TokenType::Asterix | TokenType::Slash => {
                let operator = self.clone_token();
                self.advance();
                let right = self.parse_unary()?;

                Ok(Expression::BinaryExpression {
                    left: Box::new(expr),
                    operator,
                    right: Box::new(right),
                })
            }
            _ => Ok(expr),
        }
    }

    fn parse_unary(&mut self) -> Result<Expression, ParsingError> {
        match self.current_token.value {
            TokenType::Bang | TokenType::Minus => {
                let operator = self.clone_token();
                self.advance();
                let right = self.parse_unary()?;

                Ok(Expression::UnaryExpression {
                    operator,
                    operand: Box::new(right),
                })
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> Result<Expression, ParsingError> {
        let expr = match self.current_token.value {
            TokenType::Number(_)
            | TokenType::String(_)
            | TokenType::True
            | TokenType::False
            | TokenType::Null => Ok(Expression::Literal(self.clone_token())),
            TokenType::LeftParenthese => {
                self.advance();
                let expr = self.parse_expression();

                if self.current_token.value != TokenType::RighParenethese {
                    return Err(ParsingError::MissingParenthese(self.clone_token()));
                }

                expr
            }
            _ => Err(ParsingError::UnexpectedToken(self.clone_token())),
        };

        self.advance();

        expr
    }
}
