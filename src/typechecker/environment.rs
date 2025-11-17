use std::collections::HashMap;
use crate::typechecker::Type;

/// Type environment for variable bindings
#[derive(Debug, Clone)]
pub struct Environment {
    bindings: HashMap<String, Type>,
    parent: Option<Box<Environment>>,
}

impl Environment {
    /// Create a new empty environment
    pub fn new() -> Self {
        Self {
            bindings: HashMap::new(),
            parent: None,
        }
    }

    /// Create a new environment with a parent scope
    pub fn with_parent(parent: Environment) -> Self {
        Self {
            bindings: HashMap::new(),
            parent: Some(Box::new(parent)),
        }
    }

    /// Bind a variable to a type in the current scope
    pub fn bind(&mut self, name: String, ty: Type) {
        self.bindings.insert(name, ty);
    }

    /// Look up a variable type, searching parent scopes if necessary
    pub fn lookup(&self, name: &str) -> Option<&Type> {
        self.bindings.get(name).or_else(|| {
            self.parent.as_ref().and_then(|parent| parent.lookup(name))
        })
    }

    /// Check if a variable is bound in the current scope (not parent scopes)
    pub fn is_bound_locally(&self, name: &str) -> bool {
        self.bindings.contains_key(name)
    }

    /// Get all bindings in the current scope
    pub fn local_bindings(&self) -> &HashMap<String, Type> {
        &self.bindings
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}