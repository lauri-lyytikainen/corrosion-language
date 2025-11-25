use crate::lexer::tokens::Span;

/// Type system for the Corrosion language
#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    /// Integer type
    Int,
    /// Boolean type  
    Bool,
    /// String type
    String,
    /// Unit type (void)
    Unit,
    /// Function type (T1 -> T2)
    Function { param: Box<Type>, result: Box<Type> },
    /// Pair type (T1, T2)
    Pair { first: Box<Type>, second: Box<Type> },
    /// List type (List T)
    List { element: Box<Type> },
    /// Sum type (T1 + T2)
    Sum { left: Box<Type>, right: Box<Type> },
    /// Recursive type (Rec T)
    Recursive { inner: Box<Type> },
    /// Unknown type (for type inference)
    Unknown,
    /// Error type (for type errors)
    Error,
}

impl Type {
    /// Check if two types are compatible for assignment
    pub fn is_assignable_to(&self, other: &Type) -> bool {
        match (self, other) {
            (Type::Error, _) | (_, Type::Error) => true, // Error type is compatible with anything
            (Type::Unknown, _) | (_, Type::Unknown) => true, // Unknown can be inferred
            (a, b) => a == b, // For now, require exact match. Can be extended for subtyping
        }
    }

    /// Check if a binary operation is valid between two types
    pub fn can_binary_op(&self, op: &BinaryOp, other: &Type) -> Option<Type> {
        match (self, op, other) {
            // Arithmetic operations on integers
            (Type::Int, BinaryOp::Add, Type::Int) => Some(Type::Int),
            (Type::Int, BinaryOp::Subtract, Type::Int) => Some(Type::Int),
            (Type::Int, BinaryOp::Multiply, Type::Int) => Some(Type::Int),
            (Type::Int, BinaryOp::Divide, Type::Int) => Some(Type::Int),

            // Comparison operations on integers
            (Type::Int, BinaryOp::Equal, Type::Int) => Some(Type::Bool),
            (Type::Int, BinaryOp::NotEqual, Type::Int) => Some(Type::Bool),
            (Type::Int, BinaryOp::LessThan, Type::Int) => Some(Type::Bool),
            (Type::Int, BinaryOp::LessThanEqual, Type::Int) => Some(Type::Bool),
            (Type::Int, BinaryOp::GreaterThan, Type::Int) => Some(Type::Bool),
            (Type::Int, BinaryOp::GreaterThanEqual, Type::Int) => Some(Type::Bool),

            // Boolean equality
            (Type::Bool, BinaryOp::Equal, Type::Bool) => Some(Type::Bool),
            (Type::Bool, BinaryOp::NotEqual, Type::Bool) => Some(Type::Bool),

            // String operations
            (Type::String, BinaryOp::Add, Type::String) => Some(Type::String), // String concatenation
            (Type::String, BinaryOp::Equal, Type::String) => Some(Type::Bool),
            (Type::String, BinaryOp::NotEqual, Type::String) => Some(Type::Bool),

            // Logical operations on booleans
            (Type::Bool, BinaryOp::LogicalAnd, Type::Bool) => Some(Type::Bool),
            (Type::Bool, BinaryOp::LogicalOr, Type::Bool) => Some(Type::Bool),

            // Assignment
            (_, BinaryOp::Assign, rhs) if self.is_assignable_to(rhs) => Some(rhs.clone()),

            // Error propagation
            (Type::Error, _, _) | (_, _, Type::Error) => Some(Type::Error),

            // Unknown type inference - but respect operation semantics
            (Type::Unknown, BinaryOp::Equal, _) | (_, BinaryOp::Equal, Type::Unknown) => {
                Some(Type::Bool)
            }
            (Type::Unknown, BinaryOp::NotEqual, _) | (_, BinaryOp::NotEqual, Type::Unknown) => {
                Some(Type::Bool)
            }
            (Type::Unknown, BinaryOp::LessThan, _) | (_, BinaryOp::LessThan, Type::Unknown) => {
                Some(Type::Bool)
            }
            (Type::Unknown, BinaryOp::LessThanEqual, _)
            | (_, BinaryOp::LessThanEqual, Type::Unknown) => Some(Type::Bool),
            (Type::Unknown, BinaryOp::GreaterThan, _)
            | (_, BinaryOp::GreaterThan, Type::Unknown) => Some(Type::Bool),
            (Type::Unknown, BinaryOp::GreaterThanEqual, _)
            | (_, BinaryOp::GreaterThanEqual, Type::Unknown) => Some(Type::Bool),
            (Type::Unknown, BinaryOp::LogicalAnd, _) | (_, BinaryOp::LogicalAnd, Type::Unknown) => {
                Some(Type::Bool)
            }
            (Type::Unknown, BinaryOp::LogicalOr, _) | (_, BinaryOp::LogicalOr, Type::Unknown) => {
                Some(Type::Bool)
            }
            (Type::Unknown, _, rhs) => Some(rhs.clone()),
            (lhs, _, Type::Unknown) => Some(lhs.clone()),

            _ => None, // Invalid operation
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Int => write!(f, "Int"),
            Type::Bool => write!(f, "Bool"),
            Type::String => write!(f, "String"),
            Type::Unit => write!(f, "Unit"),
            Type::Function { param, result } => write!(f, "({} -> {})", param, result),
            Type::Pair { first, second } => write!(f, "({}, {})", first, second),
            Type::List { element } => write!(f, "List {}", element),
            Type::Sum { left, right } => write!(f, "({} + {})", left, right),
            Type::Recursive { inner } => write!(f, "Rec {}", inner),
            Type::Unknown => write!(f, "unknown"),
            Type::Error => write!(f, "error"),
        }
    }
}

