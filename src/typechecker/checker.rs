use crate::ast::{Expression, Program, Spanned, Statement, TypeExpression};
use crate::lexer::tokens::Span;
use crate::typechecker::{
    BinaryOp, Environment, Type, TypedExpression, TypedProgram, TypedStatement,
};

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
        }
    }
}

impl std::error::Error for TypeError {}

pub type TypeResult<T> = Result<T, TypeError>;

/// Type checker for the Corrosion language
pub struct TypeChecker {
    environment: Environment,
    errors: Vec<TypeError>,
}

impl TypeChecker {
    /// Create a new type checker
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
            errors: Vec::new(),
        }
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
            Expression::Identifier { name, span } => match self.environment.lookup(name) {
                Some(ty) => Ok(TypedExpression::new(ty.clone(), span.clone())),
                None => Err(TypeError::UndefinedVariable {
                    name: name.clone(),
                    span: span.clone(),
                }),
            },
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
            Expression::Function { param, body, span } => {
                // Use type inference to determine parameter type
                let param_type = self.infer_parameter_type(param, body)?;

                // Create a new type checker with a child environment for the function scope
                let mut function_checker = TypeChecker {
                    environment: Environment::with_parent(self.environment.clone()),
                    errors: Vec::new(),
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
                        // Check if argument type is compatible with parameter type
                        if self.types_compatible(&argument_typed.ty, param) {
                            Ok(TypedExpression::new((**result).clone(), span.clone()))
                        } else {
                            Err(TypeError::TypeMismatch {
                                expected: (**param).clone(),
                                found: argument_typed.ty.clone(),
                                span: span.clone(),
                            })
                        }
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
            Expression::LeftInject { .. } => {
                Ok(TypedExpression::new(Type::Error, expression.span().clone()))
            }
            Expression::RightInject { .. } => {
                Ok(TypedExpression::new(Type::Error, expression.span().clone()))
            }
            Expression::Fix { .. } => {
                Ok(TypedExpression::new(Type::Error, expression.span().clone()))
            }
            Expression::Block { .. } => {
                Ok(TypedExpression::new(Type::Error, expression.span().clone()))
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
        }
    }

    /// Convert a TypeExpression to a Type
    fn convert_type_expression(&self, type_expr: &TypeExpression) -> TypeResult<Type> {
        match type_expr {
            TypeExpression::Int { .. } => Ok(Type::Int),
            TypeExpression::Bool { .. } => Ok(Type::Bool),
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
            // For non-function/non-list types, return the inferred type as-is
            _ => Ok(inferred.clone()),
        }
    }

    /// Check if two types are compatible
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
        // Simple type inference based on usage patterns
        match self.analyze_parameter_usage(param, body) {
            Some(inferred_type) => Ok(inferred_type),
            None => {
                // If we can't infer the type from usage, default to Unknown
                // In a more sophisticated system, we might require type annotations
                Ok(Type::Unknown)
            }
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
