#[cfg(test)]
mod tests {
    use crate::ast::nodes::{BinaryOperator, Expression, Program, Statement};
    use crate::interpreter::{Interpreter, InterpreterError, Value};
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
    fn test_logical_not_operation() {
        use crate::ast::nodes::UnaryOperator;
        let mut interpreter = Interpreter::new();

        // Test !true
        let not_true_expr = Expression::UnaryOp {
            operator: UnaryOperator::LogicalNot,
            operand: Box::new(Expression::Boolean {
                value: true,
                span: create_test_span(),
            }),
            span: create_test_span(),
        };
        let result = interpreter.interpret_expression(&not_true_expr).unwrap();
        assert_eq!(result, Value::Bool(false));

        // Test !false
        let not_false_expr = Expression::UnaryOp {
            operator: UnaryOperator::LogicalNot,
            operand: Box::new(Expression::Boolean {
                value: false,
                span: create_test_span(),
            }),
            span: create_test_span(),
        };
        let result = interpreter.interpret_expression(&not_false_expr).unwrap();
        assert_eq!(result, Value::Bool(true));

        // Test !!true (nested)
        let double_not_expr = Expression::UnaryOp {
            operator: UnaryOperator::LogicalNot,
            operand: Box::new(Expression::UnaryOp {
                operator: UnaryOperator::LogicalNot,
                operand: Box::new(Expression::Boolean {
                    value: true,
                    span: create_test_span(),
                }),
                span: create_test_span(),
            }),
            span: create_test_span(),
        };
        let result = interpreter.interpret_expression(&double_not_expr).unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn test_interpret_list() {
        let mut interpreter = Interpreter::new();
        let expr = Expression::List {
            elements: vec![
                Expression::Number {
                    value: 1,
                    span: create_test_span(),
                },
                Expression::Number {
                    value: 2,
                    span: create_test_span(),
                },
                Expression::Number {
                    value: 3,
                    span: create_test_span(),
                },
            ],
            span: create_test_span(),
        };

        let result = interpreter.interpret_expression(&expr).unwrap();
        assert_eq!(
            result,
            Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)])
        );
    }

    #[test]
    fn test_interpret_arithmetic() {
        let mut interpreter = Interpreter::new();

        // Test 2 + 3
        let expr = Expression::BinaryOp {
            left: Box::new(Expression::Number {
                value: 2,
                span: create_test_span(),
            }),
            operator: BinaryOperator::Add,
            right: Box::new(Expression::Number {
                value: 3,
                span: create_test_span(),
            }),
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
                    value: Expression::Number {
                        value: 42,
                        span: create_test_span(),
                    },
                    span: create_test_span(),
                },
                Statement::Expression {
                    expression: Expression::Identifier {
                        name: "x".to_string(),
                        span: create_test_span(),
                    },
                    span: create_test_span(),
                },
            ],
            create_test_span(),
        );

        // After removing the last statement return feature, programs now return Unit
        let result = interpreter.interpret_program(&program).unwrap();
        assert_eq!(result, Value::Unit);

        // Verify that the variable was properly declared and can be accessed
        let x_value = interpreter
            .interpret_expression(&Expression::Identifier {
                name: "x".to_string(),
                span: create_test_span(),
            })
            .unwrap();
        assert_eq!(x_value, Value::Int(42));
    }

    #[test]
    fn test_function_creation_and_call() {
        let mut interpreter = Interpreter::new();

        // Create function: fn(x) { x + 1 }
        let function_expr = Expression::Function {
            param: "x".to_string(),
            param_type: None,
            body: Box::new(Expression::BinaryOp {
                left: Box::new(Expression::Identifier {
                    name: "x".to_string(),
                    span: create_test_span(),
                }),
                operator: BinaryOperator::Add,
                right: Box::new(Expression::Number {
                    value: 1,
                    span: create_test_span(),
                }),
                span: create_test_span(),
            }),
            span: create_test_span(),
        };

        // Call the function with argument 5
        let call_expr = Expression::FunctionCall {
            function: Box::new(function_expr),
            argument: Box::new(Expression::Number {
                value: 5,
                span: create_test_span(),
            }),
            span: create_test_span(),
        };

        let result = interpreter.interpret_expression(&call_expr).unwrap();
        assert_eq!(result, Value::Int(6));
    }

    #[test]
    fn test_pair_creation() {
        let mut interpreter = Interpreter::new();
        let expr = Expression::Pair {
            first: Box::new(Expression::Number {
                value: 1,
                span: create_test_span(),
            }),
            second: Box::new(Expression::Boolean {
                value: true,
                span: create_test_span(),
            }),
            span: create_test_span(),
        };

        let result = interpreter.interpret_expression(&expr).unwrap();
        assert_eq!(
            result,
            Value::Pair(Box::new(Value::Int(1)), Box::new(Value::Bool(true)))
        );
    }

    #[test]
    fn test_comparison_operations() {
        let mut interpreter = Interpreter::new();

        // Test 5 > 3
        let expr = Expression::BinaryOp {
            left: Box::new(Expression::Number {
                value: 5,
                span: create_test_span(),
            }),
            operator: BinaryOperator::GreaterThan,
            right: Box::new(Expression::Number {
                value: 3,
                span: create_test_span(),
            }),
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
            left: Box::new(Expression::Number {
                value: 10,
                span: create_test_span(),
            }),
            operator: BinaryOperator::Divide,
            right: Box::new(Expression::Number {
                value: 0,
                span: create_test_span(),
            }),
            span: create_test_span(),
        };

        let result = interpreter.interpret_expression(&expr);
        assert!(result.is_err());

        match result.unwrap_err() {
            InterpreterError::DivisionByZero { .. } => (),
            _ => panic!("Expected DivisionByZero error"),
        }
    }

    #[test]
    fn test_pair_first_projection() {
        let mut interpreter = Interpreter::new();

        // Create pair (42, true)
        let pair = Expression::Pair {
            first: Box::new(Expression::Number {
                value: 42,
                span: create_test_span(),
            }),
            second: Box::new(Expression::Boolean {
                value: true,
                span: create_test_span(),
            }),
            span: create_test_span(),
        };

        // Test fst(pair)
        let fst_expr = Expression::FirstProjection {
            pair: Box::new(pair),
            span: create_test_span(),
        };

        let result = interpreter.interpret_expression(&fst_expr).unwrap();
        assert_eq!(result, Value::Int(42));
    }

    #[test]
    fn test_pair_second_projection() {
        let mut interpreter = Interpreter::new();

        // Create pair (42, true)
        let pair = Expression::Pair {
            first: Box::new(Expression::Number {
                value: 42,
                span: create_test_span(),
            }),
            second: Box::new(Expression::Boolean {
                value: true,
                span: create_test_span(),
            }),
            span: create_test_span(),
        };

        // Test snd(pair)
        let snd_expr = Expression::SecondProjection {
            pair: Box::new(pair),
            span: create_test_span(),
        };

        let result = interpreter.interpret_expression(&snd_expr).unwrap();
        assert_eq!(result, Value::Bool(true));
    }

    #[test]
    fn test_pair_projection_type_error() {
        let mut interpreter = Interpreter::new();

        // Try to project from a non-pair (integer)
        let fst_expr = Expression::FirstProjection {
            pair: Box::new(Expression::Number {
                value: 42,
                span: create_test_span(),
            }),
            span: create_test_span(),
        };

        let result = interpreter.interpret_expression(&fst_expr);
        assert!(result.is_err());

        match result.unwrap_err() {
            InterpreterError::TypeError {
                expected, found, ..
            } => {
                assert_eq!(expected, "Pair");
                assert_eq!(found, "Int");
            }
            _ => panic!("Expected TypeError"),
        }
    }

    #[test]
    fn test_cons_operation() {
        let mut interpreter = Interpreter::new();

        // Create cons(42, [1, 2, 3])
        let list = Expression::List {
            elements: vec![
                Expression::Number {
                    value: 1,
                    span: create_test_span(),
                },
                Expression::Number {
                    value: 2,
                    span: create_test_span(),
                },
                Expression::Number {
                    value: 3,
                    span: create_test_span(),
                },
            ],
            span: create_test_span(),
        };

        let cons_expr = Expression::Cons {
            head: Box::new(Expression::Number {
                value: 42,
                span: create_test_span(),
            }),
            tail: Box::new(list),
            span: create_test_span(),
        };

        let result = interpreter.interpret_expression(&cons_expr).unwrap();
        assert_eq!(
            result,
            Value::List(vec![
                Value::Int(42),
                Value::Int(1),
                Value::Int(2),
                Value::Int(3)
            ])
        );
    }

    #[test]
    fn test_head_projection() {
        let mut interpreter = Interpreter::new();

        // Create head([10, 20, 30])
        let list = Expression::List {
            elements: vec![
                Expression::Number {
                    value: 10,
                    span: create_test_span(),
                },
                Expression::Number {
                    value: 20,
                    span: create_test_span(),
                },
                Expression::Number {
                    value: 30,
                    span: create_test_span(),
                },
            ],
            span: create_test_span(),
        };

        let head_expr = Expression::HeadProjection {
            list: Box::new(list),
            span: create_test_span(),
        };

        let result = interpreter.interpret_expression(&head_expr).unwrap();
        assert_eq!(result, Value::Int(10));
    }

    #[test]
    fn test_tail_projection() {
        let mut interpreter = Interpreter::new();

        // Create tail([10, 20, 30])
        let list = Expression::List {
            elements: vec![
                Expression::Number {
                    value: 10,
                    span: create_test_span(),
                },
                Expression::Number {
                    value: 20,
                    span: create_test_span(),
                },
                Expression::Number {
                    value: 30,
                    span: create_test_span(),
                },
            ],
            span: create_test_span(),
        };

        let tail_expr = Expression::TailProjection {
            list: Box::new(list),
            span: create_test_span(),
        };

        let result = interpreter.interpret_expression(&tail_expr).unwrap();
        assert_eq!(result, Value::List(vec![Value::Int(20), Value::Int(30)]));
    }

    #[test]
    fn test_head_on_empty_list_error() {
        let mut interpreter = Interpreter::new();

        // Create head([])
        let empty_list = Expression::List {
            elements: vec![],
            span: create_test_span(),
        };

        let head_expr = Expression::HeadProjection {
            list: Box::new(empty_list),
            span: create_test_span(),
        };

        let result = interpreter.interpret_expression(&head_expr);
        assert!(result.is_err());

        match result.unwrap_err() {
            InterpreterError::RuntimeError { message, .. } => {
                assert_eq!(message, "Cannot get head of empty list");
            }
            _ => panic!("Expected RuntimeError for empty list head"),
        }
    }

    #[test]
    fn test_tail_on_empty_list_error() {
        let mut interpreter = Interpreter::new();

        // Create tail([])
        let empty_list = Expression::List {
            elements: vec![],
            span: create_test_span(),
        };

        let tail_expr = Expression::TailProjection {
            list: Box::new(empty_list),
            span: create_test_span(),
        };

        let result = interpreter.interpret_expression(&tail_expr);
        assert!(result.is_err());

        match result.unwrap_err() {
            InterpreterError::RuntimeError { message, .. } => {
                assert_eq!(message, "Cannot get tail of empty list");
            }
            _ => panic!("Expected RuntimeError for empty list tail"),
        }
    }

    #[test]
    fn test_cons_type_error() {
        let mut interpreter = Interpreter::new();

        // Try to cons with a non-list (integer) as tail
        let cons_expr = Expression::Cons {
            head: Box::new(Expression::Number {
                value: 42,
                span: create_test_span(),
            }),
            tail: Box::new(Expression::Number {
                value: 123,
                span: create_test_span(),
            }),
            span: create_test_span(),
        };

        let result = interpreter.interpret_expression(&cons_expr);
        assert!(result.is_err());

        match result.unwrap_err() {
            InterpreterError::TypeError {
                expected, found, ..
            } => {
                assert_eq!(expected, "List");
                assert_eq!(found, "Int");
            }
            _ => panic!("Expected TypeError for cons with non-list tail"),
        }
    }

    #[test]
    fn test_head_type_error() {
        let mut interpreter = Interpreter::new();

        // Try to get head from a non-list (integer)
        let head_expr = Expression::HeadProjection {
            list: Box::new(Expression::Number {
                value: 42,
                span: create_test_span(),
            }),
            span: create_test_span(),
        };

        let result = interpreter.interpret_expression(&head_expr);
        assert!(result.is_err());

        match result.unwrap_err() {
            InterpreterError::TypeError {
                expected, found, ..
            } => {
                assert_eq!(expected, "List");
                assert_eq!(found, "Int");
            }
            _ => panic!("Expected TypeError for head on non-list"),
        }
    }

    #[test]
    fn test_tail_type_error() {
        let mut interpreter = Interpreter::new();

        // Try to get tail from a non-list (boolean)
        let tail_expr = Expression::TailProjection {
            list: Box::new(Expression::Boolean {
                value: true,
                span: create_test_span(),
            }),
            span: create_test_span(),
        };

        let result = interpreter.interpret_expression(&tail_expr);
        assert!(result.is_err());

        match result.unwrap_err() {
            InterpreterError::TypeError {
                expected, found, ..
            } => {
                assert_eq!(expected, "List");
                assert_eq!(found, "Bool");
            }
            _ => panic!("Expected TypeError for tail on non-list"),
        }
    }

    #[test]
    fn test_nested_list_operations() {
        let mut interpreter = Interpreter::new();

        // Test head(tail(cons(0, [1, 2, 3])))
        // Should return 1 (head of [1, 2, 3])

        let original_list = Expression::List {
            elements: vec![
                Expression::Number {
                    value: 1,
                    span: create_test_span(),
                },
                Expression::Number {
                    value: 2,
                    span: create_test_span(),
                },
                Expression::Number {
                    value: 3,
                    span: create_test_span(),
                },
            ],
            span: create_test_span(),
        };

        let cons_expr = Expression::Cons {
            head: Box::new(Expression::Number {
                value: 0,
                span: create_test_span(),
            }),
            tail: Box::new(original_list),
            span: create_test_span(),
        };

        let tail_expr = Expression::TailProjection {
            list: Box::new(cons_expr),
            span: create_test_span(),
        };

        let head_expr = Expression::HeadProjection {
            list: Box::new(tail_expr),
            span: create_test_span(),
        };

        let result = interpreter.interpret_expression(&head_expr).unwrap();
        assert_eq!(result, Value::Int(1));
    }

    #[test]
    fn test_print_expression() {
        let mut interpreter = Interpreter::new();

        // Test printing an integer
        let print_expr = Expression::Print {
            value: Box::new(Expression::Number {
                value: 42,
                span: create_test_span(),
            }),
            span: create_test_span(),
        };

        let result = interpreter.interpret_expression(&print_expr).unwrap();
        assert_eq!(result, Value::Unit);

        // Test printing a list
        let print_list_expr = Expression::Print {
            value: Box::new(Expression::List {
                elements: vec![
                    Expression::Number {
                        value: 1,
                        span: create_test_span(),
                    },
                    Expression::Number {
                        value: 2,
                        span: create_test_span(),
                    },
                ],
                span: create_test_span(),
            }),
            span: create_test_span(),
        };

        let result = interpreter.interpret_expression(&print_list_expr).unwrap();
        assert_eq!(result, Value::Unit);
    }

    #[test]
    fn test_multiline_function_body() {
        let mut interpreter = Interpreter::new();

        // Create a function with a multiline body: fn(x) { print(x); x + 10 }
        let multiline_func = Expression::Function {
            param: "x".to_string(),
            param_type: None,
            body: Box::new(Expression::Block {
                statements: vec![
                    Statement::Expression {
                        expression: Expression::Print {
                            value: Box::new(Expression::Identifier {
                                name: "x".to_string(),
                                span: create_test_span(),
                            }),
                            span: create_test_span(),
                        },
                        span: create_test_span(),
                    },
                ],
                expression: Some(Box::new(Expression::BinaryOp {
                    left: Box::new(Expression::Identifier {
                        name: "x".to_string(),
                        span: create_test_span(),
                    }),
                    operator: crate::ast::nodes::BinaryOperator::Add,
                    right: Box::new(Expression::Number {
                        value: 10,
                        span: create_test_span(),
                    }),
                    span: create_test_span(),
                })),
                span: create_test_span(),
            }),
            span: create_test_span(),
        };

        // Verify the function can be created
        let _func_value = interpreter.interpret_expression(&multiline_func).unwrap();
        
        // Call the function with argument 5
        let call_expr = Expression::FunctionCall {
            function: Box::new(multiline_func),
            argument: Box::new(Expression::Number {
                value: 5,
                span: create_test_span(),
            }),
            span: create_test_span(),
        };

        let result = interpreter.interpret_expression(&call_expr).unwrap();
        // Should return 15 (5 + 10) and print 5 as a side effect
        assert_eq!(result, Value::Int(15));
    }
}
