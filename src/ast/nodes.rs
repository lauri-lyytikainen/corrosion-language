use crate::lexer::tokens::{Span, Token};

/// AST node types for the Corrosion language
#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    Program(Program),
    Statement(Statement),
    Expression(Expression),
    TypeExpression(TypeExpression),
}

/// Type expressions for type annotations
#[derive(Debug, Clone, PartialEq)]
pub enum TypeExpression {
    Int {
        span: Span,
    },
    Bool {
        span: Span,
    },
    List {
        element: Box<TypeExpression>,
        span: Span,
    },
    Function {
        param: Box<TypeExpression>,
        result: Box<TypeExpression>,
        span: Span,
    },
    Pair {
        first: Box<TypeExpression>,
        second: Box<TypeExpression>,
        span: Span,
    },
    Sum {
        left: Box<TypeExpression>,
        right: Box<TypeExpression>,
        span: Span,
    },
    Recursive {
        inner: Box<TypeExpression>,
        span: Span,
    },
    Named {
        name: String,
        span: Span,
    }, // For named types/type variables
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
        type_annotation: Option<TypeExpression>,
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
    Boolean {
        value: bool,
        span: Span,
    },
    BinaryOp {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
        span: Span,
    },
    // Function expressions
    Function {
        param: String, // Parameter name
        body: Box<Expression>,
        span: Span,
    },
    FunctionCall {
        function: Box<Expression>,
        argument: Box<Expression>,
        span: Span,
    },
    // List expressions
    List {
        elements: Vec<Expression>,
        span: Span,
    },
    // Pair expressions
    Pair {
        first: Box<Expression>,
        second: Box<Expression>,
        span: Span,
    },
    // Sum type constructors
    LeftInject {
        value: Box<Expression>,
        span: Span,
    },
    RightInject {
        value: Box<Expression>,
        span: Span,
    },
    // Recursive fixpoint
    Fix {
        function: Box<Expression>,
        span: Span,
    },
    // Block expressions (for function bodies)
    Block {
        statements: Vec<Statement>,
        expression: Option<Box<Expression>>,
        span: Span,
    },
    // Pair destructuring
    FirstProjection {
        pair: Box<Expression>,
        span: Span,
    },
    SecondProjection {
        pair: Box<Expression>,
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
    Equal,
    NotEqual,
    LessThan,
    LessThanEqual,
    GreaterThan,
    GreaterThanEqual,
    LogicalAnd,
    LogicalOr,
}

impl From<Token> for BinaryOperator {
    fn from(token: Token) -> Self {
        match token {
            Token::Assign => BinaryOperator::Assign,
            Token::Plus => BinaryOperator::Add,
            Token::Minus => BinaryOperator::Subtract,
            Token::Multiply => BinaryOperator::Multiply,
            Token::Divide => BinaryOperator::Divide,
            Token::Equal => BinaryOperator::Equal,
            Token::NotEqual => BinaryOperator::NotEqual,
            Token::LessThan => BinaryOperator::LessThan,
            Token::LessThanEqual => BinaryOperator::LessThanEqual,
            Token::GreaterThan => BinaryOperator::GreaterThan,
            Token::GreaterThanEqual => BinaryOperator::GreaterThanEqual,
            Token::LogicalAnd => BinaryOperator::LogicalAnd,
            Token::LogicalOr => BinaryOperator::LogicalOr,
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
            Expression::Boolean { span, .. } => span,
            Expression::BinaryOp { span, .. } => span,
            Expression::Function { span, .. } => span,
            Expression::FunctionCall { span, .. } => span,
            Expression::List { span, .. } => span,
            Expression::Pair { span, .. } => span,
            Expression::LeftInject { span, .. } => span,
            Expression::RightInject { span, .. } => span,
            Expression::Fix { span, .. } => span,
            Expression::Block { span, .. } => span,
            Expression::FirstProjection { span, .. } => span,
            Expression::SecondProjection { span, .. } => span,
        }
    }
}

impl Spanned for TypeExpression {
    fn span(&self) -> &Span {
        match self {
            TypeExpression::Int { span } => span,
            TypeExpression::Bool { span } => span,
            TypeExpression::List { span, .. } => span,
            TypeExpression::Function { span, .. } => span,
            TypeExpression::Pair { span, .. } => span,
            TypeExpression::Sum { span, .. } => span,
            TypeExpression::Recursive { span, .. } => span,
            TypeExpression::Named { span, .. } => span,
        }
    }
}
