use crate::ast::Parser;
use crate::lexer::Tokenizer;

#[test]
fn test_pair_integration() {
    let input = "(1, 2);";
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer.tokenize(input).unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        crate::ast::Statement::Expression { expression, .. } => match expression {
            crate::ast::Expression::Pair { first, second, .. } => {
                match first.as_ref() {
                    crate::ast::Expression::Number { value: 1, .. } => (),
                    _ => panic!("First element should be 1, got {:?}", first),
                }
                match second.as_ref() {
                    crate::ast::Expression::Number { value: 2, .. } => (),
                    _ => panic!("Second element should be 2, got {:?}", second),
                }
            }
            _ => panic!("Expected pair expression, got {:?}", expression),
        },
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_nested_pair_integration() {
    let input = "((1, true), 42);";
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer.tokenize(input).unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        crate::ast::Statement::Expression { expression, .. } => {
            match expression {
                crate::ast::Expression::Pair { first, second, .. } => {
                    match first.as_ref() {
                        crate::ast::Expression::Pair {
                            first: inner_first,
                            second: inner_second,
                            ..
                        } => {
                            match inner_first.as_ref() {
                                crate::ast::Expression::Number { value: 1, .. } => (),
                                _ => panic!("Inner first should be 1"),
                            }
                            match inner_second.as_ref() {
                                crate::ast::Expression::Boolean { value: true, .. } => (),
                                _ => panic!("Inner second should be true"),
                            }
                        }
                        _ => panic!("First element should be a pair"),
                    }

                    // Second should be 42
                    match second.as_ref() {
                        crate::ast::Expression::Number { value: 42, .. } => (),
                        _ => panic!("Second element should be 42"),
                    }
                }
                _ => panic!("Expected pair expression"),
            }
        }
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_parenthesized_expression_integration() {
    let input = "(42);";
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer.tokenize(input).unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);

    match &program.statements[0] {
        crate::ast::Statement::Expression { expression, .. } => match expression {
            crate::ast::Expression::Number { value: 42, .. } => (),
            _ => panic!("Expected number 42, got {:?}", expression),
        },
        _ => panic!("Expected expression statement"),
    }
}
