use crate::ast::Parser;
use crate::lexer::Tokenizer;

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
    let tokens = tokenizer
        .tokenize("let numbers: List Int = [1, 2, 3];")
        .unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    println!("List Test 4 passed: {:#?}", program);
}

#[test]
fn test_list_operations_integration() {
    // Test 1: cons operation
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer.tokenize("cons(1, [2, 3]);").unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    println!("List Op Test 1 (cons) passed: {:#?}", program);

    // Test 2: head operation
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer.tokenize("head([1, 2, 3]);").unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    println!("List Op Test 2 (head) passed: {:#?}", program);

    // Test 3: tail operation
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer.tokenize("tail([1, 2, 3]);").unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    println!("List Op Test 3 (tail) passed: {:#?}", program);

    // Test 4: nested list operations
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer
        .tokenize("cons(head([1, 2]), tail([3, 4, 5]));")
        .unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    println!("List Op Test 4 (nested) passed: {:#?}", program);

    // Test 5: list operations in variable declarations
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer
        .tokenize("let first_elem = head(numbers);")
        .unwrap();
    let mut parser = Parser::new(tokens);
    let program = parser.parse().unwrap();

    assert_eq!(program.statements.len(), 1);
    println!("List Op Test 5 (variable decl) passed: {:#?}", program);
}
