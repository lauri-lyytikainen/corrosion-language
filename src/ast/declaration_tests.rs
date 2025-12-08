use crate::ast::nodes::{Expression, Statement};
use crate::ast::parser::Parser;
use crate::interpreter::Interpreter;
use crate::lexer::tokenizer::Tokenizer;

use crate::typechecker::TypeChecker;

#[test]
fn test_function_declaration_parsing() {
    let input = "fn add(x) -> Int { x + 1 }";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize(input).unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Statement::FunctionDeclaration { name, param, .. } => {
            assert_eq!(name, "add");
            assert_eq!(param, "x");
        }
        _ => panic!("Expected function declaration"),
    }
}

#[test]
fn test_function_declaration_with_typed_parameter() {
    let input = "fn add(x: Int) -> Int { x + 1 }";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize(input).unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Statement::FunctionDeclaration {
            name,
            param,
            param_type,
            return_type,
            ..
        } => {
            assert_eq!(name, "add");
            assert_eq!(param, "x");
            assert!(param_type.is_some());
            assert!(return_type.is_some());
        }
        _ => panic!("Expected function declaration"),
    }
}

#[test]
fn test_function_declaration_with_typed_parameter_no_return() {
    let input = "fn double(y: Int) { y * 2 }";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize(input).unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Statement::FunctionDeclaration {
            name,
            param,
            param_type,
            return_type,
            ..
        } => {
            assert_eq!(name, "double");
            assert_eq!(param, "y");
            assert!(param_type.is_some());
            assert!(return_type.is_none());
        }
        _ => panic!("Expected function declaration"),
    }
}

#[test]
fn test_import_parsing() {
    let input = "import \"math.corr\" as math;";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize(input).unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Statement::Import { path, alias, .. } => {
            assert_eq!(path, "math.corr");
            assert_eq!(alias, &Some("math".to_string()));
        }
        _ => panic!("Expected import statement"),
    }
}

#[test]
fn test_qualified_identifier_parsing() {
    let input = "math.square(5);";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize(input).unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Statement::Expression { expression, .. } => match expression {
            Expression::FunctionCall { function, .. } => match function.as_ref() {
                Expression::QualifiedIdentifier { module, name, .. } => {
                    assert_eq!(module, "math");
                    assert_eq!(name, "square");
                }
                _ => panic!("Expected qualified identifier"),
            },
            _ => panic!("Expected function call"),
        },
        _ => panic!("Expected expression statement"),
    }
}

#[test]
fn test_function_declaration_type_checking() {
    let input = "fn add(x) -> Int { x + 1 }";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize(input).unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    let mut type_checker = TypeChecker::new();
    let typed_program = type_checker.check_program(&program);

    match &typed_program {
        Ok(_) => println!("Type checking succeeded"),
        Err(e) => println!("Type checking failed: {}", e),
    }

    assert!(typed_program.is_ok());
}

#[test]
fn test_function_declaration_with_typed_parameter_type_checking() {
    let input = "fn add(x: Int) -> Int { x + 1 }";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize(input).unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    let mut type_checker = TypeChecker::new();
    let typed_program = type_checker.check_program(&program);

    match &typed_program {
        Ok(_) => println!("Type checking succeeded for typed parameter"),
        Err(e) => println!("Type checking failed: {}", e),
    }

    assert!(typed_program.is_ok());
}

#[test]
fn test_function_declaration_interpretation() {
    let input = "fn add(x) { x + 1 } let result = add(5);";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize(input).unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    let mut interpreter = Interpreter::new();
    let result = interpreter.interpret_program(&program);

    assert!(result.is_ok());
}

#[test]
fn test_anonymous_function_with_typed_parameter() {
    let input = "let f = fn(x: Int) { x + 1 };";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize(input).unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Statement::VariableDeclaration { value, .. } => match value {
            Expression::Function {
                param, param_type, ..
            } => {
                assert_eq!(param, "x");
                assert!(param_type.is_some());
            }
            _ => panic!("Expected function expression"),
        },
        _ => panic!("Expected variable declaration"),
    }
}

#[test]
fn test_anonymous_function_backward_compatibility() {
    let input = "let f = fn(x) { x + 1 };";
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize(input).unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    match &program.statements[0] {
        Statement::VariableDeclaration { value, .. } => match value {
            Expression::Function {
                param, param_type, ..
            } => {
                assert_eq!(param, "x");
                assert!(param_type.is_none());
            }
            _ => panic!("Expected function expression"),
        },
        _ => panic!("Expected variable declaration"),
    }
}
