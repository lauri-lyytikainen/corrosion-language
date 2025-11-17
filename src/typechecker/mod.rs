pub mod types;
pub mod checker;
pub mod environment;

#[cfg(test)]
mod tests;

#[cfg(test)]
mod integration_tests;

pub use types::*;
pub use checker::TypeChecker;
pub use environment::Environment;