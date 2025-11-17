use crate::lexer::tokens::{Token, TokenWithSpan, Span};
use crate::ast::nodes::{Program, Statement, Expression, Spanned};

#[derive(Debug)]
pub enum ParseError {
    UnexpectedToken {
        expected: String,
        found: Token,
        span: Span,
    },
    UnexpectedEof,
    InvalidExpression {
        message: String,
        span: Span,
    },
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::UnexpectedToken { expected, found, span } => {
                write!(f, "Unexpected token at line {}, column {}: expected {}, found {:?}", 
                       span.line, span.column, expected, found)
            }
            ParseError::UnexpectedEof => write!(f, "Unexpected end of file"),
            ParseError::InvalidExpression { message, span } => {
                write!(f, "Invalid expression at line {}, column {}: {}", 
                       span.line, span.column, message)
            }
        }
    }
}

impl std::error::Error for ParseError {}

pub type ParseResult<T> = Result<T, ParseError>;

pub struct Parser {
    tokens: Vec<TokenWithSpan>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenWithSpan>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn parse(&mut self) -> ParseResult<Program> {
        let start_span = self.current_span();
        let mut statements = Vec::new();

        while !self.is_at_end() {
            if let Token::Eof = self.peek().token {
                break;
            }
            statements.push(self.parse_statement()?);
        }

        let end_span = if statements.is_empty() {
            start_span.clone()
        } else {
            self.previous_span()
        };

        let program_span = Span::new(
            start_span.start,
            end_span.end,
            start_span.line,
            start_span.column,
        );

        Ok(Program::new(statements, program_span))
    }

    fn parse_statement(&mut self) -> ParseResult<Statement> {
        match &self.peek().token {
            Token::Let => self.parse_variable_declaration(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_variable_declaration(&mut self) -> ParseResult<Statement> {
        let start_span = self.current_span();
        self.consume(Token::Let, "Expected 'let'")?;

        let name = if let Token::Identifier(name) = &self.advance().token {
            name.clone()
        } else {
            return Err(ParseError::UnexpectedToken {
                expected: "identifier".to_string(),
                found: self.previous().token.clone(),
                span: self.previous_span(),
            });
        };

        self.consume(Token::Assign, "Expected '='")?;
        let value = self.parse_expression()?;
        self.consume(Token::Semicolon, "Expected ';'")?;

        let end_span = self.previous_span();
        let span = Span::new(
            start_span.start,
            end_span.end,
            start_span.line,
            start_span.column,
        );

        Ok(Statement::VariableDeclaration { name, value, span })
    }

    fn parse_expression_statement(&mut self) -> ParseResult<Statement> {
        let expression = self.parse_expression()?;
        let span = expression.span().clone();
        self.consume(Token::Semicolon, "Expected ';'")?;

        Ok(Statement::Expression { expression, span })
    }

    fn parse_expression(&mut self) -> ParseResult<Expression> {
        self.parse_primary()
    }

    fn parse_primary(&mut self) -> ParseResult<Expression> {
        let token = self.advance().token.clone();
        match token {
            Token::Number(value) => {
                let span = self.previous_span();
                Ok(Expression::Number { 
                    value, 
                    span 
                })
            }
            Token::Identifier(name) => {
                let span = self.previous_span();
                Ok(Expression::Identifier { 
                    name, 
                    span 
                })
            }
            token => Err(ParseError::UnexpectedToken {
                expected: "expression".to_string(),
                found: token,
                span: self.previous_span(),
            }),
        }
    }

    // Helper methods
    fn consume(&mut self, expected: Token, message: &str) -> ParseResult<&TokenWithSpan> {
        if self.check(&expected) {
            Ok(self.advance())
        } else {
            Err(ParseError::UnexpectedToken {
                expected: message.to_string(),
                found: self.peek().token.clone(),
                span: self.current_span(),
            })
        }
    }

    fn check(&self, token: &Token) -> bool {
        if self.is_at_end() {
            false
        } else {
            std::mem::discriminant(&self.peek().token) == std::mem::discriminant(token)
        }
    }

    fn advance(&mut self) -> &TokenWithSpan {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.tokens.len() || matches!(self.peek().token, Token::Eof)
    }

    fn peek(&self) -> &TokenWithSpan {
        &self.tokens[self.current]
    }

    fn previous(&self) -> &TokenWithSpan {
        &self.tokens[self.current - 1]
    }

    fn current_span(&self) -> Span {
        if self.is_at_end() && self.current > 0 {
            self.previous_span()
        } else {
            self.peek().span.clone()
        }
    }

    fn previous_span(&self) -> Span {
        if self.current > 0 {
            self.previous().span.clone()
        } else {
            // Fallback for empty spans
            Span::new(0, 0, 1, 1)
        }
    }
}