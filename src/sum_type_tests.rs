use crate::ast::parser::Parser;
use crate::interpreter::interpreter::Interpreter;
use crate::lexer::tokenizer::Tokenizer;
use crate::typechecker::checker::TypeChecker;

#[test]
fn test_sum_type_inference() {
    let source = "let x = if true { 42 } else { false };";

    println!("Testing: {}", source);

    let mut tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.tokenize(source).expect("Tokenization failed");

    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Parsing failed");

    let mut typechecker = TypeChecker::new();
    let typed_ast = typechecker
        .check_program(&ast)
        .expect("Type checking failed");

    println!("Type checking passed! Typed AST: {:?}", typed_ast);

    let mut interpreter = Interpreter::new();
    let result = interpreter
        .interpret_program(&ast)
        .expect("Interpretation failed");

    println!("Result: {:?}", result);
}

#[test]
fn test_explicit_sum_types() {
    let source = r#"
        let left_val = inl(42);
        let result = case left_val of
          inl x => x + 10
          | inr y => 0;
    "#;

    println!("Testing: {}", source);

    let mut tokenizer = Tokenizer::new(source);
    let tokens = tokenizer.tokenize(source).expect("Tokenization failed");

    let mut parser = Parser::new(tokens);
    let ast = parser.parse().expect("Parsing failed");

    let mut typechecker = TypeChecker::new();
    let typed_ast = typechecker
        .check_program(&ast)
        .expect("Type checking failed");

    println!("Type checking passed! Typed AST: {:?}", typed_ast);

    let mut interpreter = Interpreter::new();
    let result = interpreter
        .interpret_program(&ast)
        .expect("Interpretation failed");

    println!("Result: {:?}", result);
}
