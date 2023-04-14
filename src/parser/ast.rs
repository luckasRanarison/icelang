use std::fmt;

use crate::lexer::tokens::Token;

#[derive(Debug, Clone)]
pub enum Expression {
    LiteralExpression(Literal),
    VariableExpression(Variable),
    UnaryExpression(Unary),
    BinaryExpression(Binary),
    IfExpression(If),
    MatchExpression(Match),
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::LiteralExpression(e) => write!(f, "{e}"),
            Expression::VariableExpression(e) => write!(f, "{e}"),
            Expression::UnaryExpression(e) => write!(f, "{e}"),
            Expression::BinaryExpression(e) => write!(f, "{e}"),
            Expression::IfExpression(e) => write!(f, "{e}"),
            Expression::MatchExpression(e) => write!(f, "{e}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub token: Token,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token.lexeme)
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub token: Token,
}

impl fmt::Display for Variable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.token.lexeme)
    }
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub operator: Token,
    pub operand: Box<Expression>,
}

impl fmt::Display for Unary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}{})", self.operator.lexeme, self.operand)
    }
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Box<Expression>,
    pub operator: Token,
    pub right: Box<Expression>,
}

impl fmt::Display for Binary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({} {} {})", self.left, self.operator.lexeme, self.right)
    }
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Box<Expression>,
    pub true_branch: Box<Statement>,
    pub else_branch: Option<Box<Statement>>,
}

impl fmt::Display for If {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut else_branch = String::new();
        if let Some(branch) = &self.else_branch {
            else_branch.push_str(&format!(" else {}", branch))
        }
        write!(
            f,
            "if ({}) {}{}",
            self.condition, self.true_branch, else_branch
        )
    }
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: Vec<Box<Expression>>,
    pub block: Box<Statement>,
}

impl fmt::Display for MatchArm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        let mut iter = self.pattern.iter();
        if let Some(pattern) = iter.next() {
            s.push_str(&format!("{}", pattern));
            for pattern in iter {
                s.push_str(&format!(" | {}", pattern));
            }
        }
        write!(f, "[( {} ) {}]", s, self.block)
    }
}

#[derive(Debug, Clone)]
pub struct Match {
    pub pattern: Box<Expression>,
    pub arms: Vec<MatchArm>,
    pub default: Option<MatchArm>,
}

impl fmt::Display for Match {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for arm in &self.arms {
            s.push_str(&format!(" {}", arm))
        }
        if let Some(defalut) = &self.default {
            s.push_str(&format!(" {}", defalut));
        }
        write!(f, "match ({}){}", self.pattern, s)
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration(Declaration),
    VariableAssignement(Assignement),
    ExpressionStatement(Expression),
    BlockStatement(Block),
    WhileStatement(While),
    LoopStatement(Loop),
    BreakStatement(Break),
    ContinueStatement(Continue),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::VariableDeclaration(s) => write!(f, "{s}"),
            Statement::VariableAssignement(s) => write!(f, "{s}"),
            Statement::ExpressionStatement(s) => write!(f, "{s}"),
            Statement::BlockStatement(s) => write!(f, "{s}"),
            Statement::WhileStatement(s) => write!(f, "{s}"),
            Statement::BreakStatement(s) => write!(f, "{s}"),
            Statement::ContinueStatement(s) => write!(f, "{s}"),
            Statement::LoopStatement(s) => write!(f, "{s}"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        for statement in &self.statements {
            s.push_str(&format!(" {};", *statement))
        }
        write!(f, "{{{s} }}")
    }
}

#[derive(Debug, Clone)]
pub struct Declaration {
    pub name: Token,
    pub value: Expression,
}

impl fmt::Display for Declaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "set {} = {}", self.name.lexeme, self.value)
    }
}

#[derive(Debug, Clone)]
pub struct Assignement {
    pub name: Token,
    pub value: Expression,
}

impl fmt::Display for Assignement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} = {}", self.name.lexeme, self.value)
    }
}

#[derive(Debug, Clone)]
pub struct While {
    pub condition: Expression,
    pub block: Box<Statement>,
}

impl fmt::Display for While {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "while ({}) {}", self.condition, self.block)
    }
}

#[derive(Debug, Clone)]
pub struct Loop {
    pub block: Box<Statement>,
}

impl fmt::Display for Loop {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "loop {}", self.block)
    }
}

#[derive(Debug, Clone)]
pub struct Break {
    pub token: Token,
}

impl fmt::Display for Break {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "break")
    }
}

#[derive(Debug, Clone)]
pub struct Continue {
    pub token: Token,
}

impl fmt::Display for Continue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "continue")
    }
}
