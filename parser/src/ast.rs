use std::fmt;

use lexer::tokens::Token;

#[derive(Debug, Clone)]
pub enum Expression {
    LiteralExpression(Literal),
    VariableExpression(Variable),
    AssignementExpression(Assign),
    ArrayExpression(Array),
    ObjectExpression(Object),
    IndexExpression(Index),
    PropAccess(Access),
    UnaryExpression(Unary),
    BinaryExpression(Binary),
    IfExpression(If),
    MatchExpression(Match),
    FunctionCall(Call),
    LambdaFunction(Lambda),
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
            Expression::ArrayExpression(e) => write!(f, "{e}"),
            Expression::IndexExpression(e) => write!(f, "{e}"),
            Expression::AssignementExpression(e) => write!(f, "{e}"),
            Expression::FunctionCall(e) => write!(f, "{e}"),
            Expression::LambdaFunction(e) => write!(f, "{e}"),
            Expression::ObjectExpression(e) => write!(f, "{e}"),
            Expression::PropAccess(e) => write!(f, "{e}"),
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
pub struct Assign {
    pub left: Box<Expression>,
    pub token: Token,
    pub value: Box<Expression>,
}

impl fmt::Display for Assign {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.left, self.token.lexeme, self.value)
    }
}

#[derive(Debug, Clone)]
pub struct Array {
    pub items: Vec<Expression>,
}

impl fmt::Display for Array {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        let mut iter = self.items.iter();
        if let Some(item) = iter.next() {
            s.push_str(&format!("{}", item));
            for item in iter {
                s.push_str(&format!(", {}", item));
            }
        }
        write!(f, "[{}]", s)
    }
}

#[derive(Debug, Clone)]
pub struct Object {
    pub props: Vec<(Token, Expression)>,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        let mut iter = self.props.iter();
        if let Some(item) = iter.next() {
            s.push_str(&format!("{}: {}", item.0.lexeme, item.1));
            for item in iter {
                s.push_str(&format!(", {}: {}", item.0.lexeme, item.1));
            }
        }
        write!(f, "{{ {} }}", s)
    }
}

#[derive(Debug, Clone)]
pub struct Index {
    pub token: Token,
    pub expression: Box<Expression>,
    pub index: Box<Expression>,
}

impl fmt::Display for Index {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}[{}]", self.expression, self.index)
    }
}

#[derive(Debug, Clone)]
pub struct Access {
    pub token: Token,
    pub expression: Box<Expression>,
    pub prop: Token,
}

impl fmt::Display for Access {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.expression, self.prop.lexeme)
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
pub struct Call {
    pub token: Token,
    pub object: Option<Box<Expression>>,
    pub caller: Box<Expression>,
    pub arguments: Vec<Expression>,
}

impl fmt::Display for Call {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        let mut iter = self.arguments.iter();
        if let Some(arg) = iter.next() {
            s.push_str(&format!("{}", arg));
            for arg in iter {
                s.push_str(&format!(", {}", arg));
            }
        }
        write!(f, "{}({})", self.caller, s)
    }
}

#[derive(Debug, Clone)]
pub struct Lambda {
    pub parameter: Vec<Token>,
    pub body: Box<Statement>,
}

impl fmt::Display for Lambda {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        let mut iter = self.parameter.iter();
        if let Some(param) = iter.next() {
            s.push_str(&param.lexeme.to_string());
            for param in iter {
                s.push_str(&format!(", {}", param.lexeme));
            }
        }
        write!(f, "lambda({}) {}", s, self.body)
    }
}

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration(Declaration),
    ExpressionStatement(Expression),
    BlockStatement(Block),
    ForStatement(For),
    WhileStatement(While),
    LoopStatement(Loop),
    BreakStatement(Break),
    ContinueStatement(Continue),
    FunctionDeclaration(FunctionDeclaration),
    ReturnStatement(Return),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::VariableDeclaration(s) => write!(f, "{s}"),
            Statement::ExpressionStatement(s) => write!(f, "{s}"),
            Statement::BlockStatement(s) => write!(f, "{s}"),
            Statement::WhileStatement(s) => write!(f, "{s}"),
            Statement::BreakStatement(s) => write!(f, "{s}"),
            Statement::ContinueStatement(s) => write!(f, "{s}"),
            Statement::LoopStatement(s) => write!(f, "{s}"),
            Statement::FunctionDeclaration(s) => write!(f, "{s}"),
            Statement::ReturnStatement(s) => write!(f, "{s}"),
            Statement::ForStatement(s) => write!(f, "{s}"),
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
pub struct For {
    pub variable: (Token, Option<Token>),
    pub iterable: Expression,
    pub iterable_token: Token,
    pub block: Box<Statement>,
}

impl fmt::Display for For {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let variables = if let Some(value) = &self.variable.1 {
            format!("{}, {}", self.variable.0.lexeme, value.lexeme)
        } else {
            String::from(&self.variable.0.lexeme)
        };
        write!(f, "for {} in {} {}", variables, self.iterable, self.block)
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

#[derive(Debug, Clone)]
pub struct FunctionDeclaration {
    pub token: Option<Token>,
    pub parameter: Vec<Token>,
    pub body: Box<Statement>,
}

impl fmt::Display for FunctionDeclaration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = String::new();
        let mut iter = self.parameter.iter();
        if let Some(param) = iter.next() {
            s.push_str(&param.lexeme.to_string());
            for param in iter {
                s.push_str(&format!(", {}", param.lexeme));
            }
        }
        let name = match &self.token {
            Some(token) => &token.lexeme,
            None => "",
        };
        write!(f, "function {}({}) {}", name, s, self.body)
    }
}

#[derive(Debug, Clone)]
pub struct Return {
    pub token: Token,
    pub expression: Expression,
}

impl fmt::Display for Return {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "return {}", self.expression)
    }
}
