use crate::lexer::Tokenizer;
use crate::ast::Parser;
use crate::typechecker::TypeChecker;

#[test]
fn test_full_pipeline_with_typechecker() {
    // Test 1: Simple number expression
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer.tokenize("42;").unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut type_checker = TypeChecker::new();
    let typed_program = type_checker.check_program(&program).unwrap();
    
    assert_eq!(typed_program.statements.len(), 1);
    println!("Test 1 - Type checked: {:#?}", typed_program);

    // Test 2: Variable declaration with type inference
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer.tokenize("let x = 42;").unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut type_checker = TypeChecker::new();
    let typed_program = type_checker.check_program(&program).unwrap();
    
    assert_eq!(typed_program.statements.len(), 1);
    println!("Test 2 - Type checked: {:#?}", typed_program);

    // Test 3: Variable usage after declaration
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer.tokenize("let x = 42; x;").unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut type_checker = TypeChecker::new();
    let typed_program = type_checker.check_program(&program).unwrap();
    
    assert_eq!(typed_program.statements.len(), 2);
    println!("Test 3 - Type checked: {:#?}", typed_program);

    // Test 4: Type error - undefined variable
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer.tokenize("undefined_var;").unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut type_checker = TypeChecker::new();
    let result = type_checker.check_program(&program);
    
    assert!(result.is_err());
    println!("Test 4 - Type error (as expected): {:?}", result.unwrap_err());
}

#[test]
fn test_list_typechecking_integration() {
    // Test 1: Empty list type checking
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer.tokenize("[];").unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut type_checker = TypeChecker::new();
    let typed_program = type_checker.check_program(&program).unwrap();
    
    assert_eq!(typed_program.statements.len(), 1);
    println!("List Type Test 1 - Empty list: {:#?}", typed_program);

    // Test 2: Homogeneous integer list
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer.tokenize("[1, 2, 3];").unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut type_checker = TypeChecker::new();
    let typed_program = type_checker.check_program(&program).unwrap();
    
    assert_eq!(typed_program.statements.len(), 1);
    println!("List Type Test 2 - Integer list: {:#?}", typed_program);

    // Test 3: Homogeneous boolean list
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer.tokenize("[true, false, true];").unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut type_checker = TypeChecker::new();
    let typed_program = type_checker.check_program(&program).unwrap();
    
    assert_eq!(typed_program.statements.len(), 1);
    println!("List Type Test 3 - Boolean list: {:#?}", typed_program);

    // Test 4: List variable declaration with explicit type
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer.tokenize("let numbers: List Int = [1, 2, 3];").unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut type_checker = TypeChecker::new();
    let typed_program = type_checker.check_program(&program).unwrap();
    
    assert_eq!(typed_program.statements.len(), 1);
    println!("List Type Test 4 - Explicit type: {:#?}", typed_program);

    // Test 5: Type error - heterogeneous list
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer.tokenize("[1, true, 3];").unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut type_checker = TypeChecker::new();
    let result = type_checker.check_program(&program);
    
    assert!(result.is_err());
    println!("List Type Test 5 - Type error (as expected): {:?}", result.unwrap_err());

    // Test 6: Empty list with type annotation
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer.tokenize("let empty: List Bool = [];").unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    let mut type_checker = TypeChecker::new();
    let typed_program = type_checker.check_program(&program).unwrap();
    
    assert_eq!(typed_program.statements.len(), 1);
    println!("List Type Test 6 - Empty list with annotation: {:#?}", typed_program);
}