#[cfg(test)]
mod tests {
    use crate::ast::parser::ParseError;
    use crate::ast::{Expression, Parser, Statement};
    use crate::lexer::tokens::{Span, Token, TokenWithSpan};

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
            Statement::Expression { expression, .. } => match expression {
                Expression::Number { value, .. } => {
                    assert_eq!(*value, 42);
                }
                _ => panic!("Expected number expression"),
            },
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
            Statement::Expression { expression, .. } => match expression {
                Expression::Identifier { name, .. } => {
                    assert_eq!(name, "x");
                }
                _ => panic!("Expected identifier expression"),
            },
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

        match &program.statements[0] {
            Statement::VariableDeclaration { name, .. } => {
                assert_eq!(name, "x");
            }
            _ => panic!("Expected variable declaration"),
        }

        // Second statement: y;
        match &program.statements[1] {
            Statement::Expression { expression, .. } => match expression {
                Expression::Identifier { name, .. } => {
                    assert_eq!(name, "y");
                }
                _ => panic!("Expected identifier expression"),
            },
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
            ParseError::UnexpectedToken {
                expected, found, ..
            } => {
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

    #[test]
    fn test_parse_pair_expression() {
        let tokens = vec![
            create_token_with_span(Token::LeftParen),
            create_token_with_span(Token::Number(1)),
            create_token_with_span(Token::Comma),
            create_token_with_span(Token::Number(2)),
            create_token_with_span(Token::RightParen),
            create_token_with_span(Token::Semicolon),
            create_token_with_span(Token::Eof),
        ];

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression { expression, .. } => match expression {
                Expression::Pair { first, second, .. } => {
                    match first.as_ref() {
                        Expression::Number { value: 1, .. } => (),
                        _ => panic!("Expected first element to be number 1"),
                    }
                    match second.as_ref() {
                        Expression::Number { value: 2, .. } => (),
                        _ => panic!("Expected second element to be number 2"),
                    }
                }
                _ => panic!("Expected pair expression, got {:?}", expression),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_parse_pair_variable_declaration() {
        let tokens = vec![
            create_token_with_span(Token::Let),
            create_token_with_span(Token::Identifier("pair".to_string())),
            create_token_with_span(Token::Assign),
            create_token_with_span(Token::LeftParen),
            create_token_with_span(Token::True),
            create_token_with_span(Token::Comma),
            create_token_with_span(Token::False),
            create_token_with_span(Token::RightParen),
            create_token_with_span(Token::Semicolon),
            create_token_with_span(Token::Eof),
        ];

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::VariableDeclaration { name, value, .. } => {
                assert_eq!(name, "pair");
                match value {
                    Expression::Pair { first, second, .. } => {
                        match first.as_ref() {
                            Expression::Boolean { value: true, .. } => (),
                            _ => panic!("Expected first element to be true"),
                        }
                        match second.as_ref() {
                            Expression::Boolean { value: false, .. } => (),
                            _ => panic!("Expected second element to be false"),
                        }
                    }
                    _ => panic!("Expected pair expression"),
                }
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_parse_parenthesized_expression() {
        let tokens = vec![
            create_token_with_span(Token::LeftParen),
            create_token_with_span(Token::Number(42)),
            create_token_with_span(Token::RightParen),
            create_token_with_span(Token::Semicolon),
            create_token_with_span(Token::Eof),
        ];

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression { expression, .. } => match expression {
                Expression::Number { value: 42, .. } => (),
                _ => panic!("Expected number expression, got {:?}", expression),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_parse_empty_list() {
        let tokens = vec![
            create_token_with_span(Token::LeftBracket),
            create_token_with_span(Token::RightBracket),
            create_token_with_span(Token::Semicolon),
            create_token_with_span(Token::Eof),
        ];

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression { expression, .. } => match expression {
                Expression::List { elements, .. } => {
                    assert_eq!(elements.len(), 0);
                }
                _ => panic!("Expected list expression, got {:?}", expression),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_parse_list_with_elements() {
        let tokens = vec![
            create_token_with_span(Token::LeftBracket),
            create_token_with_span(Token::Number(1)),
            create_token_with_span(Token::Comma),
            create_token_with_span(Token::Number(2)),
            create_token_with_span(Token::Comma),
            create_token_with_span(Token::Number(3)),
            create_token_with_span(Token::RightBracket),
            create_token_with_span(Token::Semicolon),
            create_token_with_span(Token::Eof),
        ];

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression { expression, .. } => match expression {
                Expression::List { elements, .. } => {
                    assert_eq!(elements.len(), 3);
                    match &elements[0] {
                        Expression::Number { value: 1, .. } => (),
                        _ => panic!("Expected first element to be 1"),
                    }
                    match &elements[1] {
                        Expression::Number { value: 2, .. } => (),
                        _ => panic!("Expected second element to be 2"),
                    }
                    match &elements[2] {
                        Expression::Number { value: 3, .. } => (),
                        _ => panic!("Expected third element to be 3"),
                    }
                }
                _ => panic!("Expected list expression, got {:?}", expression),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_parse_list_with_trailing_comma() {
        let tokens = vec![
            create_token_with_span(Token::LeftBracket),
            create_token_with_span(Token::Number(42)),
            create_token_with_span(Token::Comma),
            create_token_with_span(Token::RightBracket),
            create_token_with_span(Token::Semicolon),
            create_token_with_span(Token::Eof),
        ];

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression { expression, .. } => match expression {
                Expression::List { elements, .. } => {
                    assert_eq!(elements.len(), 1);
                    match &elements[0] {
                        Expression::Number { value: 42, .. } => (),
                        _ => panic!("Expected element to be 42"),
                    }
                }
                _ => panic!("Expected list expression, got {:?}", expression),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_parse_cons_expression() {
        let tokens = vec![
            create_token_with_span(Token::Cons),
            create_token_with_span(Token::LeftParen),
            create_token_with_span(Token::Number(1)),
            create_token_with_span(Token::Comma),
            create_token_with_span(Token::Identifier("list".to_string())),
            create_token_with_span(Token::RightParen),
            create_token_with_span(Token::Semicolon),
            create_token_with_span(Token::Eof),
        ];

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression { expression, .. } => match expression {
                Expression::Cons { head, tail, .. } => {
                    match &**head {
                        Expression::Number { value: 1, .. } => (),
                        _ => panic!("Expected head to be 1"),
                    }
                    match &**tail {
                        Expression::Identifier { name, .. } => {
                            assert_eq!(name, "list");
                        }
                        _ => panic!("Expected tail to be identifier 'list'"),
                    }
                }
                _ => panic!("Expected cons expression"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_parse_head_projection() {
        let tokens = vec![
            create_token_with_span(Token::Head),
            create_token_with_span(Token::LeftParen),
            create_token_with_span(Token::Identifier("list".to_string())),
            create_token_with_span(Token::RightParen),
            create_token_with_span(Token::Semicolon),
            create_token_with_span(Token::Eof),
        ];

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression { expression, .. } => match expression {
                Expression::HeadProjection { list, .. } => match &**list {
                    Expression::Identifier { name, .. } => {
                        assert_eq!(name, "list");
                    }
                    _ => panic!("Expected list to be identifier 'list'"),
                },
                _ => panic!("Expected head projection"),
            },
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_parse_tail_projection() {
        let tokens = vec![
            create_token_with_span(Token::Tail),
            create_token_with_span(Token::LeftParen),
            create_token_with_span(Token::Identifier("list".to_string())),
            create_token_with_span(Token::RightParen),
            create_token_with_span(Token::Semicolon),
            create_token_with_span(Token::Eof),
        ];

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            Statement::Expression { expression, .. } => match expression {
                Expression::TailProjection { list, .. } => match &**list {
                    Expression::Identifier { name, .. } => {
                        assert_eq!(name, "list");
                    }
                    _ => panic!("Expected list to be identifier 'list'"),
                },
                _ => panic!("Expected tail projection"),
            },
            _ => panic!("Expected expression statement"),
        }
    }
}
