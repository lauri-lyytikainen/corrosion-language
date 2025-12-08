use crate::lexer::tokens::Span;
use crate::typechecker::{Type, TypeError, TypeResult};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Module loader for handling imports
pub struct ModuleLoader {
    /// Current directory for resolving imports
    current_directory: PathBuf,
    /// Cache of loaded modules
    modules: HashMap<String, HashMap<String, Type>>,
}

impl ModuleLoader {
    pub fn new() -> Self {
        Self {
            current_directory: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
            modules: HashMap::new(),
        }
    }

    /// Set the current directory for import resolution
    pub fn set_current_directory<P: AsRef<Path>>(&mut self, path: P) {
        self.current_directory = path.as_ref().to_path_buf();
    }

    /// Get the current directory
    pub fn get_current_directory(&self) -> &PathBuf {
        &self.current_directory
    }

    /// Load and type-check a module from file
    pub fn load_and_check_module(
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
        let mut module_checker = crate::typechecker::TypeChecker::new();

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
        Ok(module_checker.get_environment().get_all_bindings_types())
    }

    /// Get a module's exports
    pub fn get_module_exports(&self, module_name: &str) -> Option<&HashMap<String, Type>> {
        self.modules.get(module_name)
    }

    /// Store module exports
    pub fn store_module_exports(&mut self, module_name: String, exports: HashMap<String, Type>) {
        self.modules.insert(module_name, exports);
    }

    /// Get all loaded modules
    pub fn get_modules(&self) -> &HashMap<String, HashMap<String, Type>> {
        &self.modules
    }

    /// Clone all modules (for creating child checkers)
    pub fn clone_modules(&self) -> HashMap<String, HashMap<String, Type>> {
        self.modules.clone()
    }
}

impl Default for ModuleLoader {
    fn default() -> Self {
        Self::new()
    }
}
