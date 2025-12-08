use crate::ast::{Expression, Statement};
use crate::typechecker::{Environment, Type, TypeResult};

pub struct TypeInference {
    environment: Environment,
}

impl TypeInference {
    pub fn new(environment: Environment) -> Self {
        Self { environment }
    }

    /// Infer the parameter type based on how it's used in the function body
    pub fn infer_parameter_type(&self, param: &str, body: &Expression) -> TypeResult<Type> {
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

        Ok(Type::Unknown)
    }

    /// Check if a parameter is used in function call contexts
    pub fn analyze_function_usage(&self, param: &str, expr: &Expression) -> Option<Type> {
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
    pub fn parameter_used_as_function(&self, param: &str, expr: &Expression) -> bool {
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
    pub fn get_expression_type_hint(&self, expr: &Expression) -> Type {
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
    pub fn analyze_parameter_usage(&self, param: &str, expr: &Expression) -> Option<Type> {
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
                        let arg_type = self.get_expression_type_hint(argument);
                        return Some(Type::Function {
                            param: Box::new(arg_type),
                            result: Box::new(Type::Unknown),
                        });
                    }
                }

                // Special case: if argument is tail(param), infer function takes lists
                if let Expression::TailProjection { list, .. } = argument.as_ref() {
                    if self.expression_uses_parameter(param, list) {
                        return Some(Type::Function {
                            param: Box::new(Type::List {
                                element: Box::new(Type::Unknown),
                            }),
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
    pub fn expression_uses_parameter(&self, param: &str, expr: &Expression) -> bool {
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
    pub fn statement_uses_parameter(&self, param: &str, stmt: &Statement) -> bool {
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
    pub fn analyze_parameter_usage_in_statement(
        &self,
        param: &str,
        stmt: &Statement,
    ) -> Option<Type> {
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
