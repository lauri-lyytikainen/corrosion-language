use super::{Environment, InterpreterError, InterpreterResult, Value};
use crate::ast::nodes::{BinaryOperator, Expression, Program, Spanned, Statement};
use crate::lexer::tokens::Span;

/// Interpreter for the Corrosion language
pub struct Interpreter {
    /// Current environment for variable bindings
    environment: Environment,
}

impl Interpreter {
    /// Create a new interpreter
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
        }
    }

    /// Create a new interpreter with a given environment
    pub fn with_environment(environment: Environment) -> Self {
        Self { environment }
    }

    /// Interpret a program and return the result
    pub fn interpret_program(&mut self, program: &Program) -> InterpreterResult<Value> {
        for statement in &program.statements {
            self.interpret_statement(statement)?;
        }

        Ok(Value::Unit)
    }

    /// Interpret a single statement
    pub fn interpret_statement(&mut self, statement: &Statement) -> InterpreterResult<Value> {
        match statement {
            Statement::VariableDeclaration { name, value, .. } => {
                let val = self.interpret_expression(value)?;
                self.environment.bind(name.clone(), val);
                Ok(Value::Unit)
            }
            Statement::FunctionDeclaration {
                name, param, body, ..
            } => {
                // Create a function value and bind it to the name
                let function_val = Value::Function {
                    param: param.clone(),
                    body: Box::new(body.clone()),
                    env: self.environment.clone(),
                };
                self.environment.bind(name.clone(), function_val);
                Ok(Value::Unit)
            }
            Statement::ConstantDeclaration { name, value, .. } => {
                // Constants are treated the same as variables in the interpreter
                let val = self.interpret_expression(value)?;
                self.environment.bind(name.clone(), val);
                Ok(Value::Unit)
            }
            Statement::Import { path, alias, span: _ } => {
                // For now, we'll just store import information without actual file loading
                // TODO: Implement actual file loading and module system
                let import_name = alias.as_ref().unwrap_or(path);
                
                // Create a placeholder module value
                let module_val = Value::Module {
                    name: import_name.clone(),
                    exports: std::collections::HashMap::new(),
                };
                
                self.environment.bind(import_name.clone(), module_val);
                Ok(Value::Unit)
            }
            Statement::Expression { expression, .. } => self.interpret_expression(expression),
        }
    }

    /// Interpret an expression and return its value
    pub fn interpret_expression(&mut self, expr: &Expression) -> InterpreterResult<Value> {
        match expr {
            Expression::Number { value, .. } => Ok(Value::Int(*value)),

            Expression::Boolean { value, .. } => Ok(Value::Bool(*value)),

            Expression::String { value, .. } => Ok(Value::String(value.clone())),

            Expression::Identifier { name, span } => {
                self.environment.lookup(name).cloned().ok_or_else(|| {
                    InterpreterError::UndefinedVariable {
                        name: name.clone(),
                        span: span.clone(),
                    }
                })
            }

            Expression::QualifiedIdentifier { module, name, span } => {
                // Look up the module first
                if let Some(module_val) = self.environment.lookup(module) {
                    if let Value::Module { exports, .. } = module_val {
                        // Look up the name in the module's exports
                        exports.get(name).cloned().ok_or_else(|| {
                            InterpreterError::UndefinedVariable {
                                name: format!("{}.{}", module, name),
                                span: span.clone(),
                            }
                        })
                    } else {
                        Err(InterpreterError::TypeError {
                            expected: "Module".to_string(),
                            found: module_val.type_name().to_string(),
                            span: span.clone(),
                        })
                    }
                } else {
                    Err(InterpreterError::UndefinedVariable {
                        name: module.clone(),
                        span: span.clone(),
                    })
                }
            }

            Expression::List { elements, .. } => {
                let mut values = Vec::new();
                for element in elements {
                    values.push(self.interpret_expression(element)?);
                }
                Ok(Value::List(values))
            }

            Expression::Pair { first, second, .. } => {
                let first_val = Box::new(self.interpret_expression(first)?);
                let second_val = Box::new(self.interpret_expression(second)?);
                Ok(Value::Pair(first_val, second_val))
            }

            Expression::BinaryOp {
                left,
                operator,
                right,
                span,
            } => self.interpret_binary_op(left, operator, right, span),

            Expression::UnaryOp {
                operator,
                operand,
                span,
            } => {
                let operand_val = self.interpret_expression(operand)?;

                match operator {
                    crate::ast::nodes::UnaryOperator::LogicalNot => match operand_val {
                        Value::Bool(b) => Ok(Value::Bool(!b)),
                        _ => Err(InterpreterError::TypeError {
                            expected: "Bool".to_string(),
                            found: operand_val.type_name().to_string(),
                            span: span.clone(),
                        }),
                    },
                }
            }

            Expression::Function { param, body, .. } => {
                Ok(Value::Function {
                    param: param.clone(),
                    body: body.clone(),
                    env: self.environment.clone(), // Capture current environment
                })
            }

            Expression::FunctionCall {
                function,
                argument,
                span,
            } => self.interpret_function_call(function, argument, span),

            Expression::LeftInject { value, .. } => {
                let val = Box::new(self.interpret_expression(value)?);
                Ok(Value::LeftInject(val))
            }

            Expression::RightInject { value, .. } => {
                let val = Box::new(self.interpret_expression(value)?);
                Ok(Value::RightInject(val))
            }

            Expression::Fix { function, span } => {
                // Implement the Y-combinator style fixed point operator
                // fix(f) = f(fix(f)) - but we need to delay evaluation to avoid infinite recursion
                let func_value = self.interpret_expression(function)?;

                match func_value {
                    Value::Function { param, body, env } => {
                        // Create a FixedPoint value that represents the recursive function
                        Ok(Value::FixedPoint {
                            function: Box::new(Value::Function { param, body, env }),
                        })
                    }
                    _ => Err(InterpreterError::RuntimeError {
                        message: "Fix can only be applied to functions".to_string(),
                        span: Some(span.clone()),
                    }),
                }
            }

            Expression::Block {
                statements,
                expression,
                ..
            } => {
                // Execute block in a new scope
                self.environment.with_new_scope(|env| {
                    let mut interpreter = Interpreter::with_environment(env.clone());

                    // Execute all statements
                    for stmt in statements {
                        interpreter.interpret_statement(stmt)?;
                    }

                    // Return the final expression if present, otherwise Unit
                    if let Some(expr) = expression {
                        interpreter.interpret_expression(expr)
                    } else {
                        Ok(Value::Unit)
                    }
                })
            }

            Expression::FirstProjection { pair, span } => {
                let pair_val = self.interpret_expression(pair)?;
                match pair_val {
                    Value::Pair(first, _) => Ok(*first),
                    _ => Err(InterpreterError::TypeError {
                        expected: "Pair".to_string(),
                        found: pair_val.type_name().to_string(),
                        span: span.clone(),
                    }),
                }
            }

            Expression::SecondProjection { pair, span } => {
                let pair_val = self.interpret_expression(pair)?;
                match pair_val {
                    Value::Pair(_, second) => Ok(*second),
                    _ => Err(InterpreterError::TypeError {
                        expected: "Pair".to_string(),
                        found: pair_val.type_name().to_string(),
                        span: span.clone(),
                    }),
                }
            }

            Expression::Cons { head, tail, span } => {
                let head_val = self.interpret_expression(head)?;
                let tail_val = self.interpret_expression(tail)?;

                match tail_val {
                    Value::List(mut list) => {
                        // Insert the head at the beginning of the list
                        list.insert(0, head_val);
                        Ok(Value::List(list))
                    }
                    _ => Err(InterpreterError::TypeError {
                        expected: "List".to_string(),
                        found: tail_val.type_name().to_string(),
                        span: span.clone(),
                    }),
                }
            }

            Expression::HeadProjection { list, span } => {
                let list_val = self.interpret_expression(list)?;
                match list_val {
                    Value::List(list) => {
                        if list.is_empty() {
                            Err(InterpreterError::RuntimeError {
                                message: "Cannot get head of empty list".to_string(),
                                span: Some(span.clone()),
                            })
                        } else {
                            Ok(list[0].clone())
                        }
                    }
                    _ => Err(InterpreterError::TypeError {
                        expected: "List".to_string(),
                        found: list_val.type_name().to_string(),
                        span: span.clone(),
                    }),
                }
            }

            Expression::TailProjection { list, span } => {
                let list_val = self.interpret_expression(list)?;
                match list_val {
                    Value::List(list) => {
                        if list.is_empty() {
                            Err(InterpreterError::RuntimeError {
                                message: "Cannot get tail of empty list".to_string(),
                                span: Some(span.clone()),
                            })
                        } else {
                            // Return all elements except the first
                            Ok(Value::List(list[1..].to_vec()))
                        }
                    }
                    _ => Err(InterpreterError::TypeError {
                        expected: "List".to_string(),
                        found: list_val.type_name().to_string(),
                        span: span.clone(),
                    }),
                }
            }

            Expression::Print { value, span: _ } => {
                let val = self.interpret_expression(value)?;
                println!("{}", val);
                Ok(Value::Unit)
            }

            Expression::For {
                variable,
                iterable,
                body,
                span: _,
            } => {
                let iterable_val = self.interpret_expression(iterable)?;

                match iterable_val {
                    Value::List(elements) => {
                        // Execute the body for each element
                        self.environment.with_new_scope(|env| {
                            let mut for_interpreter = Interpreter::with_environment(env.clone());

                            for element in elements {
                                // Bind the loop variable to the current element
                                for_interpreter.environment.bind(variable.clone(), element);

                                // Execute the body (but ignore its result)
                                for_interpreter.interpret_expression(body)?;
                            }

                            Ok(Value::Unit)
                        })
                    }
                    _ => Err(InterpreterError::TypeError {
                        expected: "List".to_string(),
                        found: iterable_val.type_name().to_string(),
                        span: iterable.span().clone(),
                    }),
                }
            }

            Expression::Range {
                start,
                end,
                span: _,
            } => {
                let start_val = self.interpret_expression(start)?;
                let end_val = self.interpret_expression(end)?;

                match (start_val, end_val) {
                    (Value::Int(s), Value::Int(e)) => {
                        let mut range_list = Vec::new();
                        for i in s..e {
                            range_list.push(Value::Int(i));
                        }
                        Ok(Value::List(range_list))
                    }
                    (Value::Int(_), other) => Err(InterpreterError::TypeError {
                        expected: "Int".to_string(),
                        found: other.type_name().to_string(),
                        span: end.span().clone(),
                    }),
                    (other, _) => Err(InterpreterError::TypeError {
                        expected: "Int".to_string(),
                        found: other.type_name().to_string(),
                        span: start.span().clone(),
                    }),
                }
            }

            Expression::Concat { left, right, .. } => {
                let left_val = self.interpret_expression(left)?;
                let right_val = self.interpret_expression(right)?;

                match (left_val, right_val) {
                    (Value::String(s1), Value::String(s2)) => {
                        Ok(Value::String(format!("{}{}", s1, s2)))
                    }
                    (Value::String(_), other) => Err(InterpreterError::TypeError {
                        expected: "String".to_string(),
                        found: other.type_name().to_string(),
                        span: right.span().clone(),
                    }),
                    (other, _) => Err(InterpreterError::TypeError {
                        expected: "String".to_string(),
                        found: other.type_name().to_string(),
                        span: left.span().clone(),
                    }),
                }
            }

            Expression::CharAt { string, index, span } => {
                let string_val = self.interpret_expression(string)?;
                let index_val = self.interpret_expression(index)?;

                match (string_val, index_val) {
                    (Value::String(s), Value::Int(i)) => {
                        if i < 0 {
                            Err(InterpreterError::RuntimeError {
                                message: "String index cannot be negative".to_string(),
                                span: Some(span.clone()),
                            })
                        } else {
                            let chars: Vec<char> = s.chars().collect();
                            let index = i as usize;
                            if index < chars.len() {
                                Ok(Value::String(chars[index].to_string()))
                            } else {
                                Err(InterpreterError::RuntimeError {
                                    message: format!("String index {} out of bounds (length {})", i, chars.len()),
                                    span: Some(span.clone()),
                                })
                            }
                        }
                    }
                    (Value::String(_), other) => Err(InterpreterError::TypeError {
                        expected: "Int".to_string(),
                        found: other.type_name().to_string(),
                        span: index.span().clone(),
                    }),
                    (other, _) => Err(InterpreterError::TypeError {
                        expected: "String".to_string(),
                        found: other.type_name().to_string(),
                        span: string.span().clone(),
                    }),
                }
            }

            Expression::Length { string, .. } => {
                let string_val = self.interpret_expression(string)?;

                match string_val {
                    Value::String(s) => {
                        let length = s.chars().count() as i64;
                        Ok(Value::Int(length))
                    }
                    other => Err(InterpreterError::TypeError {
                        expected: "String".to_string(),
                        found: other.type_name().to_string(),
                        span: string.span().clone(),
                    }),
                }
            }

            Expression::ToString { expression, .. } => {
                let value = self.interpret_expression(expression)?;
                let string_representation = self.value_to_string(&value);
                Ok(Value::String(string_representation))
            }

            Expression::Case {
                expression,
                left_pattern,
                left_body,
                right_pattern,
                right_body,
                span,
            } => {
                let val = self.interpret_expression(expression)?;
                match val {
                    Value::LeftInject(inner_val) => {
                        self.environment.push_scope();
                        self.environment.bind(left_pattern.clone(), *inner_val);
                        let result = self.interpret_expression(left_body);
                        self.environment.pop_scope();
                        result
                    }
                    Value::RightInject(inner_val) => {
                        self.environment.push_scope();
                        self.environment.bind(right_pattern.clone(), *inner_val);
                        let result = self.interpret_expression(right_body);
                        self.environment.pop_scope();
                        result
                    }
                    _ => Err(InterpreterError::TypeError {
                        expected: "Sum Type".to_string(),
                        found: val.type_name().to_string(),
                        span: span.clone(),
                    }),
                }
            }

            Expression::If {
                condition,
                then_branch,
                else_branch,
                ..
            } => {
                let condition_val = self.interpret_expression(condition)?;
                match condition_val {
                    Value::Bool(true) => self.interpret_expression(then_branch),
                    Value::Bool(false) => {
                        if let Some(else_branch) = else_branch {
                            self.interpret_expression(else_branch)
                        } else {
                            Ok(Value::Unit)
                        }
                    }
                    _ => Err(InterpreterError::TypeError {
                        expected: "Bool".to_string(),
                        found: condition_val.type_name().to_string(),
                        span: condition.span().clone(),
                    }),
                }
            }
        }
    }

    /// Interpret a binary operation
    fn interpret_binary_op(
        &mut self,
        left: &Expression,
        operator: &BinaryOperator,
        right: &Expression,
        span: &Span,
    ) -> InterpreterResult<Value> {
        let left_val = self.interpret_expression(left)?;
        let right_val = self.interpret_expression(right)?;

        match operator {
            // Arithmetic operations
            BinaryOperator::Add => match (&left_val, &right_val) {
                (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l + r)),
                (Value::String(l), Value::String(r)) => Ok(Value::String(format!("{}{}", l, r))),
                _ => Err(InterpreterError::TypeError {
                    expected: "Int + Int or String + String".to_string(),
                    found: format!("{} + {}", left_val.type_name(), right_val.type_name()),
                    span: span.clone(),
                }),
            },

            BinaryOperator::Subtract => match (&left_val, &right_val) {
                (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l - r)),
                _ => Err(InterpreterError::TypeError {
                    expected: "Int - Int".to_string(),
                    found: format!("{} - {}", left_val.type_name(), right_val.type_name()),
                    span: span.clone(),
                }),
            },

            BinaryOperator::Multiply => match (&left_val, &right_val) {
                (Value::Int(l), Value::Int(r)) => Ok(Value::Int(l * r)),
                _ => Err(InterpreterError::TypeError {
                    expected: "Int * Int".to_string(),
                    found: format!("{} * {}", left_val.type_name(), right_val.type_name()),
                    span: span.clone(),
                }),
            },

            BinaryOperator::Divide => match (&left_val, &right_val) {
                (Value::Int(l), Value::Int(r)) => {
                    if *r == 0 {
                        Err(InterpreterError::DivisionByZero { span: span.clone() })
                    } else {
                        Ok(Value::Int(l / r))
                    }
                }
                _ => Err(InterpreterError::TypeError {
                    expected: "Int / Int".to_string(),
                    found: format!("{} / {}", left_val.type_name(), right_val.type_name()),
                    span: span.clone(),
                }),
            },

            // Comparison operations
            BinaryOperator::Equal => Ok(Value::Bool(left_val == right_val)),

            BinaryOperator::NotEqual => Ok(Value::Bool(left_val != right_val)),

            BinaryOperator::LessThan => match (&left_val, &right_val) {
                (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l < r)),
                _ => Err(InterpreterError::TypeError {
                    expected: "Int < Int".to_string(),
                    found: format!("{} < {}", left_val.type_name(), right_val.type_name()),
                    span: span.clone(),
                }),
            },

            BinaryOperator::LessThanEqual => match (&left_val, &right_val) {
                (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l <= r)),
                _ => Err(InterpreterError::TypeError {
                    expected: "Int <= Int".to_string(),
                    found: format!("{} <= {}", left_val.type_name(), right_val.type_name()),
                    span: span.clone(),
                }),
            },

            BinaryOperator::GreaterThan => match (&left_val, &right_val) {
                (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l > r)),
                _ => Err(InterpreterError::TypeError {
                    expected: "Int > Int".to_string(),
                    found: format!("{} > {}", left_val.type_name(), right_val.type_name()),
                    span: span.clone(),
                }),
            },

            BinaryOperator::GreaterThanEqual => match (&left_val, &right_val) {
                (Value::Int(l), Value::Int(r)) => Ok(Value::Bool(l >= r)),
                _ => Err(InterpreterError::TypeError {
                    expected: "Int >= Int".to_string(),
                    found: format!("{} >= {}", left_val.type_name(), right_val.type_name()),
                    span: span.clone(),
                }),
            },

            // Logical operations
            BinaryOperator::LogicalAnd => {
                Ok(Value::Bool(left_val.is_truthy() && right_val.is_truthy()))
            }

            BinaryOperator::LogicalOr => {
                Ok(Value::Bool(left_val.is_truthy() || right_val.is_truthy()))
            }

            // Assignment (not typically used in expressions, but included for completeness)
            BinaryOperator::Assign => Err(InterpreterError::RuntimeError {
                message: "Assignment operator not supported in expressions".to_string(),
                span: Some(span.clone()),
            }),
        }
    }

    /// Interpret a function call
    fn interpret_function_call(
        &mut self,
        function: &Expression,
        argument: &Expression,
        span: &Span,
    ) -> InterpreterResult<Value> {
        let func_val = self.interpret_expression(function)?;
        let arg_val = self.interpret_expression(argument)?;

        match func_val {
            Value::Function { param, body, env } => {
                // Create new environment with the function's captured environment
                let mut call_env = env;
                call_env.push_scope();
                call_env.bind(param, arg_val);

                // Create new interpreter with the call environment
                let mut call_interpreter = Interpreter::with_environment(call_env);
                let result = call_interpreter.interpret_expression(&body)?;

                Ok(result)
            }
            Value::FixedPoint { function } => {
                // For fixed point functions, we need to apply the function to itself first
                // This implements the Y-combinator: fix(f) = f(fix(f))
                if let Value::Function { param, body, env } = function.as_ref() {
                    // Create a new environment with the recursive parameter bound to the fixed point itself
                    let mut call_env = env.clone();
                    call_env.push_scope();
                    call_env.bind(
                        param.clone(),
                        Value::FixedPoint {
                            function: function.clone(),
                        },
                    );

                    // Now we need to interpret the body, which should return a function
                    // that we then apply to the actual argument
                    let mut recursive_interpreter = Interpreter::with_environment(call_env);
                    let inner_func = recursive_interpreter.interpret_expression(&body)?;

                    // Apply the inner function to the actual argument
                    match inner_func {
                        Value::Function {
                            param: inner_param,
                            body: inner_body,
                            env: inner_env,
                        } => {
                            let mut final_env = inner_env;
                            final_env.push_scope();
                            final_env.bind(inner_param, arg_val);

                            let mut final_interpreter = Interpreter::with_environment(final_env);
                            final_interpreter.interpret_expression(&inner_body)
                        }
                        _ => Err(InterpreterError::RuntimeError {
                            message: "Fixed point function body must return a function".to_string(),
                            span: Some(span.clone()),
                        }),
                    }
                } else {
                    Err(InterpreterError::RuntimeError {
                        message: "Invalid fixed point function".to_string(),
                        span: Some(span.clone()),
                    })
                }
            }
            _ => Err(InterpreterError::NotCallable { span: span.clone() }),
        }
    }

    /// Get the current environment (for debugging/testing)
    pub fn environment(&self) -> &Environment {
        &self.environment
    }

    /// Convert a Value to its string representation
    fn value_to_string(&self, value: &Value) -> String {
        match value {
            Value::Int(i) => i.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::String(s) => s.clone(),
            Value::Unit => "()".to_string(),
            Value::List(elements) => {
                let element_strings: Vec<String> = elements
                    .iter()
                    .map(|elem| self.value_to_string(elem))
                    .collect();
                format!("[{}]", element_strings.join(", "))
            }
            Value::Pair(first, second) => {
                format!("({}, {})", self.value_to_string(first), self.value_to_string(second))
            }
            Value::Function { .. } => "<function>".to_string(),
            Value::LeftInject(val) => format!("inl({})", self.value_to_string(val)),
            Value::RightInject(val) => format!("inr({})", self.value_to_string(val)),
            Value::FixedPoint { .. } => "<fixed-point>".to_string(),
            Value::Module { name, .. } => format!("<module {}>", name),
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
