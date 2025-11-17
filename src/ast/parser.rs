use crate::ast::nodes::{Expression, Program, Spanned, Statement, TypeExpression};
use crate::lexer::tokens::{Span, Token, TokenWithSpan};

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
            ParseError::UnexpectedToken {
                expected,
                found,
                span,
            } => {
                write!(
                    f,
                    "Unexpected token at line {}, column {}: expected {}, found {:?}",
                    span.line, span.column, expected, found
                )
            }
            ParseError::UnexpectedEof => write!(f, "Unexpected end of file"),
            ParseError::InvalidExpression { message, span } => {
                write!(
                    f,
                    "Invalid expression at line {}, column {}: {}",
                    span.line, span.column, message
                )
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

        // Optional type annotation
        let type_annotation = if self.peek().token == Token::Colon {
            self.advance(); // consume ':'
            Some(self.parse_type_expression()?)
        } else {
            None
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

        Ok(Statement::VariableDeclaration {
            name,
            type_annotation,
            value,
            span,
        })
    }

    fn parse_expression_statement(&mut self) -> ParseResult<Statement> {
        let expression = self.parse_expression()?;
        let span = expression.span().clone();
        self.consume(Token::Semicolon, "Expected ';'")?;

        Ok(Statement::Expression { expression, span })
    }

    fn parse_expression(&mut self) -> ParseResult<Expression> {
        self.parse_binary_expression(0)
    }

    fn parse_binary_expression(&mut self, min_precedence: u8) -> ParseResult<Expression> {
        let mut left = self.parse_primary()?;

        while !self.is_at_end() {
            let token = &self.peek().token;
            if let Some((precedence, operator)) = self.get_binary_operator(token) {
                if precedence < min_precedence {
                    break;
                }

                self.advance(); // consume operator
                let right = self.parse_binary_expression(precedence + 1)?;

                let span = Span::new(
                    left.span().start,
                    right.span().end,
                    left.span().line,
                    left.span().column,
                );

                left = Expression::BinaryOp {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                    span,
                };
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn get_binary_operator(&self, token: &Token) -> Option<(u8, crate::ast::BinaryOperator)> {
        use crate::ast::BinaryOperator;
        match token {
            Token::Plus => Some((10, BinaryOperator::Add)),
            Token::Minus => Some((10, BinaryOperator::Subtract)),
            Token::Multiply => Some((20, BinaryOperator::Multiply)),
            Token::Divide => Some((20, BinaryOperator::Divide)),
            Token::Equal => Some((5, BinaryOperator::Equal)),
            Token::NotEqual => Some((5, BinaryOperator::NotEqual)),
            Token::LessThan => Some((5, BinaryOperator::LessThan)),
            Token::LessThanEqual => Some((5, BinaryOperator::LessThanEqual)),
            Token::GreaterThan => Some((5, BinaryOperator::GreaterThan)),
            Token::GreaterThanEqual => Some((5, BinaryOperator::GreaterThanEqual)),
            Token::LogicalAnd => Some((3, BinaryOperator::LogicalAnd)),
            Token::LogicalOr => Some((2, BinaryOperator::LogicalOr)),
            _ => None,
        }
    }

    fn parse_primary(&mut self) -> ParseResult<Expression> {
        let token = self.advance().token.clone();
        match token {
            Token::Number(value) => {
                let span = self.previous_span();
                Ok(Expression::Number { value, span })
            }
            Token::True => {
                let span = self.previous_span();
                Ok(Expression::Boolean { value: true, span })
            }
            Token::False => {
                let span = self.previous_span();
                Ok(Expression::Boolean { value: false, span })
            }
            Token::Identifier(name) => {
                let span = self.previous_span();
                Ok(Expression::Identifier { name, span })
            }
            Token::Fn => self.parse_function_expression(),
            Token::LeftParen => self.parse_parenthesized_or_pair_expression(),
            token => Err(ParseError::UnexpectedToken {
                expected: "expression".to_string(),
                found: token,
                span: self.previous_span(),
            }),
        }
    }

    fn parse_function_expression(&mut self) -> ParseResult<Expression> {
        let start_span = self.previous_span();

        self.consume(Token::LeftParen, "Expected '(' after 'fn'")?;

        let param = if let Token::Identifier(name) = &self.advance().token {
            name.clone()
        } else {
            return Err(ParseError::UnexpectedToken {
                expected: "parameter name".to_string(),
                found: self.previous().token.clone(),
                span: self.previous_span(),
            });
        };

        self.consume(Token::RightParen, "Expected ')' after parameter")?;
        self.consume(Token::LeftBrace, "Expected '{' to start function body")?;

        // Parse the function body as an expression
        let body = Box::new(self.parse_expression()?);

        self.consume(Token::RightBrace, "Expected '}' to end function body")?;

        let end_span = self.previous_span();
        let span = Span::new(
            start_span.start,
            end_span.end,
            start_span.line,
            start_span.column,
        );

        Ok(Expression::Function { param, body, span })
    }

    fn parse_parenthesized_or_pair_expression(&mut self) -> ParseResult<Expression> {
        let start_span = self.previous_span();

        // Parse the first expression
        let first = self.parse_expression()?;

        // Check if this is a pair (has a comma) or just a parenthesized expression
        if self.peek().token == Token::Comma {
            self.advance(); // consume ','
            let second = Box::new(self.parse_expression()?);
            self.consume(Token::RightParen, "Expected ')' after pair")?;

            let end_span = self.previous_span();
            let span = Span::new(
                start_span.start,
                end_span.end,
                start_span.line,
                start_span.column,
            );

            Ok(Expression::Pair {
                first: Box::new(first),
                second,
                span,
            })
        } else {
            // Just a parenthesized expression
            self.consume(Token::RightParen, "Expected ')'")?;
            Ok(first)
        }
    }

    fn parse_type_expression(&mut self) -> ParseResult<TypeExpression> {
        self.parse_function_type()
    }

    fn parse_function_type(&mut self) -> ParseResult<TypeExpression> {
        let mut left = self.parse_primary_type()?;

        while self.peek().token == Token::Arrow {
            self.advance(); // consume '->'
            let right = self.parse_primary_type()?;
            let span = Span::new(
                left.span().start,
                right.span().end,
                left.span().line,
                left.span().column,
            );
            left = TypeExpression::Function {
                param: Box::new(left),
                result: Box::new(right),
                span,
            };
        }

        Ok(left)
    }

    fn parse_primary_type(&mut self) -> ParseResult<TypeExpression> {
        let token = self.advance().token.clone();
        match token {
            Token::Int => {
                let span = self.previous_span();
                Ok(TypeExpression::Int { span })
            }
            Token::Bool => {
                let span = self.previous_span();
                Ok(TypeExpression::Bool { span })
            }
            Token::List => {
                let start_span = self.previous_span();
                let element = Box::new(self.parse_function_type()?);
                let span = Span::new(
                    start_span.start,
                    element.span().end,
                    start_span.line,
                    start_span.column,
                );
                Ok(TypeExpression::List { element, span })
            }
            Token::Rec => {
                let start_span = self.previous_span();
                let inner = Box::new(self.parse_function_type()?);
                let span = Span::new(
                    start_span.start,
                    inner.span().end,
                    start_span.line,
                    start_span.column,
                );
                Ok(TypeExpression::Recursive { inner, span })
            }
            Token::Identifier(name) => {
                let span = self.previous_span();
                Ok(TypeExpression::Named { name, span })
            }
            Token::LeftParen => {
                // Parse pair type (T1, T2) or parenthesized type
                let first = self.parse_function_type()?;
                if self.peek().token == Token::Comma {
                    self.advance(); // consume ','
                    let second = Box::new(self.parse_function_type()?);
                    self.consume(Token::RightParen, "Expected ')')")?;
                    let end_span = self.previous_span();
                    let span = Span::new(
                        first.span().start,
                        end_span.end,
                        first.span().line,
                        first.span().column,
                    );
                    Ok(TypeExpression::Pair {
                        first: Box::new(first),
                        second,
                        span,
                    })
                } else {
                    self.consume(Token::RightParen, "Expected ')')")?;
                    Ok(first) // Parenthesized type
                }
            }
            token => Err(ParseError::UnexpectedToken {
                expected: "type expression".to_string(),
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
