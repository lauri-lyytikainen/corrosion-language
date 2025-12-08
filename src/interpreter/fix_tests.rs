#[cfg(test)]
mod fix_tests {
    use crate::ast::{Parser, nodes::*};
    use crate::interpreter::Interpreter;
    use crate::lexer::Tokenizer;
    use crate::typechecker::TypeChecker;

    #[test]
    fn test_fix_expression_parsing() {
        let input = "fix(fn(f) { fn(x) { x } });";

        let mut tokenizer = Tokenizer::new("");
        let tokens = tokenizer.tokenize(input).unwrap();

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        assert_eq!(program.statements.len(), 1);

        if let Statement::Expression { expression, .. } = &program.statements[0] {
            if let Expression::Fix { .. } = expression {
            } else {
                panic!("Expected Fix expression, got {:?}", expression);
            }
        } else {
            panic!("Expected expression statement");
        }
    }

    #[test]
    fn test_fix_expression_interpretation() {
        let input = "let identity = fix(fn(f) { fn(x) { x } }); print(identity(42));";

        let mut tokenizer = Tokenizer::new("");
        let tokens = tokenizer.tokenize(input).unwrap();

        let mut parser = Parser::new(tokens);
        let program = parser.parse().unwrap();

        let mut type_checker = TypeChecker::new();
        let _typed_program = type_checker.check_program(&program).unwrap();

        let mut interpreter = Interpreter::new();
        let result = interpreter.interpret_program(&program);

        assert!(result.is_ok());
    }
}
