use std::{iter::Peekable, slice::Iter};

use super::{
    ast::{Expression, Statement},
    error::ParsingError,
};
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

    pub fn parse(&mut self) -> Result<Vec<Statement>, ParsingError> {
        let mut nodes: Vec<Statement> = Vec::new();

        while let Some(current_token) = self.tokens.next() {
            if current_token.value == TokenType::Eof {
                break;
            }

            let expr = self.parse_statement();

            nodes.push(expr?);
        }

        Ok(nodes)
    }

    fn clone_token(&self) -> Token {
        self.current_token.clone()
    }

    fn advance(&mut self) -> &'a Token {
        self.tokens.next().unwrap()
    }

    fn parse_statement(&mut self) -> Result<Statement, ParsingError> {
        let statement = match self.current_token.value {
            TokenType::Set => self.parse_variable_declaration()?,
            _ => {
                let expr = self.parse_expression()?;
                Statement::ExpressionStatement(expr)
            }
        };

        Ok(statement)
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, ParsingError> {
        todo!()
    }

    fn parse_expression(&mut self) -> Result<Expression, ParsingError> {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Result<Expression, ParsingError> {
        let expr = self.parse_comparaison()?;

        if self.current_token.value.is_equality() {
            let operator = self.clone_token();
            self.current_token = self.advance();

            if self.current_token.value == TokenType::Eof {
                return Err(ParsingError::MissingRightOperand(operator));
            }

            let right = self.parse_equality()?;

            return Ok(Expression::BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn parse_comparaison(&mut self) -> Result<Expression, ParsingError> {
        let expr = self.parse_term()?;

        if self.current_token.value.is_comparaison() {
            let operator = self.clone_token();
            self.current_token = self.advance();

            if self.current_token.value == TokenType::Eof {
                return Err(ParsingError::MissingRightOperand(operator));
            }

            let right = self.parse_comparaison()?;

            return Ok(Expression::BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expression, ParsingError> {
        let expr = self.parse_factor()?;

        if self.current_token.value.is_plus_min() {
            let operator = self.clone_token();
            self.current_token = self.advance();

            if self.current_token.value == TokenType::Eof {
                return Err(ParsingError::MissingRightOperand(operator));
            }

            let right = self.parse_term()?;

            return Ok(Expression::BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expression, ParsingError> {
        let expr = self.parse_unary()?;

        if self.current_token.value.is_mutl_div() {
            let operator = self.clone_token();
            self.current_token = self.advance();

            if self.current_token.value == TokenType::Eof {
                return Err(ParsingError::MissingRightOperand(operator));
            }

            let right = self.parse_factor()?;

            return Ok(Expression::BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn parse_unary(&mut self) -> Result<Expression, ParsingError> {
        if self.current_token.value.is_unary() {
            let operator = self.clone_token();
            self.current_token = self.advance();
            let right = self.parse_unary()?;

            return Ok(Expression::UnaryExpression {
                operator,
                operand: Box::new(right),
            });
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expression, ParsingError> {
        let expr = match self.current_token.value {
            TokenType::LeftParenthese => {
                self.current_token = self.advance();
                let expr = self.parse_expression()?;
                let token = self.clone_token();

                if self.current_token.value != TokenType::RighParenethese {
                    return Err(ParsingError::MissingParenthese(token));
                }

                Ok(expr)
            }
            _ => {
                if self.current_token.value.is_literal() {
                    let token = self.clone_token();
                    Ok(Expression::Literal(token))
                } else if self.current_token.value.is_binary_operator() {
                    Err(ParsingError::MissingLeftOperand(self.clone_token()))
                } else {
                    Err(ParsingError::UnexpectedToken(self.clone_token()))
                }
            }
        };

        self.current_token = self.advance();

        expr
    }
}
