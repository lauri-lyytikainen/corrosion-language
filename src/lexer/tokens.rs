#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Let,

    // Identifiers and literals
    Identifier(String),
    Number(i64),

    // Operators
    Assign, // =

    // Punctuation
    Semicolon, // ;
    Colon,     // :

    // Special
    Eof,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Span {
    pub start: usize,
    pub end: usize,
    pub line: usize,
    pub column: usize,
}

impl Span {
    pub fn new(start: usize, end: usize, line: usize, column: usize) -> Self {
        Self {
            start,
            end,
            line,
            column,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TokenWithSpan {
    pub token: Token,
    pub span: Span,
}

impl TokenWithSpan {
    pub fn new(token: Token, span: Span) -> Self {
        Self { token, span }
    }
}
