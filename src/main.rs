pub mod lexer;
pub mod ast;
// mod typechecker;
// mod interpreter;
mod repl;

use repl::Repl;

fn main() {
    let mut repl = Repl::new();
    repl.run();
}
