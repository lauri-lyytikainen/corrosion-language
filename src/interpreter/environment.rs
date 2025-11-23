use std::collections::HashMap;
use super::Value;

/// Environment for variable bindings during interpretation
#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    /// Stack of scopes, with the most recent scope at the end
    scopes: Vec<HashMap<String, Value>>,
}

impl Environment {
    /// Create a new empty environment
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()], // Start with one global scope
        }
    }

    /// Create a new environment with a parent environment
    pub fn with_parent(parent: &Environment) -> Self {
        let mut env = Environment::new();
        // Copy all scopes from the parent
        env.scopes = parent.scopes.clone();
        env
    }

    /// Push a new scope onto the environment stack
    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    /// Pop the most recent scope from the environment stack
    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    /// Bind a variable to a value in the current scope
    pub fn bind(&mut self, name: String, value: Value) {
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.insert(name, value);
        }
    }

    /// Get all bindings from the current environment (for module exports)
    pub fn get_all_bindings(&self) -> HashMap<String, Value> {
        let mut all_bindings = HashMap::new();
        // Collect all bindings from all scopes, with later scopes overriding earlier ones
        for scope in &self.scopes {
            all_bindings.extend(scope.clone());
        }
        all_bindings
    }

    /// Look up a variable in the environment
    /// Searches from the most recent scope backwards to the global scope
    pub fn lookup(&self, name: &str) -> Option<&Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value);
            }
        }
        None
    }

    /// Check if a variable is bound in the current (most recent) scope only
    pub fn is_bound_locally(&self, name: &str) -> bool {
        self.scopes
            .last()
            .map_or(false, |scope| scope.contains_key(name))
    }

    /// Check if a variable is bound in any scope
    pub fn is_bound(&self, name: &str) -> bool {
        self.lookup(name).is_some()
    }

    /// Get the number of scopes in the environment
    pub fn scope_count(&self) -> usize {
        self.scopes.len()
    }

    /// Execute a closure with a new scope, automatically cleaning up
    pub fn with_new_scope<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Environment) -> R,
    {
        self.push_scope();
        let result = f(self);
        self.pop_scope();
        result
    }
}

impl Default for Environment {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_binding_and_lookup() {
        let mut env = Environment::new();
        
        env.bind("x".to_string(), Value::Int(42));
        assert_eq!(env.lookup("x"), Some(&Value::Int(42)));
        assert_eq!(env.lookup("y"), None);
    }

    #[test]
    fn test_scope_management() {
        let mut env = Environment::new();
        
        // Bind in global scope
        env.bind("x".to_string(), Value::Int(1));
        
        // Push new scope and bind same variable
        env.push_scope();
        env.bind("x".to_string(), Value::Int(2));
        
        // Should find the most recent binding
        assert_eq!(env.lookup("x"), Some(&Value::Int(2)));
        
        // Pop scope and should find original binding
        env.pop_scope();
        assert_eq!(env.lookup("x"), Some(&Value::Int(1)));
    }

    #[test]
    fn test_with_new_scope() {
        let mut env = Environment::new();
        
        env.bind("x".to_string(), Value::Int(1));
        
        let result = env.with_new_scope(|env| {
            env.bind("x".to_string(), Value::Int(2));
            env.lookup("x").unwrap().clone()
        });
        
        assert_eq!(result, Value::Int(2));
        assert_eq!(env.lookup("x"), Some(&Value::Int(1))); // Original binding restored
    }
}