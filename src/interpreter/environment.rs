use super::Value;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    scopes: Vec<HashMap<String, Value>>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()], // Start with one global scope
        }
    }

    pub fn with_parent(parent: &Environment) -> Self {
        let mut env = Environment::new();
        env.scopes = parent.scopes.clone();
        env
    }

    pub fn push_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn pop_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        }
    }

    pub fn bind(&mut self, name: String, value: Value) {
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.insert(name, value);
        }
    }

    pub fn update(&mut self, name: String, value: Value) {
        if let Some(current_scope) = self.scopes.last_mut() {
            current_scope.insert(name, value);
        }
    }

    pub fn get_all_bindings(&self) -> HashMap<String, Value> {
        let mut all_bindings = HashMap::new();
        for scope in &self.scopes {
            all_bindings.extend(scope.clone());
        }
        all_bindings
    }

    pub fn lookup(&self, name: &str) -> Option<&Value> {
        for scope in self.scopes.iter().rev() {
            if let Some(value) = scope.get(name) {
                return Some(value);
            }
        }
        None
    }

    pub fn is_bound_locally(&self, name: &str) -> bool {
        self.scopes
            .last()
            .map_or(false, |scope| scope.contains_key(name))
    }

    pub fn is_bound(&self, name: &str) -> bool {
        self.lookup(name).is_some()
    }

    pub fn scope_count(&self) -> usize {
        self.scopes.len()
    }

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

        env.bind("x".to_string(), Value::Int(1));

        env.push_scope();
        env.bind("x".to_string(), Value::Int(2));

        assert_eq!(env.lookup("x"), Some(&Value::Int(2)));

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
