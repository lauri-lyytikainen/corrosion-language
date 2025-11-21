/// Runtime values in the Corrosion language
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// Integer value
    Int(i64),
    /// Boolean value
    Bool(bool),
    /// Unit value (void)
    Unit,
    /// List of values
    List(Vec<Value>),
    /// Pair of two values
    Pair(Box<Value>, Box<Value>),
    /// Function value (closure)
    Function {
        param: String,
        body: Box<crate::ast::nodes::Expression>,
        env: super::Environment,
    },
    /// Left injection of sum type
    LeftInject(Box<Value>),
    /// Right injection of sum type
    RightInject(Box<Value>),
    /// Fixed point value for recursive functions
    FixedPoint { function: Box<Value> },
}

impl Value {
    /// Get the type name as a string for error messages
    pub fn type_name(&self) -> &'static str {
        match self {
            Value::Int(_) => "Int",
            Value::Bool(_) => "Bool",
            Value::Unit => "Unit",
            Value::List(_) => "List",
            Value::Pair(_, _) => "Pair",
            Value::Function { .. } => "Function",
            Value::LeftInject(_) => "LeftInject",
            Value::RightInject(_) => "RightInject",
            Value::FixedPoint { .. } => "FixedPoint",
        }
    }

    /// Check if this value is truthy (for conditional expressions)
    pub fn is_truthy(&self) -> bool {
        match self {
            Value::Bool(b) => *b,
            Value::Unit => false,
            Value::Int(n) => *n != 0,
            Value::List(list) => !list.is_empty(),
            Value::FixedPoint { .. } => true, // Fixed point functions are truthy
            _ => true,                        // Other values are considered truthy
        }
    }

    /// Convert to a boolean value if possible
    pub fn to_bool(&self) -> Option<bool> {
        match self {
            Value::Bool(b) => Some(*b),
            _ => None,
        }
    }

    /// Convert to an integer value if possible
    pub fn to_int(&self) -> Option<i64> {
        match self {
            Value::Int(n) => Some(*n),
            _ => None,
        }
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Int(n) => write!(f, "{}", n),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Unit => write!(f, "()"),
            Value::List(elements) => {
                write!(f, "[")?;
                for (i, elem) in elements.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", elem)?;
                }
                write!(f, "]")
            }
            Value::Pair(first, second) => {
                write!(f, "({}, {})", first, second)
            }
            Value::Function { param, .. } => {
                write!(f, "<function {}>", param)
            }
            Value::LeftInject(value) => {
                write!(f, "Left({})", value)
            }
            Value::RightInject(value) => {
                write!(f, "Right({})", value)
            }
            Value::FixedPoint { .. } => {
                write!(f, "<recursive function>")
            }
        }
    }
}
