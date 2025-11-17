use crate::lexer::tokens::{Span, Token};

/// AST node types for the Corrosion language
#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub statements: Vec<Statement>,
    pub span: Span,
}

impl Program {
    pub fn new(statements: Vec<Statement>, span: Span) -> Self {
        Self { statements, span }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Statement {
    VariableDeclaration {
        name: String,
        value: Expression,
        span: Span,
    },
    Expression {
        expression: Expression,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Identifier {
        name: String,
        span: Span,
    },
    Number {
        value: i64,
        span: Span,
    },
    BinaryOp {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Assign,
}

impl From<Token> for BinaryOperator {
    fn from(token: Token) -> Self {
        match token {
            Token::Assign => BinaryOperator::Assign,
            _ => panic!("Unsupported binary operator token: {:?}", token),
        }
    }
}

// Helper trait for getting span information from AST nodes
pub trait Spanned {
    fn span(&self) -> &Span;
}

impl Spanned for Statement {
    fn span(&self) -> &Span {
        match self {
            Statement::VariableDeclaration { span, .. } => span,
            Statement::Expression { span, .. } => span,
        }
    }
}

impl Spanned for Expression {
    fn span(&self) -> &Span {
        match self {
            Expression::Identifier { span, .. } => span,
            Expression::Number { span, .. } => span,
            Expression::BinaryOp { span, .. } => span,
        }
    }
}
