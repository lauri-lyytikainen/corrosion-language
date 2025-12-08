use crate::lexer::tokens::Span;
use crate::typechecker::{BinaryOp, Type};

#[derive(Debug, Clone)]
pub enum TypeError {
    UndefinedVariable {
        name: String,
        span: Span,
    },
    TypeMismatch {
        expected: Type,
        found: Type,
        span: Span,
    },
    InvalidBinaryOperation {
        left: Type,
        op: BinaryOp,
        right: Type,
        span: Span,
    },
    RedefinedVariable {
        name: String,
        span: Span,
    },
    ImportError {
        path: String,
        message: String,
        span: Span,
    },
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::UndefinedVariable { name, span } => {
                write!(
                    f,
                    "Undefined variable '{}' at line {}, column {}",
                    name, span.line, span.column
                )
            }
            TypeError::TypeMismatch {
                expected,
                found,
                span,
            } => {
                write!(
                    f,
                    "Type mismatch at line {}, column {}: expected '{}', found '{}'",
                    span.line, span.column, expected, found
                )
            }
            TypeError::InvalidBinaryOperation {
                left,
                op,
                right,
                span,
            } => {
                write!(
                    f,
                    "Invalid binary operation at line {}, column {}: '{}' {:?} '{}'",
                    span.line, span.column, left, op, right
                )
            }
            TypeError::RedefinedVariable { name, span } => {
                write!(
                    f,
                    "Variable '{}' redefined at line {}, column {}",
                    name, span.line, span.column
                )
            }
            TypeError::ImportError {
                path,
                message,
                span,
            } => {
                write!(
                    f,
                    "Import error at line {}, column {}: {} (path: {})",
                    span.line, span.column, message, path
                )
            }
        }
    }
}

impl std::error::Error for TypeError {}

pub type TypeResult<T> = Result<T, TypeError>;
