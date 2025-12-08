use crate::ast::{Expression, Program, Spanned, Statement, TypeExpression};
use crate::typechecker::{
    BinaryOp, Environment, ModuleLoader, Type, TypeCompatibility, TypeError, TypeInference,
    TypeResult, TypedExpression, TypedProgram, TypedStatement,
};
use std::path::Path;

/// Type checker for the Corrosion language
pub struct TypeChecker {
    environment: Environment,
    errors: Vec<TypeError>,
    module_loader: ModuleLoader,
}

impl TypeChecker {
    /// Create a new type checker
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
            errors: Vec::new(),
            module_loader: ModuleLoader::new(),
        }
    }

    /// Set the current directory for import resolution
    pub fn set_current_directory<P: AsRef<Path>>(&mut self, path: P) {
        self.module_loader.set_current_directory(path);
    }

    /// Type check a program and return the typed AST
    pub fn check_program(&mut self, program: &Program) -> TypeResult<TypedProgram> {
        let mut typed_statements = Vec::new();

        for statement in &program.statements {
            match self.check_statement(statement) {
                Ok(typed_stmt) => typed_statements.push(typed_stmt),
                Err(err) => {
                    self.errors.push(err.clone());
                    return Err(err);
                }
            }
        }

        Ok(TypedProgram::new(typed_statements, program.span.clone()))
    }

    /// Type check a statement
    fn check_statement(&mut self, statement: &Statement) -> TypeResult<TypedStatement> {
        match statement {
            Statement::VariableDeclaration {
                name,
                type_annotation,
                value,
                span,
            } => {
                // Check if variable is already defined in current scope
                if self.environment.is_bound_locally(name) {
                    return Err(TypeError::RedefinedVariable {
                        name: name.clone(),
                        span: span.clone(),
                    });
                }

                // Type check the value expression
                let typed_value = self.check_expression(value)?;
                let inferred_type = typed_value.ty.clone();

                let final_type = if let Some(annotation) = type_annotation {
                    let annotated_type = self.convert_type_expression(annotation)?;

                    // Special handling for function types with Unknown parameters/results
                    let refined_type = TypeCompatibility::refine_type_with_annotation(
                        &inferred_type,
                        &annotated_type,
                    )?;

                    if !TypeCompatibility::types_compatible(&annotated_type, &refined_type) {
                        return Err(TypeError::TypeMismatch {
                            expected: annotated_type,
                            found: refined_type,
                            span: span.clone(),
                        });
                    }
                    annotated_type
                } else {
                    inferred_type
                };

                // Bind the variable to its type
                self.environment.bind(name.clone(), final_type.clone());

                Ok(TypedStatement::VariableDeclaration {
                    name: name.clone(),
                    ty: final_type,
                    value: typed_value,
                    span: span.clone(),
                })
            }
            Statement::FunctionDeclaration {
                name,
                param,
                param_type,
                return_type,
                body,
                span,
            } => {
                // Check if function is already defined in current scope
                if self.environment.is_bound_locally(name) {
                    return Err(TypeError::RedefinedVariable {
                        name: name.clone(),
                        span: span.clone(),
                    });
                }

                // Use explicit parameter type if provided, otherwise Unknown for inference
                let param_type = if let Some(param_type_expr) = param_type {
                    self.convert_type_expression(param_type_expr)?
                } else {
                    Type::Unknown
                };

                // Convert return type annotation if provided
                let expected_return_type = return_type
                    .as_ref()
                    .map(|rt| self.convert_type_expression(rt))
                    .transpose()?;

                // Create preliminary function type for recursive calls
                let preliminary_return_type = expected_return_type.clone().unwrap_or(Type::Unknown);
                let preliminary_function_type =
                    Type::function(param_type.clone(), preliminary_return_type);

                // Bind the function name BEFORE checking the body (enables recursion)
                self.environment
                    .bind(name.clone(), preliminary_function_type);

                // Create a new scope for the function body
                self.environment.enter_scope();

                // Bind the parameter in the function body scope
                self.environment.bind(param.clone(), param_type.clone());

                // Type check the function body
                let typed_body = self.check_expression(body)?;
                let actual_return_type = typed_body.ty.clone();

                // Check return type matches annotation if provided
                let final_return_type = if let Some(expected) = expected_return_type {
                    if !TypeCompatibility::types_compatible(&expected, &actual_return_type) {
                        self.environment.exit_scope();
                        return Err(TypeError::TypeMismatch {
                            expected,
                            found: actual_return_type,
                            span: span.clone(),
                        });
                    }
                    expected
                } else {
                    actual_return_type
                };

                self.environment.exit_scope();

                // Update the function type with the actual return type
                let final_function_type =
                    Type::function(param_type.clone(), final_return_type.clone());
                self.environment.update(name.clone(), final_function_type);

                Ok(TypedStatement::FunctionDeclaration {
                    name: name.clone(),
                    param: param.clone(),
                    param_type,
                    return_type: final_return_type,
                    body: typed_body,
                    span: span.clone(),
                })
            }

            Statement::Import { path, alias, span } => {
                let import_name = alias.as_ref().unwrap_or(path);

                // Load and type-check the module
                let module_exports =
                    self.module_loader
                        .load_and_check_module(path, import_name, span)?;

                // Store the module's exports for later lookup
                self.module_loader
                    .store_module_exports(import_name.clone(), module_exports);

                Ok(TypedStatement::Import {
                    path: path.clone(),
                    alias: alias.clone(),
                    span: span.clone(),
                })
            }
            Statement::Expression { expression, span } => {
                let typed_expr = self.check_expression(expression)?;
                Ok(TypedStatement::Expression {
                    expression: typed_expr,
                    span: span.clone(),
                })
            }
        }
    }

    /// Type check an expression
    pub fn check_expression(&mut self, expression: &Expression) -> TypeResult<TypedExpression> {
        match expression {
            Expression::Number { value: _, span } => {
                Ok(TypedExpression::new(Type::Int, span.clone()))
            }
            Expression::Boolean { value: _, span } => {
                Ok(TypedExpression::new(Type::Bool, span.clone()))
            }
            Expression::String { value: _, span } => {
                Ok(TypedExpression::new(Type::String, span.clone()))
            }
            Expression::Identifier { name, span } => match self.environment.lookup(name) {
                Some(ty) => Ok(TypedExpression::new(ty.clone(), span.clone())),
                None => Err(TypeError::UndefinedVariable {
                    name: name.clone(),
                    span: span.clone(),
                }),
            },
            Expression::QualifiedIdentifier { module, name, span } => {
                // Look up the module's exports
                if let Some(module_exports) = self.module_loader.get_module_exports(module) {
                    if let Some(export_type) = module_exports.get(name) {
                        Ok(TypedExpression::new(export_type.clone(), span.clone()))
                    } else {
                        Err(TypeError::UndefinedVariable {
                            name: format!("{}.{}", module, name),
                            span: span.clone(),
                        })
                    }
                } else {
                    Err(TypeError::ImportError {
                        message: format!("Module '{}' not found", module),
                        path: module.clone(),
                        span: span.clone(),
                    })
                }
            }
            Expression::BinaryOp {
                left,
                operator,
                right,
                span,
            } => {
                let typed_left = self.check_expression(left)?;
                let typed_right = self.check_expression(right)?;

                let op = BinaryOp::from(operator.clone());

                match typed_left.ty.can_binary_op(&op, &typed_right.ty) {
                    Some(result_type) => Ok(TypedExpression::new(result_type, span.clone())),
                    None => Err(TypeError::InvalidBinaryOperation {
                        left: typed_left.ty,
                        op,
                        right: typed_right.ty,
                        span: span.clone(),
                    }),
                }
            }
            Expression::UnaryOp {
                operator,
                operand,
                span,
            } => {
                let typed_operand = self.check_expression(operand)?;

                match operator {
                    crate::ast::nodes::UnaryOperator::LogicalNot => {
                        if typed_operand.ty == Type::Bool {
                            Ok(TypedExpression::new(Type::Bool, span.clone()))
                        } else {
                            Err(TypeError::TypeMismatch {
                                expected: Type::Bool,
                                found: typed_operand.ty,
                                span: span.clone(),
                            })
                        }
                    }
                    crate::ast::nodes::UnaryOperator::Negate => {
                        if typed_operand.ty == Type::Int {
                            Ok(TypedExpression::new(Type::Int, span.clone()))
                        } else {
                            Err(TypeError::TypeMismatch {
                                expected: Type::Int,
                                found: typed_operand.ty,
                                span: span.clone(),
                            })
                        }
                    }
                }
            }
            Expression::Function {
                param,
                param_type,
                body,
                span,
            } => {
                // Use explicit parameter type if provided, otherwise infer
                let param_type = if let Some(param_type_expr) = param_type {
                    self.convert_type_expression(param_type_expr)?
                } else {
                    let inference = TypeInference::new(self.environment.clone());
                    inference.infer_parameter_type(param, body)?
                };

                let mut function_checker = TypeChecker {
                    environment: Environment::with_parent(self.environment.clone()),
                    errors: Vec::new(),
                    module_loader: ModuleLoader::new(),
                };
                function_checker
                    .module_loader
                    .set_current_directory(self.module_loader.get_current_directory());

                // Bind the parameter in the function's scope
                function_checker
                    .environment
                    .bind(param.clone(), param_type.clone());

                // Type check the function body
                let typed_body = function_checker.check_expression(body)?;

                // Create the function type
                let function_type = Type::Function {
                    param: Box::new(param_type),
                    result: Box::new(typed_body.ty.clone()),
                };

                Ok(TypedExpression::new(function_type, span.clone()))
            }
            Expression::FunctionCall {
                function,
                argument,
                span,
            } => {
                let function_typed = self.check_expression(function)?;
                let argument_typed = self.check_expression(argument)?;

                match &function_typed.ty {
                    Type::Function { param, result } => {
                        // Enhanced type checking with Unknown type handling
                        let refined_param =
                            TypeCompatibility::refine_type_with_context(param, &argument_typed.ty);
                        let refined_result =
                            TypeCompatibility::refine_type_with_context(result, &Type::Unknown);

                        let is_compatible = TypeCompatibility::types_compatible(
                            &argument_typed.ty,
                            &refined_param,
                        ) || (argument_typed.ty == Type::Unknown
                            && matches!(refined_param, Type::List { .. }))
                            || matches!(&argument_typed.ty, Type::List { element } if **element == Type::Unknown)
                            || matches!(&argument_typed.ty, Type::Sum { left, right } if **left == Type::Unknown || **right == Type::Unknown)
                            || matches!(&refined_param, Type::Sum { left, right } if **left == Type::Unknown || **right == Type::Unknown);

                        if is_compatible {
                            Ok(TypedExpression::new(refined_result, span.clone()))
                        } else {
                            Err(TypeError::TypeMismatch {
                                expected: refined_param,
                                found: argument_typed.ty.clone(),
                                span: span.clone(),
                            })
                        }
                    }
                    Type::Unknown => Ok(TypedExpression::new(Type::Unknown, span.clone())),
                    _ => Err(TypeError::TypeMismatch {
                        expected: Type::Function {
                            param: Box::new(Type::Unknown),
                            result: Box::new(Type::Unknown),
                        },
                        found: function_typed.ty.clone(),
                        span: span.clone(),
                    }),
                }
            }
            Expression::List { elements, span } => {
                if elements.is_empty() {
                    // Empty list - we can't infer the element type, so return a generic list type
                    // In a more sophisticated system, we'd use type variables
                    Ok(TypedExpression::new(
                        Type::List {
                            element: Box::new(Type::Unknown),
                        },
                        span.clone(),
                    ))
                } else {
                    // Type check all elements and ensure they're the same type
                    let typed_elements: Result<Vec<_>, _> = elements
                        .iter()
                        .map(|elem| self.check_expression(elem))
                        .collect();
                    let typed_elements = typed_elements?;

                    // Get the type of the first element
                    let element_type = &typed_elements[0].ty;

                    // Check that all elements have the same type
                    for (i, typed_elem) in typed_elements.iter().enumerate().skip(1) {
                        if !typed_elem.ty.is_assignable_to(element_type) {
                            return Err(TypeError::TypeMismatch {
                                expected: element_type.clone(),
                                found: typed_elem.ty.clone(),
                                span: elements[i].span().clone(),
                            });
                        }
                    }

                    Ok(TypedExpression::new(
                        Type::List {
                            element: Box::new(element_type.clone()),
                        },
                        span.clone(),
                    ))
                }
            }
            Expression::Pair {
                first,
                second,
                span,
            } => {
                let typed_first = self.check_expression(first)?;
                let typed_second = self.check_expression(second)?;

                let pair_type = Type::Pair {
                    first: Box::new(typed_first.ty.clone()),
                    second: Box::new(typed_second.ty.clone()),
                };

                Ok(TypedExpression::new(pair_type, span.clone()))
            }
            Expression::LeftInject { value, span } => {
                let typed_value = self.check_expression(value)?;
                Ok(TypedExpression::new(
                    Type::Sum {
                        left: Box::new(typed_value.ty),
                        right: Box::new(Type::Unknown),
                    },
                    span.clone(),
                ))
            }
            Expression::RightInject { value, span } => {
                let typed_value = self.check_expression(value)?;
                Ok(TypedExpression::new(
                    Type::Sum {
                        left: Box::new(Type::Unknown),
                        right: Box::new(typed_value.ty),
                    },
                    span.clone(),
                ))
            }
            Expression::Case {
                expression,
                left_pattern,
                left_body,
                right_pattern,
                right_body,
                span,
            } => {
                let typed_expr = self.check_expression(expression)?;

                match &typed_expr.ty {
                    Type::Sum { left, right } => {
                        // Check left branch
                        let mut left_checker = TypeChecker {
                            environment: Environment::with_parent(self.environment.clone()),
                            errors: Vec::new(),
                            module_loader: ModuleLoader::new(),
                        };
                        left_checker
                            .module_loader
                            .set_current_directory(self.module_loader.get_current_directory());
                        // Handle Unknown type in sum (from inference)
                        let left_type = if **left == Type::Unknown {
                            Type::Unknown
                        } else {
                            *left.clone()
                        };

                        left_checker
                            .environment
                            .bind(left_pattern.clone(), left_type);
                        let typed_left_body = left_checker.check_expression(left_body)?;

                        // Check right branch
                        let mut right_checker = TypeChecker {
                            environment: Environment::with_parent(self.environment.clone()),
                            errors: Vec::new(),
                            module_loader: ModuleLoader::new(),
                        };
                        right_checker
                            .module_loader
                            .set_current_directory(self.module_loader.get_current_directory());
                        let right_type = if **right == Type::Unknown {
                            Type::Unknown
                        } else {
                            *right.clone()
                        };

                        right_checker
                            .environment
                            .bind(right_pattern.clone(), right_type);
                        let typed_right_body = right_checker.check_expression(right_body)?;

                        // Ensure branches return compatible types
                        if TypeCompatibility::types_compatible(
                            &typed_left_body.ty,
                            &typed_right_body.ty,
                        ) {
                            // If one is Unknown, prefer the other
                            let result_type = if typed_left_body.ty == Type::Unknown {
                                typed_right_body.ty
                            } else {
                                typed_left_body.ty
                            };
                            Ok(TypedExpression::new(result_type, span.clone()))
                        } else {
                            Err(TypeError::TypeMismatch {
                                expected: typed_left_body.ty,
                                found: typed_right_body.ty,
                                span: right_body.span().clone(),
                            })
                        }
                    }
                    _ => Err(TypeError::TypeMismatch {
                        expected: Type::Sum {
                            left: Box::new(Type::Unknown),
                            right: Box::new(Type::Unknown),
                        },
                        found: typed_expr.ty.clone(),
                        span: expression.span().clone(),
                    }),
                }
            }
            Expression::Fix { function, span } => {
                // Type check the function expression
                let func_typed = self.check_expression(function)?;

                // The function should have type (T -> T) -> T for some T
                // Where T is typically a function type for recursive functions
                match &func_typed.ty {
                    Type::Function { param, result } => {
                        // For fix(f), where f : (T -> T) -> (T -> T)
                        // The result should be of type T -> T
                        match (param.as_ref(), result.as_ref()) {
                            (
                                Type::Function {
                                    param: inner_param,
                                    result: inner_result,
                                },
                                Type::Function {
                                    param: outer_param,
                                    result: outer_result,
                                },
                            ) => {
                                // More flexible type checking for recursive functions
                                if TypeCompatibility::types_compatible(inner_param, inner_result)
                                    && TypeCompatibility::types_compatible(
                                        outer_param,
                                        outer_result,
                                    )
                                    && TypeCompatibility::types_compatible(inner_param, outer_param)
                                {
                                    // Return the fixed point type T -> T
                                    Ok(TypedExpression::new(
                                        Type::Function {
                                            param: outer_param.clone(),
                                            result: outer_result.clone(),
                                        },
                                        span.clone(),
                                    ))
                                } else {
                                    // For more flexible cases, return the outer function type
                                    Ok(TypedExpression::new(
                                        Type::Function {
                                            param: outer_param.clone(),
                                            result: outer_result.clone(),
                                        },
                                        span.clone(),
                                    ))
                                }
                            }
                            _ => {
                                // For simpler cases, just return the result type of the function
                                Ok(TypedExpression::new(result.as_ref().clone(), span.clone()))
                            }
                        }
                    }
                    _ => {
                        self.errors.push(TypeError::TypeMismatch {
                            expected: Type::Function {
                                param: Box::new(Type::Unknown),
                                result: Box::new(Type::Unknown),
                            },
                            found: func_typed.ty.clone(),
                            span: span.clone(),
                        });
                        Ok(TypedExpression::new(Type::Error, span.clone()))
                    }
                }
            }
            Expression::Block {
                statements,
                expression,
                span,
            } => {
                // Create a new type checker with a child environment for the block scope
                let mut block_checker = TypeChecker {
                    environment: Environment::with_parent(self.environment.clone()),
                    errors: Vec::new(),
                    module_loader: ModuleLoader::new(),
                };
                block_checker
                    .module_loader
                    .set_current_directory(self.module_loader.get_current_directory());

                // Check all statements in the block
                for stmt in statements {
                    block_checker.check_statement(stmt)?;
                }

                // Return the type of the final expression, or Unit if none
                if let Some(expr) = expression {
                    block_checker.check_expression(expr)
                } else {
                    Ok(TypedExpression::new(Type::Unit, span.clone()))
                }
            }
            Expression::FirstProjection { pair, span } => {
                let pair_typed = self.check_expression(pair)?;
                match &pair_typed.ty {
                    Type::Pair { first, .. } => {
                        Ok(TypedExpression::new((**first).clone(), span.clone()))
                    }
                    _ => Err(TypeError::TypeMismatch {
                        expected: Type::Pair {
                            first: Box::new(Type::Error),
                            second: Box::new(Type::Error),
                        },
                        found: pair_typed.ty.clone(),
                        span: span.clone(),
                    }),
                }
            }
            Expression::SecondProjection { pair, span } => {
                let pair_typed = self.check_expression(pair)?;
                match &pair_typed.ty {
                    Type::Pair { second, .. } => {
                        Ok(TypedExpression::new((**second).clone(), span.clone()))
                    }
                    _ => Err(TypeError::TypeMismatch {
                        expected: Type::Pair {
                            first: Box::new(Type::Error),
                            second: Box::new(Type::Error),
                        },
                        found: pair_typed.ty.clone(),
                        span: span.clone(),
                    }),
                }
            }
            Expression::Cons { head, tail, span } => {
                let head_typed = self.check_expression(head)?;
                let tail_typed = self.check_expression(tail)?;

                match &tail_typed.ty {
                    Type::List { element } => {
                        // Check if head type matches the list element type
                        if TypeCompatibility::types_compatible(&head_typed.ty, element) {
                            Ok(TypedExpression::new(tail_typed.ty.clone(), span.clone()))
                        } else {
                            Err(TypeError::TypeMismatch {
                                expected: (**element).clone(),
                                found: head_typed.ty.clone(),
                                span: head.span().clone(),
                            })
                        }
                    }
                    _ => Err(TypeError::TypeMismatch {
                        expected: Type::List {
                            element: Box::new(Type::Unknown),
                        },
                        found: tail_typed.ty.clone(),
                        span: tail.span().clone(),
                    }),
                }
            }
            Expression::HeadProjection { list, span } => {
                let list_typed = self.check_expression(list)?;
                match &list_typed.ty {
                    Type::List { element } => {
                        Ok(TypedExpression::new((**element).clone(), span.clone()))
                    }
                    _ => Err(TypeError::TypeMismatch {
                        expected: Type::List {
                            element: Box::new(Type::Unknown),
                        },
                        found: list_typed.ty.clone(),
                        span: span.clone(),
                    }),
                }
            }
            Expression::TailProjection { list, span } => {
                let list_typed = self.check_expression(list)?;
                match &list_typed.ty {
                    Type::List { .. } => {
                        // Tail of a list has the same type as the original list
                        Ok(TypedExpression::new(list_typed.ty.clone(), span.clone()))
                    }
                    _ => Err(TypeError::TypeMismatch {
                        expected: Type::List {
                            element: Box::new(Type::Unknown),
                        },
                        found: list_typed.ty.clone(),
                        span: span.clone(),
                    }),
                }
            }
            Expression::Print { value, span } => {
                // Type check the value being printed (but we don't need the result)
                let _ = self.check_expression(value)?;
                // Print always returns Unit type
                Ok(TypedExpression::new(Type::Unit, span.clone()))
            }
            Expression::For {
                variable,
                iterable,
                body,
                span,
            } => {
                let iterable_typed = self.check_expression(iterable)?;

                // Ensure iterable is a list
                let element_type = match &iterable_typed.ty {
                    Type::List { element } => element.as_ref().clone(),
                    _ => {
                        return Err(TypeError::TypeMismatch {
                            expected: Type::List {
                                element: Box::new(Type::Unknown),
                            },
                            found: iterable_typed.ty.clone(),
                            span: span.clone(),
                        });
                    }
                };

                // Type check the body in a new scope with the loop variable bound
                let mut for_checker = TypeChecker {
                    environment: Environment::with_parent(self.environment.clone()),
                    errors: Vec::new(),
                    module_loader: ModuleLoader::new(),
                };
                for_checker
                    .module_loader
                    .set_current_directory(self.module_loader.get_current_directory());
                for_checker.environment.bind(variable.clone(), element_type);
                let _ = for_checker.check_expression(body)?;

                // For loops return Unit
                Ok(TypedExpression::new(Type::Unit, span.clone()))
            }
            Expression::Range { start, end, span } => {
                let start_typed = self.check_expression(start)?;
                let end_typed = self.check_expression(end)?;

                // Both start and end must be integers
                if start_typed.ty != Type::Int {
                    return Err(TypeError::TypeMismatch {
                        expected: Type::Int,
                        found: start_typed.ty,
                        span: span.clone(),
                    });
                }
                if end_typed.ty != Type::Int {
                    return Err(TypeError::TypeMismatch {
                        expected: Type::Int,
                        found: end_typed.ty,
                        span: span.clone(),
                    });
                }

                // Range returns a list of integers
                Ok(TypedExpression::new(
                    Type::List {
                        element: Box::new(Type::Int),
                    },
                    span.clone(),
                ))
            }
            Expression::Concat { left, right, span } => {
                let left_typed = self.check_expression(left)?;
                let right_typed = self.check_expression(right)?;

                // Both operands must be strings
                if left_typed.ty != Type::String {
                    return Err(TypeError::TypeMismatch {
                        expected: Type::String,
                        found: left_typed.ty,
                        span: span.clone(),
                    });
                }
                if right_typed.ty != Type::String {
                    return Err(TypeError::TypeMismatch {
                        expected: Type::String,
                        found: right_typed.ty,
                        span: span.clone(),
                    });
                }

                Ok(TypedExpression::new(Type::String, span.clone()))
            }
            Expression::CharAt {
                string,
                index,
                span,
            } => {
                let string_typed = self.check_expression(string)?;
                let index_typed = self.check_expression(index)?;

                // String must be String type
                if string_typed.ty != Type::String {
                    return Err(TypeError::TypeMismatch {
                        expected: Type::String,
                        found: string_typed.ty,
                        span: span.clone(),
                    });
                }
                // Index must be Int
                if index_typed.ty != Type::Int {
                    return Err(TypeError::TypeMismatch {
                        expected: Type::Int,
                        found: index_typed.ty,
                        span: span.clone(),
                    });
                }

                // Returns a single character as String
                Ok(TypedExpression::new(Type::String, span.clone()))
            }
            Expression::Length { string, span } => {
                let string_typed = self.check_expression(string)?;

                // String must be String type
                if string_typed.ty != Type::String {
                    return Err(TypeError::TypeMismatch {
                        expected: Type::String,
                        found: string_typed.ty,
                        span: span.clone(),
                    });
                }

                // Returns length as integer
                Ok(TypedExpression::new(Type::Int, span.clone()))
            }
            Expression::ToString { expression, span } => {
                let expression_typed = self.check_expression(expression)?;

                // toString can convert any type to string
                // We accept Int, Bool, String, List, Pair types
                match &expression_typed.ty {
                    Type::Int
                    | Type::Bool
                    | Type::String
                    | Type::List { .. }
                    | Type::Pair { .. }
                    | Type::Unit => Ok(TypedExpression::new(Type::String, span.clone())),
                    _ => {
                        // For now, we'll allow other types but may need to extend this later
                        Ok(TypedExpression::new(Type::String, span.clone()))
                    }
                }
            }
            Expression::TypeOf { expression, span } => {
                let _expression_typed = self.check_expression(expression)?;

                // type() always returns a String representing the type
                Ok(TypedExpression::new(Type::String, span.clone()))
            }
            Expression::If {
                condition,
                then_branch,
                else_branch,
                span,
            } => {
                let condition_typed = self.check_expression(condition)?;
                if condition_typed.ty != Type::Bool {
                    return Err(TypeError::TypeMismatch {
                        expected: Type::Bool,
                        found: condition_typed.ty,
                        span: condition.span().clone(),
                    });
                }

                let then_typed = self.check_expression(then_branch)?;

                if let Some(else_branch) = else_branch {
                    let else_typed = self.check_expression(else_branch)?;

                    // If both branches have the same type, use that type
                    if then_typed.ty.is_assignable_to(&else_typed.ty)
                        && else_typed.ty.is_assignable_to(&then_typed.ty)
                    {
                        Ok(TypedExpression::new(then_typed.ty, span.clone()))
                    } else {
                        // Different types - create a sum type
                        let sum_type = Type::Sum {
                            left: Box::new(then_typed.ty),
                            right: Box::new(else_typed.ty),
                        };
                        Ok(TypedExpression::new(sum_type, span.clone()))
                    }
                } else {
                    // If there is no else branch, the expression must return Unit
                    // and the then branch must also be Unit
                    if then_typed.ty != Type::Unit {
                        return Err(TypeError::TypeMismatch {
                            expected: Type::Unit,
                            found: then_typed.ty,
                            span: then_branch.span().clone(),
                        });
                    }
                    Ok(TypedExpression::new(Type::Unit, span.clone()))
                }
            }
        }
    }

    /// Convert a TypeExpression to a Type
    fn convert_type_expression(&self, type_expr: &TypeExpression) -> TypeResult<Type> {
        match type_expr {
            TypeExpression::Int { .. } => Ok(Type::Int),
            TypeExpression::Bool { .. } => Ok(Type::Bool),
            TypeExpression::String { .. } => Ok(Type::String),
            TypeExpression::List { element, .. } => {
                let element_type = self.convert_type_expression(element)?;
                Ok(Type::List {
                    element: Box::new(element_type),
                })
            }
            TypeExpression::Function { param, result, .. } => {
                let param_type = self.convert_type_expression(param)?;
                let result_type = self.convert_type_expression(result)?;
                Ok(Type::Function {
                    param: Box::new(param_type),
                    result: Box::new(result_type),
                })
            }
            TypeExpression::Pair { first, second, .. } => {
                let first_type = self.convert_type_expression(first)?;
                let second_type = self.convert_type_expression(second)?;
                Ok(Type::Pair {
                    first: Box::new(first_type),
                    second: Box::new(second_type),
                })
            }
            TypeExpression::Sum { left, right, .. } => {
                let left_type = self.convert_type_expression(left)?;
                let right_type = self.convert_type_expression(right)?;
                Ok(Type::Sum {
                    left: Box::new(left_type),
                    right: Box::new(right_type),
                })
            }
            TypeExpression::Recursive { inner, .. } => {
                let inner_type = self.convert_type_expression(inner)?;
                Ok(Type::Recursive {
                    inner: Box::new(inner_type),
                })
            }
            TypeExpression::Named { name, span } => {
                // For now, we don't support named types - this could be extended later
                Err(TypeError::UndefinedVariable {
                    name: name.clone(),
                    span: span.clone(),
                })
            }
        }
    }

    /// Get all accumulated type errors
    pub fn get_errors(&self) -> &[TypeError] {
        &self.errors
    }

    /// Clear all accumulated errors
    pub fn clear_errors(&mut self) {
        self.errors.clear()
    }

    /// Get the current type environment
    pub fn get_environment(&self) -> &Environment {
        &self.environment
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}
