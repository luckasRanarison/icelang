pub mod ast;
pub mod error;

use self::{ast::*, error::ParsingErrorKind};

use error::ParsingError;
use lexer::tokens::{Token, TokenType};
use std::{iter::Peekable, slice::Iter, vec};

pub struct Parser<'a> {
    pub tokens: Peekable<Iter<'a, Token>>,
    pub current_token: &'a Token,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Self {
            tokens: tokens.iter().peekable(),
            current_token: tokens.first().unwrap(), // assuming existing EOF
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Statement>, ParsingError> {
        self.advance();

        let mut nodes: Vec<Statement> = Vec::new();

        while !self.current_token.value.is_eof() {
            let statement = self.parse_statement()?;
            match &statement {
                Statement::FunctionDeclaration(_) => nodes.insert(0, statement),
                _ => nodes.push(statement),
            }
        }

        Ok(nodes)
    }

    fn clone_token(&self) -> Token {
        self.current_token.clone()
    }

    fn clone_lexeme(&self) -> String {
        self.current_token.lexeme.clone()
    }

    fn peek(&mut self) -> &Token {
        self.tokens.peek().unwrap()
    }

    fn advance(&mut self) {
        self.current_token = self.tokens.next().unwrap();

        while self.current_token.value.is_skipable() {
            self.current_token = self.tokens.next().unwrap();
        }
    }

    fn skip_line(&mut self) {
        while self.current_token.value.is_line_break() {
            self.current_token = self.tokens.next().unwrap();
        }
    }

    fn parse_statement(&mut self) -> Result<Statement, ParsingError> {
        let statement = match self.current_token.value {
            TokenType::Set => self.parse_variable_declaration()?,
            TokenType::LeftBrace => self.parse_block()?,
            TokenType::For => self.parse_for()?,
            TokenType::While => self.parse_while()?,
            TokenType::Loop => self.parse_loop()?,
            TokenType::Break => self.parse_break()?,
            TokenType::Continue => self.parse_continue()?,
            TokenType::Function => self.parse_function()?,
            TokenType::Return => self.parse_return()?,
            _ => Statement::ExpressionStatement(self.parse_expression()?),
        };

        Ok(statement)
    }

    fn parse_variable_declaration(&mut self) -> Result<Statement, ParsingError> {
        self.advance();
        let name = match &self.current_token.value {
            TokenType::Identifier(_) => self.clone_token(),
            _ => {
                return Err(ParsingError::new(
                    ParsingErrorKind::ExpectedIdentifier(self.clone_lexeme()),
                    self.current_token.pos,
                ))
            }
        };
        self.advance();

        if self.current_token.value != TokenType::Equal {
            return Err(ParsingError::new(
                ParsingErrorKind::MissingAssignment,
                self.current_token.pos,
            ));
        }

        self.advance();
        let value = self.parse_expression()?;
        let declaration = Statement::VariableDeclaration(Declaration { name, value });

        Ok(declaration)
    }

    fn parse_block(&mut self) -> Result<Statement, ParsingError> {
        // TODO: find a better way to tell if it's an object
        let mut next_three = self
            .tokens
            .clone()
            .filter(|token| !token.value.is_line_break());

        if let Some(token) = next_three.next() {
            if token.value == TokenType::RightBrace {
                return Ok(Statement::ExpressionStatement(self.parse_expression()?));
            }
        }

        if let Some(token) = next_three.next() {
            if token.value == TokenType::Colon || token.value == TokenType::Comma {
                return Ok(Statement::ExpressionStatement(self.parse_expression()?));
            }
        }

        if self.current_token.value.is_eof() {
            return Err(ParsingError::new(
                ParsingErrorKind::UnexpedtedEndOfInput,
                self.current_token.pos,
            ));
        }

        self.advance();

        let mut statements: Vec<Statement> = vec![];
        while self.current_token.value != TokenType::RightBrace {
            if self.current_token.value.is_eof() {
                return Err(ParsingError::new(
                    ParsingErrorKind::MissingClosingBrace,
                    self.current_token.pos,
                ));
            }

            let statement = self.parse_statement()?;
            match &statement {
                Statement::FunctionDeclaration(_) => statements.insert(0, statement),
                _ => statements.push(statement),
            }
        }
        let statement = Statement::BlockStatement(Block { statements });
        self.advance();

        Ok(statement)
    }

    fn parse_for(&mut self) -> Result<Statement, ParsingError> {
        self.advance();
        let key = if self.current_token.value.is_identifier() {
            self.clone_token()
        } else {
            return Err(ParsingError::new(
                ParsingErrorKind::ExpectedIdentifier(self.clone_lexeme()),
                self.current_token.pos,
            ));
        };
        self.advance();

        let mut value = None;
        if self.current_token.value == TokenType::Comma {
            self.advance();
            if !self.current_token.value.is_identifier() {
                return Err(ParsingError::new(
                    ParsingErrorKind::ExpectedIdentifier(self.clone_lexeme()),
                    self.current_token.pos,
                ));
            }
            value = Some(self.clone_token());
            self.advance();
        }

        if self.current_token.value != TokenType::In {
            return Err(ParsingError::new(
                ParsingErrorKind::ExpectedIn(self.clone_lexeme()),
                self.current_token.pos,
            ));
        }
        self.advance();

        let iterable_token = self.clone_token();
        let iterable = self.parse_expression()?;

        if self.current_token.value != TokenType::LeftBrace {
            return Err(ParsingError::new(
                ParsingErrorKind::ExpectedLeftBrace(self.clone_lexeme()),
                self.current_token.pos,
            ));
        }

        let block = Box::new(self.parse_block()?);

        let for_statement = Statement::ForStatement(For {
            variable: (key, value),
            iterable,
            iterable_token,
            block,
        });

        Ok(for_statement)
    }

    fn parse_while(&mut self) -> Result<Statement, ParsingError> {
        self.advance();
        let condition = self.parse_expression()?;

        if self.current_token.value != TokenType::LeftBrace {
            return Err(ParsingError::new(
                ParsingErrorKind::ExpectedLeftBrace(self.clone_lexeme()),
                self.current_token.pos,
            ));
        }

        let block = Box::new(self.parse_block()?);
        let statement = Statement::WhileStatement(While { condition, block });

        Ok(statement)
    }

    fn parse_loop(&mut self) -> Result<Statement, ParsingError> {
        self.advance();

        if self.current_token.value != TokenType::LeftBrace {
            return Err(ParsingError::new(
                ParsingErrorKind::ExpectedLeftBrace(self.clone_lexeme()),
                self.current_token.pos,
            ));
        }

        let block = Box::new(self.parse_block()?);
        let statement = Statement::LoopStatement(Loop { block });

        Ok(statement)
    }

    fn parse_break(&mut self) -> Result<Statement, ParsingError> {
        let token = self.clone_token();
        let statement = Statement::BreakStatement(Break { token });
        self.advance();

        Ok(statement)
    }

    fn parse_continue(&mut self) -> Result<Statement, ParsingError> {
        let token = self.clone_token();
        let statement = Statement::ContinueStatement(Continue { token });
        self.advance();

        Ok(statement)
    }

    fn parse_function(&mut self) -> Result<Statement, ParsingError> {
        self.advance();
        let token = Some(self.clone_token());

        if !self.current_token.value.is_identifier() {
            return Err(ParsingError::new(
                ParsingErrorKind::ExpectedIdentifier(self.clone_lexeme()),
                self.current_token.pos,
            ));
        }

        let parameter = self.get_function_param()?;

        if self.current_token.value != TokenType::LeftBrace {
            return Err(ParsingError::new(
                ParsingErrorKind::ExpectedLeftBrace(self.clone_lexeme()),
                self.current_token.pos,
            ));
        }

        let body = Box::new(self.parse_block()?);
        let declaration = Statement::FunctionDeclaration(FunctionDeclaration {
            token,
            parameter,
            body,
        });

        Ok(declaration)
    }

    fn parse_return(&mut self) -> Result<Statement, ParsingError> {
        let token = self.clone_token();
        self.advance();
        let expression = self.parse_expression()?;
        let statement = Statement::ReturnStatement(Return { token, expression });

        Ok(statement)
    }

    fn parse_expression(&mut self) -> Result<Expression, ParsingError> {
        self.parse_assignment()
    }

    fn parse_assignment(&mut self) -> Result<Expression, ParsingError> {
        let target_token = self.clone_token();
        let expression = self.parse_or()?;

        if self.current_token.value.is_assignment() {
            let token = self.clone_token();
            self.advance();
            let value = self.parse_assignment()?;

            if let Expression::VariableExpression(_)
            | Expression::IndexExpression(_)
            | Expression::PropAccess(_) = expression
            {
                let assignment = Expression::AssignementExpression(Assign {
                    left: Box::new(expression),
                    token,
                    value: Box::new(value),
                });
                return Ok(assignment);
            }

            return Err(ParsingError::new(
                ParsingErrorKind::InvalidAssignment,
                target_token.pos,
            ));
        }

        Ok(expression)
    }

    fn parse_or(&mut self) -> Result<Expression, ParsingError> {
        let expression = self.parse_and()?;

        if self.current_token.value.is_or() {
            let operator = self.clone_token();
            self.advance();

            if self.current_token.value.is_eof() {
                return Err(ParsingError::new(
                    ParsingErrorKind::MissingRightOperand(operator.lexeme),
                    operator.pos,
                ));
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
                return Err(ParsingError::new(
                    ParsingErrorKind::MissingRightOperand(operator.lexeme),
                    operator.pos,
                ));
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
                return Err(ParsingError::new(
                    ParsingErrorKind::MissingRightOperand(operator.lexeme),
                    operator.pos,
                ));
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
        let expression = self.parse_range()?;

        if self.current_token.value.is_comparaison() {
            let operator = self.clone_token();
            self.advance();

            if self.current_token.value.is_eof() {
                return Err(ParsingError::new(
                    ParsingErrorKind::MissingRightOperand(operator.lexeme),
                    operator.pos,
                ));
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

    fn parse_range(&mut self) -> Result<Expression, ParsingError> {
        let expression = self.parse_term()?;

        if self.current_token.value.is_range() {
            let operator = self.clone_token();
            self.advance();

            if self.current_token.value.is_eof() {
                return Err(ParsingError::new(
                    ParsingErrorKind::MissingRightOperand(operator.lexeme),
                    operator.pos,
                ));
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

    fn parse_term(&mut self) -> Result<Expression, ParsingError> {
        let expression = self.parse_factor()?;

        if self.current_token.value.is_term_op() {
            let operator = self.clone_token();
            self.advance();

            if self.current_token.value.is_eof() {
                return Err(ParsingError::new(
                    ParsingErrorKind::MissingRightOperand(operator.lexeme),
                    operator.pos,
                ));
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

        if self.current_token.value.is_factor_op() {
            let operator = self.clone_token();
            self.advance();

            if self.current_token.value.is_eof() {
                return Err(ParsingError::new(
                    ParsingErrorKind::MissingRightOperand(operator.lexeme),
                    operator.pos,
                ));
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
        let mut expression = match token.value {
            TokenType::Eof => {
                return Err(ParsingError::new(
                    ParsingErrorKind::UnexpedtedEndOfInput,
                    self.current_token.pos,
                ))
            }
            TokenType::If => self.parse_if()?,
            TokenType::Match => self.parse_match()?,
            TokenType::Lambda => self.parse_lambda()?,
            TokenType::LeftParenthesis => self.parse_group()?,
            TokenType::LeftBracket => self.parse_array()?,
            TokenType::LeftBrace => self.parse_object()?,
            TokenType::Identifier(_) => {
                let next_token = self.peek();
                if next_token.value.is_literal() || next_token.value.is_identifier() {
                    return Err(ParsingError::new(
                        ParsingErrorKind::UnexpectedToken(next_token.lexeme.clone()),
                        self.current_token.pos,
                    ));
                }
                Expression::VariableExpression(Variable {
                    token: token.clone(),
                })
            }
            _ => {
                if self.current_token.value.is_literal() {
                    let next_token = self.peek();
                    if next_token.value.is_literal() || next_token.value.is_identifier() {
                        return Err(ParsingError::new(
                            ParsingErrorKind::UnexpectedToken(next_token.lexeme.clone()),
                            self.current_token.pos,
                        ));
                    }
                    Expression::LiteralExpression(Literal {
                        token: token.clone(),
                    })
                } else if self.current_token.value.is_binary_operator() {
                    return Err(ParsingError::new(
                        ParsingErrorKind::MissingLeftOperand(token.lexeme),
                        self.current_token.pos,
                    ));
                } else {
                    return Err(ParsingError::new(
                        ParsingErrorKind::UnexpectedToken(token.lexeme),
                        self.current_token.pos,
                    ));
                }
            }
        };
        let has_block = matches!(
            &token.value,
            TokenType::If | TokenType::Match | TokenType::Lambda
        );

        if !has_block {
            self.advance();
        }

        loop {
            expression = match self.current_token.value {
                TokenType::LeftBracket => self.parse_index(expression)?,
                TokenType::LeftParenthesis => self.parse_call(expression, None)?,
                TokenType::Dot => self.parse_prop_access(expression)?,
                _ => break,
            }
        }

        Ok(expression)
    }

    fn parse_group(&mut self) -> Result<Expression, ParsingError> {
        self.advance();
        let expression = self.parse_expression()?;

        if self.current_token.value != TokenType::RighParenethesis {
            return Err(ParsingError::new(
                ParsingErrorKind::MissingClosingParenthesis,
                self.current_token.pos,
            ));
        }

        Ok(expression)
    }

    fn parse_array(&mut self) -> Result<Expression, ParsingError> {
        self.advance();
        let mut items: Vec<Expression> = vec![];

        while self.current_token.value != TokenType::RightBracket {
            if self.current_token.value.is_eof() {
                return Err(ParsingError::new(
                    ParsingErrorKind::UnexpedtedEndOfInput,
                    self.current_token.pos,
                ));
            }

            items.push(self.parse_expression()?);
            self.skip_line();

            if self.current_token.value != TokenType::RightBracket
                && self.current_token.value != TokenType::Comma
            {
                return Err(ParsingError::new(
                    ParsingErrorKind::ExpectedComma(self.clone_lexeme()),
                    self.current_token.pos,
                ));
            }

            if self.current_token.value == TokenType::Comma {
                self.advance();
            }
        }
        let array_expression = Expression::ArrayExpression(Array { items });

        Ok(array_expression)
    }

    fn parse_object(&mut self) -> Result<Expression, ParsingError> {
        self.advance();
        let mut props: Vec<(Token, Expression)> = vec![];

        while self.current_token.value != TokenType::RightBrace {
            if self.current_token.value.is_eof() {
                return Err(ParsingError::new(
                    ParsingErrorKind::UnexpedtedEndOfInput,
                    self.current_token.pos,
                ));
            }

            if self.current_token.value.is_symbol() || self.current_token.lexeme.contains('.') {
                return Err(ParsingError::new(
                    ParsingErrorKind::InvalidProp(self.clone_lexeme()),
                    self.current_token.pos,
                ));
            }

            let name = self.clone_token();
            self.advance();

            let value = match self.current_token.value {
                TokenType::Colon => {
                    self.advance();
                    self.parse_expression()?
                }
                TokenType::Comma | TokenType::RightBrace => {
                    Expression::VariableExpression(Variable {
                        token: name.clone(),
                    })
                }
                _ => {
                    return Err(ParsingError::new(
                        ParsingErrorKind::ExpectedColon(self.clone_lexeme()),
                        self.current_token.pos,
                    ));
                }
            };

            props.push((name, value));
            self.skip_line();

            if self.current_token.value != TokenType::RightBrace
                && self.current_token.value != TokenType::Comma
            {
                return Err(ParsingError::new(
                    ParsingErrorKind::ExpectedComma(self.clone_lexeme()),
                    self.current_token.pos,
                ));
            }

            if self.current_token.value == TokenType::Comma {
                self.advance()
            }
        }
        let object_expression = Expression::ObjectExpression(Object { props });

        Ok(object_expression)
    }

    fn parse_index(&mut self, expression: Expression) -> Result<Expression, ParsingError> {
        let token = self.clone_token();
        self.advance();
        let index = Box::new(self.parse_expression()?);

        if self.current_token.value != TokenType::RightBracket {
            return Err(ParsingError::new(
                ParsingErrorKind::MissingClosingBracket,
                self.current_token.pos,
            ));
        }

        self.advance();
        let index_expression = Expression::IndexExpression(Index {
            token,
            expression: Box::new(expression),
            index,
        });

        Ok(index_expression)
    }

    fn parse_prop_access(&mut self, expression: Expression) -> Result<Expression, ParsingError> {
        let token = self.clone_token();
        self.advance();

        if !self.current_token.value.is_identifier() && !self.current_token.value.is_keyword() {
            return Err(ParsingError::new(
                ParsingErrorKind::InvalidProp(self.clone_lexeme()),
                self.current_token.pos,
            ));
        }

        let prop = self.clone_token();
        self.advance();

        let prop_access = Expression::PropAccess(Access {
            token,
            expression: Box::new(expression.clone()),
            prop,
        });

        if self.current_token.value == TokenType::LeftParenthesis {
            let call = self.parse_call(prop_access, Some(expression))?;
            return Ok(call);
        }

        Ok(prop_access)
    }

    fn parse_if(&mut self) -> Result<Expression, ParsingError> {
        self.advance();
        let condition = Box::new(self.parse_expression()?);

        if self.current_token.value != TokenType::LeftBrace {
            return Err(ParsingError::new(
                ParsingErrorKind::ExpectedLeftBrace(self.clone_lexeme()),
                self.current_token.pos,
            ));
        }

        let true_branch = Box::new(self.parse_block()?);
        let else_branch = if self.current_token.value == TokenType::Else {
            self.advance();
            match self.current_token.value {
                TokenType::If => Some(Box::new(self.parse_statement()?)),
                _ => {
                    if self.current_token.value != TokenType::LeftBrace {
                        return Err(ParsingError::new(
                            ParsingErrorKind::ExpectedLeftBrace(self.clone_lexeme()),
                            self.current_token.pos,
                        ));
                    }
                    Some(Box::new(self.parse_block()?))
                }
            }
        } else {
            None
        };
        let if_exression = Expression::IfExpression(If {
            condition,
            true_branch,
            else_branch,
        });

        Ok(if_exression)
    }

    fn parse_match(&mut self) -> Result<Expression, ParsingError> {
        self.advance();
        let pattern = Box::new(self.parse_expression()?);

        if self.current_token.value != TokenType::LeftBrace {
            return Err(ParsingError::new(
                ParsingErrorKind::ExpectedLeftBrace(self.clone_lexeme()),
                self.current_token.pos,
            ));
        }

        self.advance();
        let mut arms: Vec<MatchArm> = vec![];
        let mut default = None;

        while self.current_token.value != TokenType::RightBrace {
            if self.current_token.value.is_eof() {
                return Err(ParsingError::new(
                    ParsingErrorKind::MissingClosingBrace,
                    self.current_token.pos,
                ));
            }

            if self.current_token.lexeme == "_" {
                default = Some(self.parse_match_arm()?);
            } else {
                arms.push(self.parse_match_arm()?);
            }

            if self.current_token.value != TokenType::Comma {
                return Err(ParsingError::new(
                    ParsingErrorKind::ExpectedComma(self.clone_lexeme()),
                    self.current_token.pos,
                ));
            }

            self.advance();
        }
        self.advance();
        let expression = Expression::MatchExpression(Match {
            pattern,
            arms,
            default,
        });

        Ok(expression)
    }

    fn parse_match_arm(&mut self) -> Result<MatchArm, ParsingError> {
        let mut pattern: Vec<Box<Expression>> = vec![];
        loop {
            let expr = match &self.current_token.value {
                TokenType::Colon => break,
                TokenType::Eof => {
                    return Err(ParsingError::new(
                        ParsingErrorKind::UnexpedtedEndOfInput,
                        self.current_token.pos,
                    ))
                }
                TokenType::Comma => {
                    self.advance();
                    continue;
                }
                _ => {
                    if self.current_token.lexeme == "_" && self.peek().value != TokenType::Colon {
                        return Err(ParsingError::new(
                            ParsingErrorKind::UnexpectedToken(self.peek().lexeme.clone()),
                            self.current_token.pos,
                        ));
                    }

                    Box::new(self.parse_expression()?)
                }
            };

            pattern.push(expr);
        }

        if pattern.is_empty() {
            return Err(ParsingError::new(
                ParsingErrorKind::MissingArmExpression,
                self.current_token.pos,
            ));
        }

        self.advance();
        let block = Box::new(self.parse_statement()?);
        let arm = MatchArm { pattern, block };

        Ok(arm)
    }

    fn parse_call(
        &mut self,
        expression: Expression,
        object: Option<Expression>,
    ) -> Result<Expression, ParsingError> {
        let token = self.clone_token();
        self.advance();
        let mut arguments: Vec<Expression> = vec![];

        while self.current_token.value != TokenType::RighParenethesis {
            if self.current_token.value.is_eof() {
                return Err(ParsingError::new(
                    ParsingErrorKind::MissingClosingParenthesis,
                    self.current_token.pos,
                ));
            }

            arguments.push(self.parse_expression()?);

            if self.current_token.value == TokenType::Comma {
                self.advance();
            }
        }
        self.advance();

        let object = object.map(Box::new);
        let call = Expression::FunctionCall(Call {
            token,
            caller: Box::new(expression),
            object,
            arguments,
        });

        Ok(call)
    }

    fn parse_lambda(&mut self) -> Result<Expression, ParsingError> {
        let parameter = self.get_function_param()?;
        let body = Box::new(self.parse_statement()?);
        let lambda = Expression::LambdaFunction(Lambda { parameter, body });

        Ok(lambda)
    }

    fn get_function_param(&mut self) -> Result<Vec<Token>, ParsingError> {
        self.advance();
        if self.current_token.value != TokenType::LeftParenthesis {
            return Err(ParsingError::new(
                ParsingErrorKind::ExpectedLeftParenthesis(self.clone_lexeme()),
                self.current_token.pos,
            ));
        }
        self.advance();

        let mut parameter: Vec<Token> = vec![];

        while self.current_token.value != TokenType::RighParenethesis {
            if self.current_token.value.is_eof() {
                return Err(ParsingError::new(
                    ParsingErrorKind::MissingClosingParenthesis,
                    self.current_token.pos,
                ));
            }

            if !self.current_token.value.is_identifier() {
                return Err(ParsingError::new(
                    ParsingErrorKind::ExpectedParameter(self.clone_lexeme()),
                    self.current_token.pos,
                ));
            }

            parameter.push(self.clone_token());
            self.advance();

            let check = matches!(
                self.current_token.value,
                TokenType::Comma | TokenType::RighParenethesis
            );

            if !check {
                return Err(ParsingError::new(
                    ParsingErrorKind::ExpectedComma(self.clone_lexeme()),
                    self.current_token.pos,
                ));
            }

            if self.current_token.value == TokenType::Comma {
                self.advance();
            }
        }

        self.advance();
        Ok(parameter)
    }
}

#[cfg(test)]
mod test {
    use super::Parser;
    use lexer::Lexer;

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

    #[test]
    fn test_block() {
        let stmt = "
            {
                set a = 1;
                set b = 2;
                a + b
            }
        ";
        let expected = "{ set a = 1; set b = 2; (a + b); }";
        let tokens = Lexer::new(stmt).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let node = ast.first().unwrap();
        assert_eq!(node.to_string(), expected);
    }

    #[test]
    fn test_if() {
        let stmt = "
            if (false) {
                'unreachable'
            } else if (false and true) {
                'unreachable'
            } else {
                set message = 'return this';
                message
            }
        ";
        let expected = "if (false) { 'unreachable'; } else if ((false and true)) { 'unreachable'; } else { set message = 'return this'; message; }";
        let tokens = Lexer::new(stmt).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let node = ast.first().unwrap();
        assert_eq!(node.to_string(), expected);
    }

    #[test]
    fn test_while() {
        let stmt = "
            set i = 0;
            while (true) {
                i = i + 1;
                if (i == 3) {
                    continue;
                }
                
                if (i % 3) == 0 {
                    break;
                }
            }
        ";
        let expected = "while (true) { i = (i + 1); if ((i == 3)) { continue; }; if (((i % 3) == 0)) { break; }; }";
        let tokens = Lexer::new(stmt).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let node = ast.get(1).unwrap();
        assert_eq!(node.to_string(), expected);
    }

    #[test]
    fn test_loop() {
        let stmt = "
            loop {
                if (true) {
                    break;
                }
            }
        ";
        let expected = "loop { if (true) { break; }; }";
        let tokens = Lexer::new(stmt).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let node = ast.first().unwrap();
        assert_eq!(node.to_string(), expected);
    }

    #[test]
    fn test_match() {
        let stmt = "
            set a = 2;
            set b = match a {
                0, 1: {
                    false
                },
                2: true, 
                _: {
                    'unreachable' 
                },
            };
        ";
        let expected =
            "set b = match (a) [( 0 | 1 ) { false; }] [( 2 ) true] [( _ ) { 'unreachable'; }]";
        let tokens = Lexer::new(stmt).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let node = ast.get(1).unwrap();
        assert_eq!(node.to_string(), expected);
    }

    #[test]
    fn test_array_index() {
        let stmt = "
            !array[2][3] / [1, 2, 3][0]
        ";
        let expected = "((!array[2][3]) / [1, 2, 3][0])";
        let tokens = Lexer::new(stmt).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let node = ast.first().unwrap();
        assert_eq!(node.to_string(), expected);
    }

    #[test]
    fn test_function_declaration() {
        let stmt = "
            function hello(name) {
                set message = 'Hello' + name;
                return message;
            }
        ";
        let expected = "function hello(name) { set message = ('Hello' + name); return message; }";
        let tokens = Lexer::new(stmt).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let node = ast.first().unwrap();
        assert_eq!(node.to_string(), expected);
    }

    #[test]
    fn test_function_call() {
        let stmt = "
            f(true, 2 * 3 + 1, [1, 2])
        ";
        let expected = "f(true, ((2 * 3) + 1), [1, 2])";
        let tokens = Lexer::new(stmt).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let node = ast.first().unwrap();
        assert_eq!(node.to_string(), expected);
    }

    #[test]
    fn test_lambda() {
        let stmt = "
            set hello = lambda() {
                'Hello World'
            }
        ";
        let expected = "set hello = lambda() { 'Hello World'; }";
        let tokens = Lexer::new(stmt).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let node = ast.first().unwrap();
        assert_eq!(node.to_string(), expected);
    }

    #[test]
    fn test_object() {
        let stmt = "
            set my_object = {
                my_prop: 'stuff',
                my_another_prop: 3 * 2 + 1,
                my_method: lambda() {
                    return 'stuff';
                }
            }
        ";
        let expected = "set my_object = { my_prop: 'stuff', my_another_prop: ((3 * 2) + 1), my_method: lambda() { return 'stuff'; } }";
        let tokens = Lexer::new(stmt).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let node = ast.first().unwrap();
        assert_eq!(node.to_string(), expected);
    }

    #[test]
    fn test_prop_access() {
        let stmt = "
            object.prop.method()
        ";
        let expected = "object.prop.method()";
        let tokens = Lexer::new(stmt).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let node = ast.first().unwrap();
        assert_eq!(node.to_string(), expected);
    }

    #[test]
    fn test_for() {
        let stmt = "
            for i in [0, 1, 2] {
                print(i) 
            }
        ";
        let expected = "for i in [0, 1, 2] { print(i); }";
        let tokens = Lexer::new(stmt).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let node = ast.first().unwrap();
        assert_eq!(node.to_string(), expected);

        let stmt = "
            for key, value in object {
                print(key, value) 
            }
        ";
        let expected = "for key, value in object { print(key, value); }";
        let tokens = Lexer::new(stmt).tokenize().unwrap();
        let ast = Parser::new(&tokens).parse().unwrap();
        let node = ast.first().unwrap();
        assert_eq!(node.to_string(), expected);
    }
}
