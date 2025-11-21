pub mod environment;
pub mod interpreter;
pub mod value;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod fix_tests;

pub use environment::Environment;
pub use interpreter::Interpreter;
pub use value::Value;

/// Result type for interpreter operations
pub type InterpreterResult<T> = Result<T, InterpreterError>;

/// Interpreter error types
#[derive(Debug, Clone, PartialEq)]
pub enum InterpreterError {
    /// Runtime error with a message and optional location
    RuntimeError {
        message: String,
        span: Option<crate::lexer::tokens::Span>,
    },
    /// Division by zero error
    DivisionByZero { span: crate::lexer::tokens::Span },
    /// Variable not found error
    UndefinedVariable {
        name: String,
        span: crate::lexer::tokens::Span,
    },
    /// Type error during runtime
    TypeError {
        expected: String,
        found: String,
        span: crate::lexer::tokens::Span,
    },
    /// Function call on non-function value
    NotCallable { span: crate::lexer::tokens::Span },
    /// Index out of bounds for list access
    IndexOutOfBounds {
        index: i64,
        length: usize,
        span: crate::lexer::tokens::Span,
    },
}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InterpreterError::RuntimeError { message, span } => {
                if let Some(span) = span {
                    write!(
                        f,
                        "Runtime error at line {}, column {}: {}",
                        span.line, span.column, message
                    )
                } else {
                    write!(f, "Runtime error: {}", message)
                }
            }
            InterpreterError::DivisionByZero { span } => {
                write!(
                    f,
                    "Division by zero at line {}, column {}",
                    span.line, span.column
                )
            }
            InterpreterError::UndefinedVariable { name, span } => {
                write!(
                    f,
                    "Undefined variable '{}' at line {}, column {}",
                    name, span.line, span.column
                )
            }
            InterpreterError::TypeError {
                expected,
                found,
                span,
            } => {
                write!(
                    f,
                    "Type error at line {}, column {}: expected {}, found {}",
                    span.line, span.column, expected, found
                )
            }
            InterpreterError::NotCallable { span } => {
                write!(
                    f,
                    "Attempt to call non-function value at line {}, column {}",
                    span.line, span.column
                )
            }
            InterpreterError::IndexOutOfBounds {
                index,
                length,
                span,
            } => {
                write!(
                    f,
                    "Index {} out of bounds for list of length {} at line {}, column {}",
                    index, length, span.line, span.column
                )
            }
        }
    }
}

impl std::error::Error for InterpreterError {}
