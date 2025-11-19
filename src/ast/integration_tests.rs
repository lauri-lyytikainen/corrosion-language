use crate::lexer::Tokenizer;
use crate::ast::Parser;

#[test]
fn test_end_to_end_parsing() {
    // Test 1: Simple number expression
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer.tokenize("42;").unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    assert_eq!(program.statements.len(), 1);
    println!("Test 1 passed: {:#?}", program);

    // Test 2: Variable declaration
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer.tokenize("let x = 42;").unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    assert_eq!(program.statements.len(), 1);
    println!("Test 2 passed: {:#?}", program);

    // Test 3: Multiple statements
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer.tokenize("let x = 42; y;").unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    assert_eq!(program.statements.len(), 2);
    println!("Test 3 passed: {:#?}", program);
}

#[test]
fn test_list_parsing_integration() {
    // Test 1: Empty list
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer.tokenize("[];").unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    assert_eq!(program.statements.len(), 1);
    println!("List Test 1 passed: {:#?}", program);

    // Test 2: List with numbers
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer.tokenize("[1, 2, 3];").unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    assert_eq!(program.statements.len(), 1);
    println!("List Test 2 passed: {:#?}", program);

    // Test 3: List with trailing comma
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer.tokenize("[42,];").unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    assert_eq!(program.statements.len(), 1);
    println!("List Test 3 passed: {:#?}", program);

    // Test 4: List variable declaration
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer.tokenize("let numbers: List Int = [1, 2, 3];").unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();
    
    assert_eq!(program.statements.len(), 1);
    println!("List Test 4 passed: {:#?}", program);
}