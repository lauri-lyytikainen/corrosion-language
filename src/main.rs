pub mod ast;
pub mod lexer;
pub mod typechecker;
// mod interpreter;
mod repl;
mod tests;

use repl::Repl;

fn main() {
    let mut repl = Repl::new();
    repl.run();
}
