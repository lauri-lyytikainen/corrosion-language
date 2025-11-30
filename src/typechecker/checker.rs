use crate::ast::{Expression, Program, Spanned, Statement, TypeExpression};
use crate::lexer::tokens::Span;
use crate::typechecker::{
    BinaryOp, Environment, Type, TypedExpression, TypedProgram, TypedStatement,
};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Type checking errors
#[derive(Debug, Clone)]
pub enum TypeError {
    UndefinedVariable {
        name: String,
        span: Span,
    },
    TypeMismatch {
        expected: Type,
        found: Type,
        span: Span,
    },
    InvalidBinaryOperation {
        left: Type,
        op: BinaryOp,
        right: Type,
        span: Span,
    },
    RedefinedVariable {
        name: String,
        span: Span,
    },
    ImportError {
        path: String,
        message: String,
        span: Span,
    },
}

impl std::fmt::Display for TypeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TypeError::UndefinedVariable { name, span } => {
                write!(
                    f,
                    "Undefined variable '{}' at line {}, column {}",
                    name, span.line, span.column
                )
            }
            TypeError::TypeMismatch {
                expected,
                found,
                span,
            } => {
                write!(
                    f,
                    "Type mismatch at line {}, column {}: expected '{}', found '{}'",
                    span.line, span.column, expected, found
                )
            }
            TypeError::InvalidBinaryOperation {
                left,
                op,
                right,
                span,
            } => {
                write!(
                    f,
                    "Invalid binary operation at line {}, column {}: '{}' {:?} '{}'",
                    span.line, span.column, left, op, right
                )
            }
            TypeError::RedefinedVariable { name, span } => {
                write!(
                    f,
                    "Variable '{}' redefined at line {}, column {}",
                    name, span.line, span.column
                )
            }
            TypeError::ImportError {
                path,
                message,
                span,
            } => {
                write!(
                    f,
                    "Import error at line {}, column {}: {} (path: {})",
                    span.line, span.column, message, path
                )
            }
        }
    }
}

impl std::error::Error for TypeError {}

pub type TypeResult<T> = Result<T, TypeError>;

/// Type checker for the Corrosion language
pub struct TypeChecker {
    environment: Environment,
    errors: Vec<TypeError>,
    /// Current directory for resolving imports
    current_directory: PathBuf,
    /// Cache of loaded modules
    modules: HashMap<String, HashMap<String, Type>>,
}

