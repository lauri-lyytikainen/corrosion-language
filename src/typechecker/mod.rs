pub mod checker;
pub mod compatibility;
pub mod environment;
pub mod errors;
pub mod inference;
pub mod module_loader;
pub mod types;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod integration_tests;

pub use checker::TypeChecker;
pub use compatibility::TypeCompatibility;
pub use environment::Environment;
pub use errors::{TypeError, TypeResult};
pub use inference::TypeInference;
pub use module_loader::ModuleLoader;
pub use types::*;
