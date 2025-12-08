use crate::typechecker::Type;
use std::collections::HashMap;

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

    /// Update a variable's type in the current scope (for recursive functions)
    /// This should only be called for variables that are already bound in the current scope
    pub fn update(&mut self, name: String, ty: Type) {
        self.bindings.insert(name, ty);
    }

    /// Look up a variable type, searching parent scopes if necessary
    pub fn lookup(&self, name: &str) -> Option<&Type> {
        self.bindings
            .get(name)
            .or_else(|| self.parent.as_ref().and_then(|parent| parent.lookup(name)))
    }

    /// Check if a variable is bound in the current scope (not parent scopes)
    pub fn is_bound_locally(&self, name: &str) -> bool {
        self.bindings.contains_key(name)
    }

    /// Get all bindings in the current scope
    pub fn local_bindings(&self) -> &HashMap<String, Type> {
        &self.bindings
    }

    /// Get all bindings from all scopes (for module exports)
    pub fn get_all_bindings_types(&self) -> HashMap<String, Type> {
        let mut all_bindings = HashMap::new();

        // Start with parent bindings (lower precedence)
        if let Some(parent) = &self.parent {
            all_bindings.extend(parent.get_all_bindings_types());
        }

        // Override with current scope bindings (higher precedence)
        all_bindings.extend(self.bindings.clone());

        all_bindings
    }

    /// Enter a new scope (create a new environment with current as parent)
    pub fn enter_scope(&mut self) {
        let current = std::mem::replace(self, Self::new());
        *self = Self::with_parent(current);
    }

    /// Exit current scope (restore parent environment)
    pub fn exit_scope(&mut self) {
        if let Some(parent) = self.parent.take() {
            *self = *parent;
        }
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}