impl TypeChecker {
    /// Create a new type checker
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
            errors: Vec::new(),
            current_directory: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            modules: HashMap::new(),
        }
    }

    /// Set the current directory for import resolution
    pub fn set_current_directory<P: AsRef<Path>>(&mut self, path: P) {
        self.current_directory = path.as_ref().to_path_buf();
    }

    /// Load and type-check a module from file
    fn load_and_check_module(
        &mut self,
        path: &str,
        module_name: &str,
        span: &Span,
    ) -> TypeResult<HashMap<String, Type>> {
        // Resolve the import path relative to current directory
        let import_path = self.current_directory.join(path);

        // Read the file content
        let content = fs::read_to_string(&import_path).map_err(|_| TypeError::ImportError {
            message: format!("Failed to read module file: {}", import_path.display()),
            path: path.to_string(),
            span: span.clone(),
        })?;

        // Parse the file content
        let mut lexer = crate::lexer::tokenizer::Tokenizer::new("");
        let tokens = lexer
            .tokenize(&content)
            .map_err(|e| TypeError::ImportError {
                message: format!("Failed to tokenize module {}: {}", module_name, e),
                path: path.to_string(),
                span: span.clone(),
            })?;

        let mut parser = crate::ast::parser::Parser::new(tokens);
        let program = parser.parse().map_err(|e| TypeError::ImportError {
            message: format!("Failed to parse module {}: {}", module_name, e),
            path: path.to_string(),
            span: span.clone(),
        })?;

        // Create a new type checker for the module
        let mut module_checker = TypeChecker::new();

        // Set the module's current directory to the imported file's directory
        if let Some(parent) = import_path.parent() {
            module_checker.set_current_directory(parent);
        }

        // Type-check the module
        let _typed_program =
            module_checker
                .check_program(&program)
                .map_err(|e| TypeError::ImportError {
                    message: format!("Failed to type-check module {}: {}", module_name, e),
                    path: path.to_string(),
                    span: span.clone(),
                })?;

        // Extract all top-level bindings as exports
        Ok(module_checker.environment.get_all_bindings_types())
    }

    /// Type check a program and return the typed AST
    pub fn check_program(&mut self, program: &Program) -> TypeResult<TypedProgram> {
        let mut typed_statements = Vec::new();

        for statement in &program.statements {
            match self.check_statement(statement) {
                Ok(typed_stmt) => typed_statements.push(typed_stmt),
                Err(err) => {
                    self.errors.push(err.clone());
                    // Continue checking other statements even after errors
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

                // If there's a type annotation, check it matches the inferred type
                let final_type = if let Some(annotation) = type_annotation {
                    let annotated_type = self.convert_type_expression(annotation)?;

                    // Special handling for function types with Unknown parameters/results
                    let refined_type =
                        self.refine_type_with_annotation(&inferred_type, &annotated_type)?;

                    if !self.types_compatible(&annotated_type, &refined_type) {
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

                // Create a new scope for the function body
                self.environment.enter_scope();

                // If there's a return type annotation, use it; otherwise infer
                let expected_return_type = return_type
                    .as_ref()
                    .map(|rt| self.convert_type_expression(rt))
                    .transpose()?;

                // Use explicit parameter type if provided, otherwise Unknown for inference
                let param_type = if let Some(param_type_expr) = param_type {
                    self.convert_type_expression(param_type_expr)?
                } else {
                    Type::Unknown
                };
                self.environment.bind(param.clone(), param_type.clone());

                // Type check the function body
                let typed_body = self.check_expression(body)?;
                let actual_return_type = typed_body.ty.clone();

                // Check return type matches annotation if provided
                let final_return_type = if let Some(expected) = expected_return_type {
                    if !self.types_compatible(&expected, &actual_return_type) {
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

                // Create function type
                let function_type = Type::function(param_type.clone(), final_return_type.clone());

                // Bind the function name to its type
                self.environment.bind(name.clone(), function_type);

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
                let module_exports = self.load_and_check_module(path, import_name, span)?;

                // Store the module's exports for later lookup
                self.modules.insert(import_name.clone(), module_exports);

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
                if let Some(module_exports) = self.modules.get(module) {
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
                    self.infer_parameter_type(param, body)?
                };

                // Create a new type checker with a child environment for the function scope
                let mut function_checker = TypeChecker {
                    environment: Environment::with_parent(self.environment.clone()),
                    errors: Vec::new(),
                    current_directory: self.current_directory.clone(),
                    modules: self.modules.clone(),
                };

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
                            self.refine_type_with_context(param, &argument_typed.ty);
                        let refined_result = self.refine_type_with_context(result, &Type::Unknown);

                        // Check if argument type is compatible with parameter type
                        // Be more permissive with Unknown types in recursive contexts
                        let is_compatible = self
                            .types_compatible(&argument_typed.ty, &refined_param)
                            || (argument_typed.ty == Type::Unknown
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
                    Type::Unknown => {
                        // If function type is unknown, try to infer it from context
                        // Assume it's a function and return Unknown result
                        Ok(TypedExpression::new(Type::Unknown, span.clone()))
                    }
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
                            current_directory: self.current_directory.clone(),
                            modules: self.modules.clone(),
                        };
                        // Handle Unknown type in sum (from inference)
                        let left_type = if **left == Type::Unknown {
                            // If we don't know the type, we can't check the body properly unless we infer it.
                            // For now, let's assume Unknown propagates or we treat it as Unknown.
                            // But better to bind it as Unknown.
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
                            current_directory: self.current_directory.clone(),
                            modules: self.modules.clone(),
                        };
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
                        if self.types_compatible(&typed_left_body.ty, &typed_right_body.ty) {
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
                                if self.types_compatible(inner_param, inner_result)
                                    && self.types_compatible(outer_param, outer_result)
                                    && self.types_compatible(inner_param, outer_param)
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
                    current_directory: self.current_directory.clone(),
                    modules: self.modules.clone(),
                };

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
                        if self.types_compatible(&head_typed.ty, element) {
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
                    current_directory: self.current_directory.clone(),
                    modules: self.modules.clone(),
                };
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

    /// Refine a type based on type annotation information
    fn refine_type_with_annotation(&self, inferred: &Type, annotated: &Type) -> TypeResult<Type> {
        match (inferred, annotated) {
            // If the inferred type has Unknown components, use the annotated type
            (
                Type::Function {
                    param: inf_param,
                    result: inf_result,
                },
                Type::Function {
                    param: ann_param,
                    result: ann_result,
                },
            ) => {
                // If inferred parameter is Unknown, use annotated parameter type
                let refined_param = if matches!(**inf_param, Type::Unknown) {
                    ann_param.clone()
                } else {
                    inf_param.clone()
                };

                // If inferred result is Unknown, use annotated result type
                let refined_result = if matches!(**inf_result, Type::Unknown) {
                    ann_result.clone()
                } else {
                    inf_result.clone()
                };

                Ok(Type::Function {
                    param: refined_param,
                    result: refined_result,
                })
            }
            // Handle list types with Unknown elements
            (Type::List { element: inf_elem }, Type::List { element: ann_elem }) => {
                // If inferred element is Unknown, use annotated element type
                let refined_element = if matches!(**inf_elem, Type::Unknown) {
                    ann_elem.clone()
                } else {
                    inf_elem.clone()
                };

                Ok(Type::List {
                    element: refined_element,
                })
            }
            // Handle sum types with Unknown components
            (
                Type::Sum {
                    left: inf_left,
                    right: inf_right,
                },
                Type::Sum {
                    left: ann_left,
                    right: ann_right,
                },
            ) => {
                let refined_left = if matches!(**inf_left, Type::Unknown) {
                    ann_left.clone()
                } else {
                    inf_left.clone()
                };

                let refined_right = if matches!(**inf_right, Type::Unknown) {
                    ann_right.clone()
                } else {
                    inf_right.clone()
                };

                Ok(Type::Sum {
                    left: refined_left,
                    right: refined_right,
                })
            }
            // For non-function/non-list types, return the inferred type as-is
            _ => Ok(inferred.clone()),
        }
    }

    /// Refine a type by replacing Unknown with more specific types based on context
    fn refine_type_with_context(&self, original: &Type, context: &Type) -> Type {
        match (original, context) {
            (Type::Unknown, concrete_type) if !matches!(concrete_type, Type::Unknown) => {
                concrete_type.clone()
            }
            // Handle List types with Unknown elements
            (
                Type::List { element },
                Type::List {
                    element: context_element,
                },
            ) => Type::List {
                element: Box::new(self.refine_type_with_context(element, context_element)),
            },
            // If original is Unknown but context suggests List, use List with Unknown elements
            (Type::Unknown, Type::List { .. }) => context.clone(),
            // If context has Unknown but original is more specific, prefer original
            (original_type, Type::Unknown) => original_type.clone(),
            // Handle Sum types with Unknown components
            (
                Type::Sum { left, right },
                Type::Sum {
                    left: context_left,
                    right: context_right,
                },
            ) => Type::Sum {
                left: Box::new(self.refine_type_with_context(left, context_left)),
                right: Box::new(self.refine_type_with_context(right, context_right)),
            },
            // If original is Unknown but context suggests Sum, use Sum
            (Type::Unknown, Type::Sum { .. }) => context.clone(),
            (Type::Function { param, result }, _) => Type::Function {
                param: Box::new(self.refine_type_with_context(param, &Type::Unknown)),
                result: Box::new(self.refine_type_with_context(result, &Type::Unknown)),
            },
            _ => original.clone(),
        }
    }

    fn types_compatible(&self, t1: &Type, t2: &Type) -> bool {
        match (t1, t2) {
            // Unknown types are compatible with anything
            (Type::Unknown, _) | (_, Type::Unknown) => true,

            // Function types are compatible if their parameters and results are compatible
            (
                Type::Function {
                    param: p1,
                    result: r1,
                },
                Type::Function {
                    param: p2,
                    result: r2,
                },
            ) => self.types_compatible(p1, p2) && self.types_compatible(r1, r2),

            // List types are compatible if their element types are compatible
            (Type::List { element: e1 }, Type::List { element: e2 }) => {
                self.types_compatible(e1, e2)
            }

            // Pair types are compatible if their first and second types are compatible
            (
                Type::Pair {
                    first: f1,
                    second: s1,
                },
                Type::Pair {
                    first: f2,
                    second: s2,
                },
            ) => self.types_compatible(f1, f2) && self.types_compatible(s1, s2),

            // Sum types are compatible if their left and right types are compatible
            (
                Type::Sum {
                    left: l1,
                    right: r1,
                },
                Type::Sum {
                    left: l2,
                    right: r2,
                },
            ) => self.types_compatible(l1, l2) && self.types_compatible(r1, r2),

            // Otherwise, use structural equality
            _ => t1 == t2,
        }
    }

    /// Get all accumulated type errors
    pub fn get_errors(&self) -> &[TypeError] {
        &self.errors
    }

    /// Clear all accumulated errors
    pub fn clear_errors(&mut self) {
        self.errors.clear();
    }

    /// Get the current type environment
    pub fn get_environment(&self) -> &Environment {
        &self.environment
    }

    /// Infer the parameter type based on how it's used in the function body
    fn infer_parameter_type(&mut self, param: &str, body: &Expression) -> TypeResult<Type> {
        // Enhanced type inference for function parameters

        // First, try to analyze direct usage patterns
        if let Some(inferred_type) = self.analyze_parameter_usage(param, body) {
            return Ok(inferred_type);
        }

        // Check if parameter is used in function call contexts
        if let Some(func_type) = self.analyze_function_usage(param, body) {
            return Ok(func_type);
        }

        // Check if parameter appears to be a function based on call patterns
        if self.parameter_used_as_function(param, body) {
            // Create a generic function type - this will be refined later
            return Ok(Type::Function {
                param: Box::new(Type::Unknown),
                result: Box::new(Type::Unknown),
            });
        }

        // If we can't infer the type from usage, start with Unknown
        // but try to refine it through constraint solving
        Ok(Type::Unknown)
    }

    /// Check if a parameter is used in function call contexts
    fn analyze_function_usage(&self, param: &str, expr: &Expression) -> Option<Type> {
        match expr {
            Expression::FunctionCall {
                function, argument, ..
            } => {
                if let Expression::Identifier { name, .. } = function.as_ref() {
                    if name == param {
                        // Parameter is called as a function
                        // Try to infer the argument type
                        let arg_type = self.get_expression_type_hint(argument);
                        return Some(Type::Function {
                            param: Box::new(arg_type),
                            result: Box::new(Type::Unknown),
                        });
                    }
                }
                // Recursively check sub-expressions
                if let Some(func_type) = self.analyze_function_usage(param, function) {
                    return Some(func_type);
                }
                if let Some(func_type) = self.analyze_function_usage(param, argument) {
                    return Some(func_type);
                }
            }
            Expression::Function { body, .. } => {
                return self.analyze_function_usage(param, body);
            }
            Expression::Block {
                statements,
                expression,
                ..
            } => {
                for stmt in statements {
                    if let Statement::Expression {
                        expression: expr, ..
                    } = stmt
                    {
                        if let Some(func_type) = self.analyze_function_usage(param, expr) {
                            return Some(func_type);
                        }
                    }
                }
                if let Some(expr) = expression {
                    return self.analyze_function_usage(param, expr);
                }
            }
            _ => {}
        }
        None
    }

    /// Check if a parameter is used as a function (called with arguments)
    fn parameter_used_as_function(&self, param: &str, expr: &Expression) -> bool {
        match expr {
            Expression::FunctionCall { function, .. } => {
                if let Expression::Identifier { name, .. } = function.as_ref() {
                    if name == param {
                        return true;
                    }
                }
                false
            }
            Expression::Function { body, .. } => self.parameter_used_as_function(param, body),
            Expression::Block {
                statements,
                expression,
                ..
            } => {
                for stmt in statements {
                    if let Statement::Expression {
                        expression: expr, ..
                    } = stmt
                    {
                        if self.parameter_used_as_function(param, expr) {
                            return true;
                        }
                    }
                }
                if let Some(expr) = expression {
                    return self.parameter_used_as_function(param, expr);
                }
                false
            }
            _ => false,
        }
    }

    /// Get a type hint for an expression without full type checking
    fn get_expression_type_hint(&self, expr: &Expression) -> Type {
        match expr {
            Expression::Number { .. } => Type::Int,
            Expression::Boolean { .. } => Type::Bool,
            Expression::Identifier { name, .. } => {
                // Try to look up in current environment
                if let Some(ty) = self.environment.lookup(name) {
                    ty.clone()
                } else {
                    Type::Unknown
                }
            }
            Expression::Function { .. } => Type::Function {
                param: Box::new(Type::Unknown),
                result: Box::new(Type::Unknown),
            },
            _ => Type::Unknown,
        }
    }

    /// Analyze how a parameter is used in an expression to infer its type
    fn analyze_parameter_usage(&self, param: &str, expr: &Expression) -> Option<Type> {
        match expr {
            Expression::Identifier { name, .. } if name == param => {
                // Parameter is used directly - we need more context to infer type
                None
            }
            Expression::BinaryOp {
                left,
                operator,
                right,
                ..
            } => {
                // First check if parameter is used in pair operations in sub-expressions
                let left_pair_usage = self.analyze_parameter_usage(param, left);
                let right_pair_usage = self.analyze_parameter_usage(param, right);

                // If we find pair usage, prioritize it
                if let Some(ref pair_type) = left_pair_usage {
                    if matches!(pair_type, Type::Pair { .. }) {
                        return left_pair_usage;
                    }
                }
                if let Some(ref pair_type) = right_pair_usage {
                    if matches!(pair_type, Type::Pair { .. }) {
                        return right_pair_usage;
                    }
                }

                // Check if parameter is used in arithmetic operations
                let left_uses_param = self.expression_uses_parameter(param, left);
                let right_uses_param = self.expression_uses_parameter(param, right);

                // Also check for list operations in arithmetic context
                let left_list_usage = self.analyze_parameter_usage(param, left);
                let right_list_usage = self.analyze_parameter_usage(param, right);

                // If we find list usage in arithmetic context, return List<Int>
                if matches!(
                    operator,
                    crate::ast::BinaryOperator::Add
                        | crate::ast::BinaryOperator::Subtract
                        | crate::ast::BinaryOperator::Multiply
                        | crate::ast::BinaryOperator::Divide
                ) {
                    if let Some(Type::List { .. }) = left_list_usage {
                        return Some(Type::List {
                            element: Box::new(Type::Int),
                        });
                    }
                    if let Some(Type::List { .. }) = right_list_usage {
                        return Some(Type::List {
                            element: Box::new(Type::Int),
                        });
                    }
                }

                if left_uses_param || right_uses_param {
                    match operator {
                        crate::ast::BinaryOperator::Add
                        | crate::ast::BinaryOperator::Subtract
                        | crate::ast::BinaryOperator::Multiply
                        | crate::ast::BinaryOperator::Divide => Some(Type::Int),
                        crate::ast::BinaryOperator::LogicalAnd
                        | crate::ast::BinaryOperator::LogicalOr => Some(Type::Bool),
                        _ => None,
                    }
                } else {
                    // Return any other inferred type
                    left_pair_usage.or(right_pair_usage)
                }
            }
            Expression::FunctionCall {
                function, argument, ..
            } => {
                // If parameter is used as a function, infer it's a function type
                if let Expression::Identifier { name, .. } = function.as_ref() {
                    if name == param {
                        // Parameter is being called as a function
                        // We'd need more sophisticated analysis to determine exact function type
                        return Some(Type::Function {
                            param: Box::new(Type::Unknown),
                            result: Box::new(Type::Unknown),
                        });
                    }
                }

                // Special case: if argument is tail(param), infer function takes lists
                if let Expression::TailProjection { list, .. } = argument.as_ref() {
                    if self.expression_uses_parameter(param, list) {
                        // The function being called should accept a list type
                        // and return some type (Unknown for now)
                        return Some(Type::List {
                            element: Box::new(Type::Unknown),
                        });
                    }
                }

                // Recursively check sub-expressions
                self.analyze_parameter_usage(param, function)
                    .or_else(|| self.analyze_parameter_usage(param, argument))
            }
            Expression::FirstProjection { pair, .. } => {
                // If parameter is used in fst(), infer it's a pair type
                if self.expression_uses_parameter(param, pair) {
                    Some(Type::Pair {
                        first: Box::new(Type::Unknown),
                        second: Box::new(Type::Unknown),
                    })
                } else {
                    self.analyze_parameter_usage(param, pair)
                }
            }
            Expression::SecondProjection { pair, .. } => {
                // If parameter is used in snd(), infer it's a pair type
                if self.expression_uses_parameter(param, pair) {
                    Some(Type::Pair {
                        first: Box::new(Type::Unknown),
                        second: Box::new(Type::Unknown),
                    })
                } else {
                    self.analyze_parameter_usage(param, pair)
                }
            }
            Expression::HeadProjection { list, .. } => {
                // If parameter is used in head(), infer it's a list type
                if self.expression_uses_parameter(param, list) {
                    // Use Unknown for element type to allow flexible inference
                    Some(Type::List {
                        element: Box::new(Type::Unknown),
                    })
                } else {
                    self.analyze_parameter_usage(param, list)
                }
            }
            Expression::TailProjection { list, .. } => {
                // If parameter is used in tail(), infer it's a list type
                if self.expression_uses_parameter(param, list) {
                    // Use Unknown for element type to allow flexible inference
                    Some(Type::List {
                        element: Box::new(Type::Unknown),
                    })
                } else {
                    self.analyze_parameter_usage(param, list)
                }
            }
            Expression::Case {
                expression,
                left_body,
                right_body,
                ..
            } => {
                // If parameter is used in case expression, it should be a sum type
                if self.expression_uses_parameter(param, expression) {
                    // Parameter is being case-matched, so it must be a sum type
                    // We need to infer the types from the branches
                    return Some(Type::Sum {
                        left: Box::new(Type::Unknown),
                        right: Box::new(Type::Unknown),
                    });
                }

                // Check if parameter is used in the branches
                let left_usage = self.analyze_parameter_usage(param, left_body);
                let right_usage = self.analyze_parameter_usage(param, right_body);

                left_usage.or(right_usage)
            }
            Expression::Block {
                statements,
                expression,
                ..
            } => {
                // Check statements first
                for stmt in statements {
                    if let Some(ty) = self.analyze_parameter_usage_in_statement(param, stmt) {
                        return Some(ty);
                    }
                }

                // Check final expression
                if let Some(expr) = expression {
                    self.analyze_parameter_usage(param, expr)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Check if an expression uses the given parameter
    fn expression_uses_parameter(&self, param: &str, expr: &Expression) -> bool {
        match expr {
            Expression::Identifier { name, .. } => name == param,
            Expression::BinaryOp { left, right, .. } => {
                self.expression_uses_parameter(param, left)
                    || self.expression_uses_parameter(param, right)
            }
            Expression::FunctionCall {
                function, argument, ..
            } => {
                self.expression_uses_parameter(param, function)
                    || self.expression_uses_parameter(param, argument)
            }
            Expression::FirstProjection { pair, .. } => self.expression_uses_parameter(param, pair),
            Expression::SecondProjection { pair, .. } => {
                self.expression_uses_parameter(param, pair)
            }
            Expression::HeadProjection { list, .. } => self.expression_uses_parameter(param, list),
            Expression::TailProjection { list, .. } => self.expression_uses_parameter(param, list),
            Expression::Case {
                expression,
                left_body,
                right_body,
                ..
            } => {
                self.expression_uses_parameter(param, expression)
                    || self.expression_uses_parameter(param, left_body)
                    || self.expression_uses_parameter(param, right_body)
            }
            Expression::Block {
                statements,
                expression,
                ..
            } => {
                statements
                    .iter()
                    .any(|stmt| self.statement_uses_parameter(param, stmt))
                    || expression
                        .as_ref()
                        .map_or(false, |expr| self.expression_uses_parameter(param, expr))
            }
            _ => false,
        }
    }

    /// Check if a statement uses the given parameter
    fn statement_uses_parameter(&self, param: &str, stmt: &Statement) -> bool {
        match stmt {
            Statement::VariableDeclaration { value, .. } => {
                self.expression_uses_parameter(param, value)
            }
            Statement::FunctionDeclaration { body, .. } => {
                self.expression_uses_parameter(param, body)
            }

            Statement::Import { .. } => false,
            Statement::Expression { expression, .. } => {
                self.expression_uses_parameter(param, expression)
            }
        }
    }

    /// Analyze parameter usage in statements
    fn analyze_parameter_usage_in_statement(&self, param: &str, stmt: &Statement) -> Option<Type> {
        match stmt {
            Statement::VariableDeclaration { value, .. } => {
                self.analyze_parameter_usage(param, value)
            }
            Statement::FunctionDeclaration { body, .. } => {
                self.analyze_parameter_usage(param, body)
            }

            Statement::Import { .. } => None,
            Statement::Expression { expression, .. } => {
                self.analyze_parameter_usage(param, expression)
            }
        }
    }
}

impl Default for TypeChecker {
    fn default() -> Self {
        Self::new()
    }
}
