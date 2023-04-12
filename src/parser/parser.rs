use std::{iter::Peekable, slice::Iter};

use super::{
    ast::{Expression, Statement},
    error::ParsingError,
};
use crate::tokenizer::tokens::{Token, TokenType};

pub struct Parser<'a> {
    pub tokens: Peekable<Iter<'a, Token>>,
    pub current_token: &'a Token,
    pub is_parsing_block: bool, // flag to keep track of unexplicit returns
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a Vec<Token>) -> Self {
        Self {
            tokens: tokens.iter().peekable(),
            current_token: tokens.first().unwrap(), // assuming existing EOF
            is_parsing_block: false,
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, ParsingError> {
        self.advance();
        let mut nodes: Vec<Statement> = Vec::new();

        while self.current_token.value != TokenType::Eof {
            nodes.push(self.parse_statement()?);
        }

        Ok(nodes)
    }

    fn clone_token(&self) -> Token {
        self.current_token.clone()
    }

    fn advance(&mut self) {
        if self.current_token.value != TokenType::Eof {
            self.current_token = self.tokens.next().unwrap();
        }
    }

    fn peek_token(&mut self) -> &Token {
        self.tokens.peek().unwrap()
    }

    fn parse_statement(&mut self) -> Result<Statement, ParsingError> {
        let statement = match self.current_token.value {
            TokenType::Set => self.parse_variable_declaration()?,
            TokenType::Identifier(_) => self.parse_assignement()?,
            TokenType::While => self.parse_while()?,
            _ => {
                let expr = self.parse_expression()?;

                if !self.is_parsing_block && self.current_token.value == TokenType::Semicolon {
                    self.advance();
                }

                Statement::ExpressionStatement(expr)
            }
        };

        Ok(statement)
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, ParsingError> {
        self.advance();
        let token = self.clone_token();
        let name = match &self.current_token.value {
            TokenType::Identifier(value) => value.clone(),
            _ => return Err(ParsingError::ExpectedIdentifier(self.clone_token())),
        };

        self.advance();
        if self.current_token.value != TokenType::Equal {
            return Err(ParsingError::MissingAssignment(self.clone_token()));
        }

        self.advance();
        let value = self.parse_expression()?;

        if self.current_token.value != TokenType::Semicolon {
            return Err(ParsingError::MissingSemicolon(self.clone_token()));
        }
        self.advance();

        Ok(Statement::VariableDeclaration { token, name, value })
    }

    fn parse_assignement(&mut self) -> Result<Statement, ParsingError> {
        if self.tokens.peek().unwrap().value != TokenType::Equal {
            let expr = self.parse_expression()?;

            if !self.is_parsing_block && self.current_token.value == TokenType::Semicolon {
                self.advance();
            }

            return Ok(Statement::ExpressionStatement(expr));
        }

        let token = self.clone_token();
        let name = self.current_token.lexeme.clone();

        self.advance();
        self.advance();

        let value = self.parse_expression()?;

        if self.current_token.value != TokenType::Semicolon {
            return Err(ParsingError::MissingSemicolon(self.clone_token()));
        }
        self.advance();

        Ok(Statement::VariableAssignement { token, name, value })
    }

    fn parse_block(&mut self) -> Result<Expression, ParsingError> {
        self.is_parsing_block = true;
        let token = self.clone_token();
        let mut statements: Vec<Statement> = Vec::new();
        let mut return_expr = None;
        self.advance();

        while self.current_token.value != TokenType::RightBrace {
            match self.current_token.value {
                TokenType::Eof => return Err(ParsingError::MissingClosingBrace(token)),
                _ => {
                    let statement = self.parse_statement()?;

                    if let Statement::ExpressionStatement(expr) = statement.clone() {
                        if self.current_token.value == TokenType::Semicolon {
                            self.advance();
                            statements.push(statement);
                        } else {
                            return_expr = Some(Box::new(expr));
                        }
                    } else {
                        statements.push(statement)
                    }
                }
            }
        }
        self.is_parsing_block = false;
        let block = Expression::BlockExpression {
            statements,
            return_expr,
        };

        Ok(block)
    }

    fn parse_if_expression(&mut self) -> Result<Expression, ParsingError> {
        self.advance();
        let condition = Box::new(self.parse_expression()?);

        if self.current_token.value != TokenType::LeftBrace {
            return Err(ParsingError::ExpectedLeftBrace(self.clone_token()));
        }

        let true_branch = Box::new(self.parse_block()?);
        let else_branch = if self.peek_token().value == TokenType::Else {
            self.advance();
            match self.peek_token().value {
                TokenType::If => {
                    self.advance();
                    Some(Box::new(self.parse_expression()?))
                }
                _ => {
                    self.advance();
                    if self.current_token.value != TokenType::LeftBrace {
                        return Err(ParsingError::ExpectedLeftBrace(self.clone_token()));
                    }
                    Some(Box::new(self.parse_block()?))
                }
            }
        } else {
            None
        };
        let if_expr = Expression::IfExpression {
            condition,
            true_branch,
            else_branch,
        };

        Ok(if_expr)
    }

    fn parse_while(&mut self) -> Result<Statement, ParsingError> {
        self.advance();
        let condition = Box::new(self.parse_expression()?);

        if self.current_token.value != TokenType::LeftBrace {
            return Err(ParsingError::ExpectedLeftBrace(self.clone_token()));
        }
        let block = Box::new(self.parse_block()?);
        let while_expr = Statement::WhileStatement { condition, block };
        self.advance();

        Ok(while_expr)
    }

    fn parse_group(&mut self) -> Result<Expression, ParsingError> {
        self.advance();
        let expr = self.parse_expression()?;
        let token = self.clone_token();

        if self.current_token.value != TokenType::RighParenethese {
            return Err(ParsingError::MissingParenthesis(token));
        }

        Ok(expr)
    }

    fn parse_expression(&mut self) -> Result<Expression, ParsingError> {
        Ok(self.parse_or()?)
    }

    fn parse_or(&mut self) -> Result<Expression, ParsingError> {
        let expr = self.parse_and()?;

        if self.current_token.value == TokenType::Or {
            let operator = self.clone_token();
            self.advance();

            if self.current_token.value == TokenType::Eof {
                return Err(ParsingError::MissingRightOperand(operator));
            }

            let right = self.parse_or()?;

            return Ok(Expression::BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn parse_and(&mut self) -> Result<Expression, ParsingError> {
        let expr = self.parse_equality()?;

        if self.current_token.value == TokenType::And {
            let operator = self.clone_token();
            self.advance();

            if self.current_token.value == TokenType::Eof {
                return Err(ParsingError::MissingRightOperand(operator));
            }

            let right = self.parse_and()?;

            return Ok(Expression::BinaryExpression {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn parse_equality(&mut self) -> Result<Expression, ParsingError> {
        let expr = self.parse_comparaison()?;

        if self.current_token.value.is_equality() {
            let operator = self.clone_token();
            self.advance();

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
            self.advance();

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
            self.advance();

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
            self.advance();

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
            self.advance();
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
            TokenType::Eof => {
                return Err(ParsingError::UnexpedtedEndOfInput(self.clone_token()));
            }

            TokenType::LeftBrace => Ok(self.parse_block()?),
            TokenType::LeftParenthese => Ok(self.parse_group()?),
            TokenType::If => Ok(self.parse_if_expression()?),
            TokenType::Identifier(_) => {
                let token = self.clone_token();
                let next_token_type = &self.peek_token().value;
                if next_token_type.is_literal() || next_token_type.is_identifier() {
                    return Err(ParsingError::UnexpectedToken(token));
                }

                Ok(Expression::VariableExpression(token))
            }
            _ => {
                if self.current_token.value.is_literal() {
                    let token = self.clone_token();
                    let next_token_type = &self.peek_token().value;
                    if next_token_type.is_literal() || next_token_type.is_identifier() {
                        return Err(ParsingError::UnexpectedToken(token));
                    }

                    Ok(Expression::Literal(token))
                } else if self.current_token.value.is_binary_operator() {
                    return Err(ParsingError::MissingLeftOperand(self.clone_token()));
                } else {
                    return Err(ParsingError::UnexpectedToken(self.clone_token()));
                }
            }
        };

        self.advance();

        expr
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use crate::tokenizer::lexer::Lexer;

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
    fn test_block() {
        let input = "
            {
                set x = 10;
                set y = 5;
                x + y  
            }";
        let expected = "{ set x = 10; set y = 5; (x + y) }";
        let tokens = Lexer::new(input).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let node = ast.first().unwrap();
        assert_eq!(node.to_string(), expected);
    }

    #[test]
    fn test_if_expression() {
        let input = "
            if (3 > 4) {
                if (true) {
                    'don\\'t return this';
                    'return this'
                }
            } else {
                'unreachable'
            }      
        ";
        let expected = "if ((3 > 4)) { if (true) { 'don't return this'; 'return this' } } else { 'unreachable' }";
        let tokens = Lexer::new(input).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let node = ast.first().unwrap();
        assert_eq!(node.to_string(), expected);
    }
}
