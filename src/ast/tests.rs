#[cfg(test)]
mod tests {
    use crate::lexer::tokens::{Token, TokenWithSpan, Span};
    use crate::ast::{Parser, Statement, Expression};
    use crate::ast::parser::ParseError;

    fn create_test_span() -> Span {
        Span::new(0, 1, 1, 1)
    }

    fn create_token_with_span(token: Token) -> TokenWithSpan {
        TokenWithSpan::new(token, create_test_span())
    }

    #[test]
    fn test_parse_number_literal() {
        let tokens = vec![
            create_token_with_span(Token::Number(42)),
            create_token_with_span(Token::Semicolon),
            create_token_with_span(Token::Eof),
        ];

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression { expression, .. } => {
                match expression {
                    Expression::Number { value, .. } => {
                        assert_eq!(*value, 42);
                    }
                    _ => panic!("Expected number expression"),
                }
            }
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_parse_identifier() {
        let tokens = vec![
            create_token_with_span(Token::Identifier("x".to_string())),
            create_token_with_span(Token::Semicolon),
            create_token_with_span(Token::Eof),
        ];

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression { expression, .. } => {
                match expression {
                    Expression::Identifier { name, .. } => {
                        assert_eq!(name, "x");
                    }
                    _ => panic!("Expected identifier expression"),
                }
            }
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_parse_variable_declaration() {
        let tokens = vec![
            create_token_with_span(Token::Let),
            create_token_with_span(Token::Identifier("x".to_string())),
            create_token_with_span(Token::Assign),
            create_token_with_span(Token::Number(42)),
            create_token_with_span(Token::Semicolon),
            create_token_with_span(Token::Eof),
        ];

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::VariableDeclaration { name, value, .. } => {
                assert_eq!(name, "x");
                match value {
                    Expression::Number { value, .. } => {
                        assert_eq!(*value, 42);
                    }
                    _ => panic!("Expected number expression"),
                }
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_parse_multiple_statements() {
        let tokens = vec![
            create_token_with_span(Token::Let),
            create_token_with_span(Token::Identifier("x".to_string())),
            create_token_with_span(Token::Assign),
            create_token_with_span(Token::Number(42)),
            create_token_with_span(Token::Semicolon),
            create_token_with_span(Token::Identifier("y".to_string())),
            create_token_with_span(Token::Semicolon),
            create_token_with_span(Token::Eof),
        ];

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 2);
        
        // First statement: let x = 42;
        match &program.statements[0] {
            Statement::VariableDeclaration { name, .. } => {
                assert_eq!(name, "x");
            }
            _ => panic!("Expected variable declaration"),
        }

        // Second statement: y;
        match &program.statements[1] {
            Statement::Expression { expression, .. } => {
                match expression {
                    Expression::Identifier { name, .. } => {
                        assert_eq!(name, "y");
                    }
                    _ => panic!("Expected identifier expression"),
                }
            }
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_parse_error_unexpected_token() {
        let tokens = vec![
            create_token_with_span(Token::Let),
            create_token_with_span(Token::Number(42)), // Should be identifier
            create_token_with_span(Token::Eof),
        ];

        let mut parser = Parser::new(tokens);
        let result = parser.parse();

        assert!(result.is_err());
        match result.unwrap_err() {
            ParseError::UnexpectedToken { expected, found, .. } => {
                assert_eq!(expected, "identifier");
                assert_eq!(found, Token::Number(42));
            }
            _ => panic!("Expected unexpected token error"),
        }
    }

    #[test]
    fn test_empty_program() {
        let tokens = vec![create_token_with_span(Token::Eof)];

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 0);
    }
}