/// Binary operations in type checking context
#[derive(Debug, Clone, PartialEq)]
pub enum BinaryOp {
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

impl From<crate::ast::BinaryOperator> for BinaryOp {
    fn from(op: crate::ast::BinaryOperator) -> Self {
        match op {
            crate::ast::BinaryOperator::Add => BinaryOp::Add,
            crate::ast::BinaryOperator::Subtract => BinaryOp::Subtract,
            crate::ast::BinaryOperator::Multiply => BinaryOp::Multiply,
            crate::ast::BinaryOperator::Divide => BinaryOp::Divide,
            crate::ast::BinaryOperator::Assign => BinaryOp::Assign,
            crate::ast::BinaryOperator::Equal => BinaryOp::Equal,
            crate::ast::BinaryOperator::NotEqual => BinaryOp::NotEqual,
            crate::ast::BinaryOperator::LessThan => BinaryOp::LessThan,
            crate::ast::BinaryOperator::LessThanEqual => BinaryOp::LessThanEqual,
            crate::ast::BinaryOperator::GreaterThan => BinaryOp::GreaterThan,
            crate::ast::BinaryOperator::GreaterThanEqual => BinaryOp::GreaterThanEqual,
            crate::ast::BinaryOperator::LogicalAnd => BinaryOp::LogicalAnd,
            crate::ast::BinaryOperator::LogicalOr => BinaryOp::LogicalOr,
        }
    }
}

/// Type information with source location
#[derive(Debug, Clone, PartialEq)]
pub struct TypedExpression {
    pub ty: Type,
    pub span: Span,
}

impl TypedExpression {
    pub fn new(ty: Type, span: Span) -> Self {
        Self { ty, span }
    }
}

/// Type-checked statement
#[derive(Debug, Clone, PartialEq)]
pub enum TypedStatement {
    VariableDeclaration {
        name: String,
        ty: Type,
        value: TypedExpression,
        span: Span,
    },
    FunctionDeclaration {
        name: String,
        param: String,
        param_type: Type,
        return_type: Type,
        body: TypedExpression,
        span: Span,
    },
    Import {
        path: String,
        alias: Option<String>,
        span: Span,
    },
    Expression {
        expression: TypedExpression,
        span: Span,
    },
}

/// Type-checked program
#[derive(Debug, Clone, PartialEq)]
pub struct TypedProgram {
    pub statements: Vec<TypedStatement>,
    pub span: Span,
}

impl TypedProgram {
    pub fn new(statements: Vec<TypedStatement>, span: Span) -> Self {
        Self { statements, span }
    }
}

impl Type {
    /// Helper to create function type
    pub fn function(param: Type, result: Type) -> Type {
        Type::Function {
            param: Box::new(param),
            result: Box::new(result),
        }
    }

    /// Helper to create pair type
    pub fn pair(first: Type, second: Type) -> Type {
        Type::Pair {
            first: Box::new(first),
            second: Box::new(second),
        }
    }

    /// Helper to create list type
    pub fn list(element: Type) -> Type {
        Type::List {
            element: Box::new(element),
        }
    }

    /// Helper to create sum type
    pub fn sum(left: Type, right: Type) -> Type {
        Type::Sum {
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    /// Helper to create recursive type
    pub fn recursive(inner: Type) -> Type {
        Type::Recursive {
            inner: Box::new(inner),
        }
    }
}
