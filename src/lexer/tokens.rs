#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Keywords
    Let,

    // Type keywords
    Int,  // Int
    Bool, // Bool
    List, // List
    Rec,  // Rec

    // Function keywords
    Fn,  // fn
    Fix, // fix

    // Pair destructuring keywords
    Fst, // fst (first element)
    Snd, // snd (second element)

    // List operations
    Cons, // cons (construct list)
    Head, // head (first element of list)
    Tail, // tail (rest of list)

    // Built-in functions
    Print, // print (output to console)

    // Control flow
    If,    // if
    Else,  // else
    For,   // for (list iteration)
    In,    // in (for iteration keyword)
    Range, // range (numeric range generation)

    // Sum type constructors
    Inl, // inl (left injection)
    Inr, // inr (right injection)

    // Sum type pattern matching
    Case,     // case
    Of,       // of
    Pipe,     // |
    FatArrow, // =>

    // Boolean literals
    True,  // true
    False, // false

    // Identifiers and literals
    Identifier(String),
    Number(i64),

    // Operators
    Assign,           // =
    Arrow,            // ->
    Plus,             // +
    Minus,            // -
    Multiply,         // *
    Divide,           // /
    Equal,            // ==
    NotEqual,         // !=
    LessThan,         // <
    LessThanEqual,    // <=
    GreaterThan,      // >
    GreaterThanEqual, // >=
    LogicalAnd,       // &&
    LogicalOr,        // ||
    LogicalNot,       // !

    // Punctuation
    Semicolon,    // ;
    Colon,        // :
    LeftParen,    // (
    RightParen,   // )
    LeftBracket,  // [
    RightBracket, // ]
    LeftBrace,    // {
    RightBrace,   // }
    Comma,        // ,

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
