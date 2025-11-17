#[cfg(test)]
mod new_features_tests {
    use crate::ast::Parser;
    use crate::lexer::Tokenizer;
    use crate::typechecker::Type;
    use crate::typechecker::TypeChecker;

    #[test]
    fn test_boolean_literal_true() {
        let mut tokenizer = Tokenizer::new("");
        let tokens = tokenizer.tokenize("true;").unwrap();

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        let mut type_checker = TypeChecker::new();
        let typed_program = type_checker.check_program(&program).unwrap();

        // Should have one expression statement with Boolean type
        assert_eq!(typed_program.statements.len(), 1);
        match &typed_program.statements[0] {
            crate::typechecker::TypedStatement::Expression { expression, .. } => {
                assert_eq!(expression.ty, Type::Bool);
            }
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_boolean_literal_false() {
        let mut tokenizer = Tokenizer::new("");
        let tokens = tokenizer.tokenize("false;").unwrap();

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        let mut type_checker = TypeChecker::new();
        let typed_program = type_checker.check_program(&program).unwrap();

        // Should have one expression statement with Boolean type
        assert_eq!(typed_program.statements.len(), 1);
        match &typed_program.statements[0] {
            crate::typechecker::TypedStatement::Expression { expression, .. } => {
                assert_eq!(expression.ty, Type::Bool);
            }
            _ => panic!("Expected expression statement"),
        }
    }

    #[test]
    fn test_type_annotation_int() {
        let mut tokenizer = Tokenizer::new("");
        let tokens = tokenizer.tokenize("let x: Int = 42;").unwrap();

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        // Verify AST has type annotation
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            crate::ast::Statement::VariableDeclaration {
                type_annotation, ..
            } => {
                assert!(type_annotation.is_some());
            }
            _ => panic!("Expected variable declaration"),
        }

        let mut type_checker = TypeChecker::new();
        let typed_program = type_checker.check_program(&program).unwrap();

        // Should type check successfully with Int type
        assert_eq!(typed_program.statements.len(), 1);
        match &typed_program.statements[0] {
            crate::typechecker::TypedStatement::VariableDeclaration { ty, .. } => {
                assert_eq!(*ty, Type::Int);
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_type_annotation_bool() {
        let mut tokenizer = Tokenizer::new("");
        let tokens = tokenizer.tokenize("let flag: Bool = true;").unwrap();

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        let mut type_checker = TypeChecker::new();
        let typed_program = type_checker.check_program(&program).unwrap();

        // Should type check successfully with Bool type
        assert_eq!(typed_program.statements.len(), 1);
        match &typed_program.statements[0] {
            crate::typechecker::TypedStatement::VariableDeclaration { ty, .. } => {
                assert_eq!(*ty, Type::Bool);
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_type_annotation_mismatch() {
        let mut tokenizer = Tokenizer::new("");
        let tokens = tokenizer.tokenize("let x: Int = true;").unwrap();

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        let mut type_checker = TypeChecker::new();
        let result = type_checker.check_program(&program);

        // Should fail with type mismatch error
        assert!(result.is_err());
    }

    #[test]
    fn test_function_type_annotation() {
        let mut tokenizer = Tokenizer::new("");
        let tokens = tokenizer
            .tokenize("let f: Int -> Int = fn(x) { x };")
            .unwrap();

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        // Verify AST has function type annotation
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            crate::ast::Statement::VariableDeclaration {
                type_annotation, ..
            } => {
                assert!(type_annotation.is_some());
                if let Some(crate::ast::TypeExpression::Function { param, result, .. }) =
                    type_annotation
                {
                    // Should be Int -> Int
                    assert!(matches!(**param, crate::ast::TypeExpression::Int { .. }));
                    assert!(matches!(**result, crate::ast::TypeExpression::Int { .. }));
                } else {
                    panic!("Expected function type annotation");
                }
            }
            _ => panic!("Expected variable declaration"),
        }

        let mut type_checker = TypeChecker::new();
        let typed_program = type_checker.check_program(&program).unwrap();

        // Should type check successfully with function type
        assert_eq!(typed_program.statements.len(), 1);
        match &typed_program.statements[0] {
            crate::typechecker::TypedStatement::VariableDeclaration { ty, .. } => {
                if let Type::Function { param, result } = ty {
                    assert_eq!(**param, Type::Int);
                    assert_eq!(**result, Type::Int);
                } else {
                    panic!("Expected function type, got {:?}", ty);
                }
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_higher_order_function_type_annotation() {
        let mut tokenizer = Tokenizer::new("");
        let tokens = tokenizer
            .tokenize("let g: (Int -> Int) -> Bool = fn(f) { true };")
            .unwrap();

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        // Verify AST has higher-order function type annotation
        assert_eq!(program.statements.len(), 1);
        match &program.statements[0] {
            crate::ast::Statement::VariableDeclaration {
                type_annotation, ..
            } => {
                assert!(type_annotation.is_some());
                if let Some(crate::ast::TypeExpression::Function { param, result, .. }) =
                    type_annotation
                {
                    // Should be (Int -> Int) -> Bool
                    assert!(matches!(
                        **param,
                        crate::ast::TypeExpression::Function { .. }
                    ));
                    assert!(matches!(**result, crate::ast::TypeExpression::Bool { .. }));
                } else {
                    panic!("Expected function type annotation");
                }
            }
            _ => panic!("Expected variable declaration"),
        }

        let mut type_checker = TypeChecker::new();
        let typed_program = type_checker.check_program(&program).unwrap();

        // Should type check successfully with higher-order function type
        assert_eq!(typed_program.statements.len(), 1);
        match &typed_program.statements[0] {
            crate::typechecker::TypedStatement::VariableDeclaration { ty, .. } => {
                if let Type::Function { param, result } = ty {
                    // The parameter should be a function type (Int -> Int)
                    assert!(matches!(**param, Type::Function { .. }));
                    // The result should be Bool
                    assert_eq!(**result, Type::Bool);
                } else {
                    panic!("Expected function type, got {:?}", ty);
                }
            }
            _ => panic!("Expected variable declaration"),
        }
    }

    #[test]
    fn test_type_inference_with_arithmetic() {
        let mut tokenizer = Tokenizer::new("");
        let tokens = tokenizer
            .tokenize("let add: Int -> Int = fn(x) { x + 1 };")
            .unwrap();

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        let mut type_checker = TypeChecker::new();
        let typed_program = type_checker.check_program(&program).unwrap();

        // Should infer that parameter x is Int based on usage in x + 1
        assert_eq!(typed_program.statements.len(), 1);
        match &typed_program.statements[0] {
            crate::typechecker::TypedStatement::VariableDeclaration { ty, .. } => {
                if let Type::Function { param, result } = ty {
                    assert_eq!(**param, Type::Int);
                    assert_eq!(**result, Type::Int);
                } else {
                    panic!("Expected function type, got {:?}", ty);
                }
            }
            _ => panic!("Expected variable declaration"),
        }
    }
}
