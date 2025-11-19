#[cfg(test)]
mod tests {
    use super::*;
    use crate::interpreter::{Interpreter, Value, InterpreterError};
    use crate::ast::nodes::{Expression, Program, Statement, BinaryOperator};
    use crate::lexer::tokens::Span;

    fn create_test_span() -> Span {
        Span::new(0, 1, 1, 1)
    }

    #[test]
    fn test_interpret_number() {
        let mut interpreter = Interpreter::new();
        let expr = Expression::Number {
            value: 42,
            span: create_test_span(),
        };
        
        let result = interpreter.interpret_expression(&expr).unwrap();
        assert_eq!(result, Value::Int(42));
    }

    #[test]
    fn test_interpret_boolean() {
        let mut interpreter = Interpreter::new();
        let expr = Expression::Boolean {
            value: true,
            span: create_test_span(),
        };
        
        let result = interpreter.interpret_expression(&expr).unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn test_interpret_list() {
        let mut interpreter = Interpreter::new();
        let expr = Expression::List {
            elements: vec![
                Expression::Number { value: 1, span: create_test_span() },
                Expression::Number { value: 2, span: create_test_span() },
                Expression::Number { value: 3, span: create_test_span() },
            ],
            span: create_test_span(),
        };
        
        let result = interpreter.interpret_expression(&expr).unwrap();
        assert_eq!(result, Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)]));
    }

    #[test]
    fn test_interpret_arithmetic() {
        let mut interpreter = Interpreter::new();
        
        // Test 2 + 3
        let expr = Expression::BinaryOp {
            left: Box::new(Expression::Number { value: 2, span: create_test_span() }),
            operator: BinaryOperator::Add,
            right: Box::new(Expression::Number { value: 3, span: create_test_span() }),
            span: create_test_span(),
        };
        
        let result = interpreter.interpret_expression(&expr).unwrap();
        assert_eq!(result, Value::Int(5));
    }

    #[test]
    fn test_variable_declaration_and_usage() {
        let mut interpreter = Interpreter::new();
        
        // Create program: let x = 42; x;
        let program = Program::new(
            vec![
                Statement::VariableDeclaration {
                    name: "x".to_string(),
                    type_annotation: None,
                    value: Expression::Number { value: 42, span: create_test_span() },
                    span: create_test_span(),
                },
                Statement::Expression {
                    expression: Expression::Identifier { 
                        name: "x".to_string(), 
                        span: create_test_span() 
                    },
                    span: create_test_span(),
                },
            ],
            create_test_span(),
        );
        
        let result = interpreter.interpret_program(&program).unwrap();
        assert_eq!(result, Value::Int(42));
    }

    #[test]
    fn test_function_creation_and_call() {
        let mut interpreter = Interpreter::new();
        
        // Create function: fn(x) { x + 1 }
        let function_expr = Expression::Function {
            param: "x".to_string(),
            body: Box::new(Expression::BinaryOp {
                left: Box::new(Expression::Identifier { 
                    name: "x".to_string(), 
                    span: create_test_span() 
                }),
                operator: BinaryOperator::Add,
                right: Box::new(Expression::Number { value: 1, span: create_test_span() }),
                span: create_test_span(),
            }),
            span: create_test_span(),
        };
        
        // Call the function with argument 5
        let call_expr = Expression::FunctionCall {
            function: Box::new(function_expr),
            argument: Box::new(Expression::Number { value: 5, span: create_test_span() }),
            span: create_test_span(),
        };
        
        let result = interpreter.interpret_expression(&call_expr).unwrap();
        assert_eq!(result, Value::Int(6));
    }

    #[test]
    fn test_pair_creation() {
        let mut interpreter = Interpreter::new();
        let expr = Expression::Pair {
            first: Box::new(Expression::Number { value: 1, span: create_test_span() }),
            second: Box::new(Expression::Boolean { value: true, span: create_test_span() }),
            span: create_test_span(),
        };
        
        let result = interpreter.interpret_expression(&expr).unwrap();
        assert_eq!(result, Value::Pair(Box::new(Value::Int(1)), Box::new(Value::Bool(true))));
    }

    #[test]
    fn test_comparison_operations() {
        let mut interpreter = Interpreter::new();
        
        // Test 5 > 3
        let expr = Expression::BinaryOp {
            left: Box::new(Expression::Number { value: 5, span: create_test_span() }),
            operator: BinaryOperator::GreaterThan,
            right: Box::new(Expression::Number { value: 3, span: create_test_span() }),
            span: create_test_span(),
        };
        
        let result = interpreter.interpret_expression(&expr).unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn test_undefined_variable_error() {
        let mut interpreter = Interpreter::new();
        let expr = Expression::Identifier {
            name: "undefined_var".to_string(),
            span: create_test_span(),
        };
        
        let result = interpreter.interpret_expression(&expr);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            InterpreterError::UndefinedVariable { name, .. } => {
                assert_eq!(name, "undefined_var");
            }
            _ => panic!("Expected UndefinedVariable error"),
        }
    }

    #[test]
    fn test_division_by_zero() {
        let mut interpreter = Interpreter::new();
        let expr = Expression::BinaryOp {
            left: Box::new(Expression::Number { value: 10, span: create_test_span() }),
            operator: BinaryOperator::Divide,
            right: Box::new(Expression::Number { value: 0, span: create_test_span() }),
            span: create_test_span(),
        };
        
        let result = interpreter.interpret_expression(&expr);
        assert!(result.is_err());
        
        match result.unwrap_err() {
            InterpreterError::DivisionByZero { .. } => (),
            _ => panic!("Expected DivisionByZero error"),
        }
    }
}