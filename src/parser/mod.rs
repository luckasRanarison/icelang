pub mod ast;
pub mod error;

use self::{ast::*, error::ParsingError};
use crate::lexer::tokens::{Token, TokenType};
use std::{iter::Peekable, slice::Iter};

pub struct Parser<'a> {
    pub tokens: Peekable<Iter<'a, Token>>,
    pub current_token: &'a Token,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens: tokens.iter().peekable(),
            current_token: tokens.first().unwrap(), // assuming existing EOF
        }
    }

    fn clone_token(&self) -> Token {
        self.current_token.clone()
    }

    fn peek(&mut self) -> &Token {
        self.tokens.peek().unwrap()
    }

    fn advance(&mut self) {
        self.current_token = self.tokens.next().unwrap();
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, ParsingError> {
        self.advance();
        let mut nodes: Vec<Statement> = Vec::new();

        while !self.current_token.value.is_eof() {
            if self.current_token.value.is_line_break() {
                self.advance();
            }
            nodes.push(self.parse_statement()?);
        }

        Ok(nodes)
    }

    fn parse_statement(&mut self) -> Result<Statement, ParsingError> {
        let statement = match self.current_token.value {
            TokenType::Set => self.parse_variable_declaration()?,
            TokenType::Identifier(_) => self.parse_assignement()?,
            _ => Statement::ExpressionStatement(self.parse_expression()?),
        };

        Ok(statement)
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, ParsingError> {
        self.advance();
        let name = match &self.current_token.value {
            TokenType::Identifier(_) => self.clone_token(),
            _ => return Err(ParsingError::ExpectedIdentifier(self.clone_token())),
        };
        self.advance();
        if self.current_token.value != TokenType::Equal {
            return Err(ParsingError::MissingAssignment(self.clone_token()));
        }
        self.advance();
        let value = self.parse_expression()?;
        let declaration = Statement::VariableDeclaration(Declaration { name, value });

        Ok(declaration)
    }

    fn parse_assignement(&mut self) -> Result<Statement, ParsingError> {
        let next_type = &self.peek().value;
        if *next_type != TokenType::Equal {
            let expr = self.parse_expression()?;
            return Ok(Statement::ExpressionStatement(expr));
        }
        let name = self.clone_token();
        self.advance();
        self.advance();
        let value = self.parse_expression()?;
        let assignement = Statement::VariableAssignement(Assignement { name, value });

        Ok(assignement)
    }

    fn parse_expression(&mut self) -> Result<Expression, ParsingError> {
        Ok(self.parse_or()?)
    }

    fn parse_or(&mut self) -> Result<Expression, ParsingError> {
        let expression = self.parse_and()?;

        if self.current_token.value.is_or() {
            let operator = self.clone_token();
            self.advance();
            if self.current_token.value.is_eof() {
                return Err(ParsingError::MissingRightOperand(operator));
            }
            let right = self.parse_or()?;

            return Ok(Expression::BinaryExpression(Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            }));
        }

        Ok(expression)
    }

    fn parse_and(&mut self) -> Result<Expression, ParsingError> {
        let expression = self.parse_eqality()?;

        if self.current_token.value.is_and() {
            let operator = self.clone_token();
            self.advance();
            if self.current_token.value.is_eof() {
                return Err(ParsingError::MissingRightOperand(operator));
            }
            let right = self.parse_and()?;

            return Ok(Expression::BinaryExpression(Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            }));
        }

        Ok(expression)
    }

    fn parse_eqality(&mut self) -> Result<Expression, ParsingError> {
        let expression = self.parse_comparaison()?;

        if self.current_token.value.is_equality() {
            let operator = self.clone_token();
            self.advance();
            if self.current_token.value.is_eof() {
                return Err(ParsingError::MissingRightOperand(operator));
            }
            let right = self.parse_eqality()?;

            return Ok(Expression::BinaryExpression(Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            }));
        }

        Ok(expression)
    }

    fn parse_comparaison(&mut self) -> Result<Expression, ParsingError> {
        let expression = self.parse_term()?;

        if self.current_token.value.is_comparaison() {
            let operator = self.clone_token();
            self.advance();
            if self.current_token.value.is_eof() {
                return Err(ParsingError::MissingRightOperand(operator));
            }
            let right = self.parse_comparaison()?;

            return Ok(Expression::BinaryExpression(Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            }));
        }

        Ok(expression)
    }

    fn parse_term(&mut self) -> Result<Expression, ParsingError> {
        let expression = self.parse_factor()?;

        if self.current_token.value.is_plus_min_mod() {
            let operator = self.clone_token();
            self.advance();
            if self.current_token.value.is_eof() {
                return Err(ParsingError::MissingRightOperand(operator));
            }
            let right = self.parse_term()?;

            return Ok(Expression::BinaryExpression(Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            }));
        }

        Ok(expression)
    }

    fn parse_factor(&mut self) -> Result<Expression, ParsingError> {
        let expression = self.parse_unary()?;

        if self.current_token.value.is_mutl_div() {
            let operator = self.clone_token();
            self.advance();
            if self.current_token.value.is_eof() {
                return Err(ParsingError::MissingRightOperand(operator));
            }
            let right = self.parse_factor()?;

            return Ok(Expression::BinaryExpression(Binary {
                left: Box::new(expression),
                operator,
                right: Box::new(right),
            }));
        }

        Ok(expression)
    }

    fn parse_unary(&mut self) -> Result<Expression, ParsingError> {
        if self.current_token.value.is_unary() {
            let operator = self.clone_token();
            self.advance();
            let operand = Box::new(self.parse_unary()?);

            return Ok(Expression::UnaryExpression(Unary { operator, operand }));
        }

        self.parse_primary()
    }

    fn parse_primary(&mut self) -> Result<Expression, ParsingError> {
        let token = self.clone_token();
        let expression = match &self.current_token.value {
            TokenType::Eof => return Err(ParsingError::UnexpedtedEndOfInput(token)),
            TokenType::LeftParenthesis => self.parse_group()?,
            TokenType::Identifier(_) => {
                let next_type = &self.peek().value;
                if next_type.is_literal() || next_type.is_identifier() {
                    return Err(ParsingError::UnexpectedToken(token));
                } else if next_type.is_skipable() {
                    self.advance()
                }

                Expression::VariableExpression(Variable { token })
            }
            _ => {
                if self.current_token.value.is_literal() {
                    let next_type = &self.peek().value;
                    if next_type.is_literal() || next_type.is_identifier() {
                        return Err(ParsingError::UnexpectedToken(token));
                    } else if next_type.is_skipable() {
                        self.advance()
                    }

                    Expression::LiteralExpression(Literal { token })
                } else if self.current_token.value.is_binary_operator() {
                    return Err(ParsingError::MissingLeftOperand(token));
                } else {
                    return Err(ParsingError::UnexpectedToken(token));
                }
            }
        };
        self.advance();

        Ok(expression)
    }

    fn parse_group(&mut self) -> Result<Expression, ParsingError> {
        self.advance();
        let expression = self.parse_expression()?;
        if self.current_token.value != TokenType::RighParenethesis {
            return Err(ParsingError::MissingAssignment(self.clone_token()));
        }

        Ok(expression)
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use crate::lexer::Lexer;

    #[test]
    fn test_precedence() {
        let expr = "2 * 3 + 5 / (2 + 2)";
        let exptected = "((2 * 3) + (5 / (2 + 2)))";
        let tokens = Lexer::new(expr).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let node = ast.first().unwrap();
        assert_eq!(node.to_string(), exptected);

        let expr = "true or false and false";
        let exptected = "(true or (false and false))";
        let tokens = Lexer::new(expr).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let node = ast.first().unwrap();
        assert_eq!(node.to_string(), exptected);

        let expr = "(2 + 3 * 5) / (2 + 2) != 4 and true or false";
        let exptected = "(((((2 + (3 * 5)) / (2 + 2)) != 4) and true) or false)";
        let tokens = Lexer::new(expr).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let node = ast.first().unwrap();
        assert_eq!(node.to_string(), exptected);
    }

    #[test]
    fn test_declaration() {
        let stmt = "set a = 2;";
        let exptected = "set a = 2";
        let tokens = Lexer::new(stmt).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let node = ast.first().unwrap();
        assert_eq!(node.to_string(), exptected);

        let stmt = "
            set a = 2
            2 * 2
            set b = 2 
        ";
        let exptected_a = "set a = 2";
        let exptected_b = "set b = 2";
        let tokens = Lexer::new(stmt).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let a = ast.first().unwrap();
        let b = ast.get(2).unwrap();
        assert_eq!(a.to_string(), exptected_a);
        assert_eq!(b.to_string(), exptected_b);
    }
}
