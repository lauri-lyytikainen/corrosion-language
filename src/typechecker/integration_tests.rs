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