use super::{Environment, InterpreterError, InterpreterResult, Value};
use crate::ast::nodes::{BinaryOperator, Expression, Program, Spanned, Statement};
use crate::lexer::tokens::Span;
use std::fs;
use std::path::{Path, PathBuf};

pub struct Interpreter {
    environment: Environment,
    current_directory: PathBuf,
}

impl Interpreter {
    pub fn new() -> Self {
        Self {
            environment: Environment::new(),
            current_directory: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    pub fn with_environment(environment: Environment) -> Self {
        Self {
            environment,
            current_directory: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    pub fn set_current_directory<P: AsRef<Path>>(&mut self, path: P) {
        self.current_directory = path.as_ref().to_path_buf();
    }

    pub fn interpret_program(&mut self, program: &Program) -> InterpreterResult<Value> {
        for statement in &program.statements {
            self.interpret_statement(statement)?;
        }

        Ok(Value::Unit)
    }

    pub fn interpret_program_repl(&mut self, program: &Program) -> InterpreterResult<Value> {
        let mut last_result = Value::Unit;

        for statement in &program.statements {
            last_result = self.interpret_statement(statement)?;
        }

        Ok(last_result)
    }

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
                let recursive_function = Value::Function {
                    param: name.clone(), // The recursive reference parameter
                    body: Box::new(Expression::Function {
                        param: param.clone(),
                        param_type: None,
                        body: Box::new(body.clone()),
                        span: body.span().clone(),
                    }),
                    env: self.environment.clone(),
                };

                let function_val = Value::FixedPoint {
                    function: Box::new(recursive_function),
                };

                self.environment.bind(name.clone(), function_val);
                Ok(Value::Unit)
            }
            Statement::Import { path, alias, span } => {
                let import_name = alias.as_ref().unwrap_or(path);

                let import_path = self.current_directory.join(path);

                let module_val = self.load_module(&import_path, import_name, span)?;

                self.environment.bind(import_name.clone(), module_val);
                Ok(Value::Unit)
            }
            Statement::Expression { expression, .. } => self.interpret_expression(expression),
        }
    }

    fn load_module(
        &mut self,
        path: &Path,
        module_name: &str,
        span: &Span,
    ) -> InterpreterResult<Value> {
        let content = fs::read_to_string(path).map_err(|_| InterpreterError::RuntimeError {
            message: format!("Failed to read module file: {}", path.display()),
            span: Some(span.clone()),
        })?;

        let mut lexer = crate::lexer::tokenizer::Tokenizer::new("");
        let tokens = lexer
            .tokenize(&content)
            .map_err(|e| InterpreterError::RuntimeError {
                message: format!("Failed to tokenize module {}: {}", module_name, e),
                span: Some(span.clone()),
            })?;

        let mut parser = crate::ast::parser::Parser::new(tokens);
        let program = parser.parse().map_err(|e| InterpreterError::RuntimeError {
            message: format!("Failed to parse module {}: {}", module_name, e),
            span: Some(span.clone()),
        })?;

        let mut module_interpreter = Interpreter::new();

        if let Some(parent) = path.parent() {
            module_interpreter.set_current_directory(parent);
        }

        module_interpreter
            .interpret_program(&program)
            .map_err(|e| InterpreterError::RuntimeError {
                message: format!("Failed to execute module {}: {}", module_name, e),
                span: Some(span.clone()),
            })?;

        let exports = module_interpreter.environment.get_all_bindings();

        Ok(Value::Module {
            name: module_name.to_string(),
            exports,
        })
    }

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
                    crate::ast::nodes::UnaryOperator::Negate => match operand_val {
                        Value::Int(n) => Ok(Value::Int(-n)),
                        _ => Err(InterpreterError::TypeError {
                            expected: "Int".to_string(),
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
                let func_value = self.interpret_expression(function)?;

                match func_value {
                    Value::Function { param, body, env } => Ok(Value::FixedPoint {
                        function: Box::new(Value::Function { param, body, env }),
                    }),
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
                println!("{}", self.format_for_print(&val));
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

            Expression::CharAt {
                string,
                index,
                span,
            } => {
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
                                    message: format!(
                                        "String index {} out of bounds (length {})",
                                        i,
                                        chars.len()
                                    ),
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

            Expression::TypeOf { expression, .. } => {
                let value = self.interpret_expression(expression)?;
                let type_string = self.value_to_type_string(&value);
                Ok(Value::String(type_string))
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

            BinaryOperator::LogicalAnd => {
                Ok(Value::Bool(left_val.is_truthy() && right_val.is_truthy()))
            }

            BinaryOperator::LogicalOr => {
                Ok(Value::Bool(left_val.is_truthy() || right_val.is_truthy()))
            }

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
                let mut call_env = env;
                call_env.push_scope();
                call_env.bind(param, arg_val);

                let mut call_interpreter = Interpreter::with_environment(call_env);
                let result = call_interpreter.interpret_expression(&body)?;

                Ok(result)
            }
            Value::FixedPoint { function } => {
                if let Value::Function { param, body, env } = function.as_ref() {
                    let mut call_env = env.clone();
                    call_env.push_scope();
                    call_env.bind(
                        param.clone(),
                        Value::FixedPoint {
                            function: function.clone(),
                        },
                    );

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

    pub fn environment(&self) -> &Environment {
        &self.environment
    }

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
                format!(
                    "({}, {})",
                    self.value_to_string(first),
                    self.value_to_string(second)
                )
            }
            Value::Function { .. } => "<function>".to_string(),
            Value::LeftInject(val) => format!("inl({})", self.value_to_string(val)),
            Value::RightInject(val) => format!("inr({})", self.value_to_string(val)),
            Value::FixedPoint { .. } => "<fixed-point>".to_string(),
            Value::Module { name, .. } => format!("<module {}>", name),
        }
    }

    fn format_for_print(&self, value: &Value) -> String {
        match value {
            Value::String(s) => s.clone(), // No quotes for print output
            Value::Int(n) => n.to_string(),
            Value::Bool(b) => b.to_string(),
            Value::Unit => "()".to_string(),
            Value::List(elements) => {
                let mut result = String::from("[");
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        result.push_str(", ");
                    }
                    result.push_str(&self.format_for_print(elem));
                }
                result.push(']');
                result
            }
            Value::Pair(first, second) => {
                format!(
                    "({}, {})",
                    self.format_for_print(first),
                    self.format_for_print(second)
                )
            }
            Value::Function { param, .. } => format!("<function {}>", param),
            Value::LeftInject(val) => format!("Left({})", self.format_for_print(val)),
            Value::RightInject(val) => format!("Right({})", self.format_for_print(val)),
            Value::FixedPoint { .. } => "<fixed_point>".to_string(),
            Value::Module { name, .. } => format!("<module {}>", name),
        }
    }

    fn value_to_type_string(&self, value: &Value) -> String {
        match value {
            Value::Int(_) => "Int".to_string(),
            Value::Bool(_) => "Bool".to_string(),
            Value::String(_) => "String".to_string(),
            Value::Unit => "Unit".to_string(),
            Value::List(elements) => {
                if elements.is_empty() {
                    "List Unknown".to_string()
                } else {
                    // For simplicity, assume all elements have the same type as the first
                    let element_type = self.value_to_type_string(&elements[0]);
                    format!("List {}", element_type)
                }
            }
            Value::Pair(first, second) => {
                let first_type = self.value_to_type_string(first);
                let second_type = self.value_to_type_string(second);
                format!("({}, {})", first_type, second_type)
            }
            Value::Function { param, body, .. } => {
                // Try to infer function type from parameter name and body analysis
                self.infer_function_type_string(param, body)
            }
            Value::LeftInject(val) => {
                let inner_type = self.value_to_type_string(val);
                format!("({} + Unknown)", inner_type)
            }
            Value::RightInject(val) => {
                let inner_type = self.value_to_type_string(val);
                format!("(Unknown + {})", inner_type)
            }
            Value::FixedPoint { .. } => "FixedPoint".to_string(),
            Value::Module { .. } => "Module".to_string(),
        }
    }

    fn infer_function_type_string(&self, param: &str, body: &Expression) -> String {
        let return_type = self.infer_expression_type_string(body, param);

        let param_type = self.infer_parameter_type_from_usage(param, body);

        format!("{} -> {}", param_type, return_type)
    }

    /// Infer the type of an expression for type string generation
    fn infer_expression_type_string(&self, expr: &Expression, param: &str) -> String {
        match expr {
            Expression::Block { expression, .. } => {
                if let Some(expr) = expression {
                    self.infer_expression_type_string(expr, param)
                } else {
                    "Unit".to_string()
                }
            }
            Expression::Number { .. } => "Int".to_string(),
            Expression::Boolean { .. } => "Bool".to_string(),
            Expression::String { .. } => "String".to_string(),
            Expression::Identifier { name, .. } => {
                if name == param {
                    "Unknown".to_string() // We don't know the parameter type yet
                } else {
                    // Try to look up in environment
                    "Unknown".to_string()
                }
            }
            Expression::BinaryOp {
                left,
                right,
                operator,
                ..
            } => {
                use crate::ast::nodes::BinaryOperator;
                match operator {
                    BinaryOperator::Add
                    | BinaryOperator::Subtract
                    | BinaryOperator::Multiply
                    | BinaryOperator::Divide => "Int".to_string(),
                    BinaryOperator::Equal
                    | BinaryOperator::NotEqual
                    | BinaryOperator::LessThan
                    | BinaryOperator::LessThanEqual
                    | BinaryOperator::GreaterThan
                    | BinaryOperator::GreaterThanEqual
                    | BinaryOperator::LogicalAnd
                    | BinaryOperator::LogicalOr => "Bool".to_string(),
                    _ => {
                        // For other operators, try to infer from operands
                        let left_type = self.infer_expression_type_string(left, param);
                        if left_type != "Unknown" {
                            left_type
                        } else {
                            self.infer_expression_type_string(right, param)
                        }
                    }
                }
            }
            Expression::List { elements, .. } => {
                if elements.is_empty() {
                    "List Unknown".to_string()
                } else {
                    let elem_type = self.infer_expression_type_string(&elements[0], param);
                    format!("List {}", elem_type)
                }
            }
            Expression::Pair { first, second, .. } => {
                let first_type = self.infer_expression_type_string(first, param);
                let second_type = self.infer_expression_type_string(second, param);
                format!("({}, {})", first_type, second_type)
            }
            Expression::Function { .. } => "Function".to_string(),
            Expression::Print { .. } => "Unit".to_string(),
            _ => "Unknown".to_string(),
        }
    }

    /// Infer parameter type from how it's used in the function body
    fn infer_parameter_type_from_usage(&self, param: &str, body: &Expression) -> String {
        // Simple heuristics for parameter type inference
        match body {
            Expression::Block { expression, .. } => {
                // Handle block expressions - recurse into the inner expression
                if let Some(expr) = expression {
                    self.infer_parameter_type_from_usage(param, expr)
                } else {
                    "Unknown".to_string()
                }
            }
            Expression::BinaryOp {
                left,
                right,
                operator,
                ..
            } => {
                use crate::ast::nodes::BinaryOperator;

                // Check if parameter is used directly in arithmetic operations
                let param_used_directly = matches!(left.as_ref(), Expression::Identifier { name, .. } if name == param)
                    || matches!(right.as_ref(), Expression::Identifier { name, .. } if name == param);

                if param_used_directly {
                    match operator {
                        BinaryOperator::Add
                        | BinaryOperator::Subtract
                        | BinaryOperator::Multiply
                        | BinaryOperator::Divide => "Int".to_string(),
                        BinaryOperator::LogicalAnd | BinaryOperator::LogicalOr => {
                            "Bool".to_string()
                        }
                        BinaryOperator::Equal
                        | BinaryOperator::NotEqual
                        | BinaryOperator::LessThan
                        | BinaryOperator::LessThanEqual
                        | BinaryOperator::GreaterThan
                        | BinaryOperator::GreaterThanEqual => {
                            match (left.as_ref(), right.as_ref()) {
                                (Expression::Number { .. }, _) | (_, Expression::Number { .. }) => {
                                    "Int".to_string()
                                }
                                _ => "Unknown".to_string(),
                            }
                        }
                        _ => "Unknown".to_string(),
                    }
                } else {
                    // Recurse into sub-expressions
                    let left_infer = self.infer_parameter_type_from_usage(param, left);
                    if left_infer != "Unknown" {
                        return left_infer;
                    }
                    self.infer_parameter_type_from_usage(param, right)
                }
            }
            Expression::FunctionCall {
                function, argument, ..
            } => {
                // If parameter is called as a function, it's a function type
                if let Expression::Identifier { name, .. } = function.as_ref() {
                    if name == param {
                        let arg_type = self.infer_expression_type_string(argument, param);
                        return format!("{} -> Unknown", arg_type);
                    }
                }
                // Recurse
                let func_infer = self.infer_parameter_type_from_usage(param, function);
                if func_infer != "Unknown" {
                    return func_infer;
                }
                self.infer_parameter_type_from_usage(param, argument)
            }
            Expression::FirstProjection { pair, .. }
            | Expression::SecondProjection { pair, .. } => {
                // If parameter is used in fst() or snd(), it's a pair
                if self.expression_uses_param(pair, param) {
                    "(Unknown, Unknown)".to_string()
                } else {
                    self.infer_parameter_type_from_usage(param, pair)
                }
            }
            Expression::HeadProjection { list, .. } | Expression::TailProjection { list, .. } => {
                // If parameter is used in head() or tail(), it's a list
                if self.expression_uses_param(list, param) {
                    "List Unknown".to_string()
                } else {
                    self.infer_parameter_type_from_usage(param, list)
                }
            }
            Expression::Cons { head, tail, .. } => {
                let head_infer = self.infer_parameter_type_from_usage(param, head);
                if head_infer != "Unknown" {
                    head_infer
                } else {
                    self.infer_parameter_type_from_usage(param, tail)
                }
            }
            _ => "Unknown".to_string(),
        }
    }

    fn expression_uses_param(&self, expr: &Expression, param: &str) -> bool {
        match expr {
            Expression::Block { expression, .. } => {
                if let Some(expr) = expression {
                    self.expression_uses_param(expr, param)
                } else {
                    false
                }
            }
            Expression::Identifier { name, .. } => name == param,
            Expression::BinaryOp { left, right, .. } => {
                self.expression_uses_param(left, param) || self.expression_uses_param(right, param)
            }
            Expression::FunctionCall {
                function, argument, ..
            } => {
                self.expression_uses_param(function, param)
                    || self.expression_uses_param(argument, param)
            }
            Expression::FirstProjection { pair, .. }
            | Expression::SecondProjection { pair, .. } => self.expression_uses_param(pair, param),
            Expression::HeadProjection { list, .. } | Expression::TailProjection { list, .. } => {
                self.expression_uses_param(list, param)
            }
            Expression::List { elements, .. } => elements
                .iter()
                .any(|elem| self.expression_uses_param(elem, param)),
            Expression::Pair { first, second, .. } => {
                self.expression_uses_param(first, param)
                    || self.expression_uses_param(second, param)
            }
            _ => false,
        }
    }
}

impl Default for Interpreter {
    fn default() -> Self {
        Self::new()
    }
}
