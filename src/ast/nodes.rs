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
    String {
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
    FunctionDeclaration {
        name: String,
        param: String,
        param_type: Option<TypeExpression>,
        return_type: Option<TypeExpression>,
        body: Expression,
        span: Span,
    },
    Import {
        path: String,
        alias: Option<String>, // Optional alias for the imported module
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
    QualifiedIdentifier {
        module: String,
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
    String {
        value: String,
        span: Span,
    },
    BinaryOp {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
        span: Span,
    },
    UnaryOp {
        operator: UnaryOperator,
        operand: Box<Expression>,
        span: Span,
    },
    // Function expressions
    Function {
        param: String, // Parameter name
        param_type: Option<TypeExpression>,
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
    // List operations
    Cons {
        head: Box<Expression>,
        tail: Box<Expression>,
        span: Span,
    },
    HeadProjection {
        list: Box<Expression>,
        span: Span,
    },
    TailProjection {
        list: Box<Expression>,
        span: Span,
    },
    // Built-in functions
    Print {
        value: Box<Expression>,
        span: Span,
    },
    // Control flow
    If {
        condition: Box<Expression>,
        then_branch: Box<Expression>,
        else_branch: Option<Box<Expression>>,
        span: Span,
    },
    // Loop constructs
    For {
        variable: String,
        iterable: Box<Expression>,
        body: Box<Expression>,
        span: Span,
    },
    Range {
        start: Box<Expression>,
        end: Box<Expression>,
        span: Span,
    },
    // String operations
    Concat {
        left: Box<Expression>,
        right: Box<Expression>,
        span: Span,
    },
    CharAt {
        string: Box<Expression>,
        index: Box<Expression>,
        span: Span,
    },
    Length {
        string: Box<Expression>,
        span: Span,
    },
    ToString {
        expression: Box<Expression>,
        span: Span,
    },
    TypeOf {
        expression: Box<Expression>,
        span: Span,
    },
    // Pattern matching
    Case {
        expression: Box<Expression>,
        left_pattern: String,
        left_body: Box<Expression>,
        right_pattern: String,
        right_body: Box<Expression>,
        span: Span,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum UnaryOperator {
    LogicalNot,
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

impl From<Token> for UnaryOperator {
    fn from(token: Token) -> Self {
        match token {
            Token::LogicalNot => UnaryOperator::LogicalNot,
            _ => panic!("Invalid unary operator token: {:?}", token),
        }
    }
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
            Statement::FunctionDeclaration { span, .. } => span,
            Statement::Import { span, .. } => span,
            Statement::Expression { span, .. } => span,
        }
    }
}

impl Spanned for Expression {
    fn span(&self) -> &Span {
        match self {
            Expression::Identifier { span, .. } => span,
            Expression::QualifiedIdentifier { span, .. } => span,
            Expression::Number { span, .. } => span,
            Expression::Boolean { span, .. } => span,
            Expression::String { span, .. } => span,
            Expression::BinaryOp { span, .. } => span,
            Expression::UnaryOp { span, .. } => span,
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
            Expression::Cons { span, .. } => span,
            Expression::HeadProjection { span, .. } => span,
            Expression::TailProjection { span, .. } => span,
            Expression::Print { span, .. } => span,
            Expression::If { span, .. } => span,
            Expression::For { span, .. } => span,
            Expression::Range { span, .. } => span,
            Expression::Concat { span, .. } => span,
            Expression::CharAt { span, .. } => span,
            Expression::Length { span, .. } => span,
            Expression::ToString { span, .. } => span,
            Expression::TypeOf { span, .. } => span,
            Expression::Case { span, .. } => span,
        }
    }
}

impl Spanned for TypeExpression {
    fn span(&self) -> &Span {
        match self {
            TypeExpression::Int { span } => span,
            TypeExpression::Bool { span } => span,
            TypeExpression::String { span } => span,
            TypeExpression::List { span, .. } => span,
            TypeExpression::Function { span, .. } => span,
            TypeExpression::Pair { span, .. } => span,
            TypeExpression::Sum { span, .. } => span,
            TypeExpression::Recursive { span, .. } => span,
            TypeExpression::Named { span, .. } => span,
        }
    }
}
