pub mod ast;
pub mod interpreter;
pub mod lexer;
mod repl;
mod tests;
pub mod typechecker;

#[cfg(test)]
mod sum_type_tests;

use repl::Repl;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            // No arguments - start REPL
            let mut repl = Repl::new();
            repl.run();
        }
        2 => {
            // One argument - load and execute file
            let filename = &args[1];
            if let Err(e) = load_and_execute_file(filename) {
                eprintln!("Error: {}", e);
                process::exit(1);
            }
        }
        _ => {
            eprintln!("Usage: {} [filename]", args[0]);
            eprintln!("  - Run without arguments to start the REPL");
            eprintln!("  - Provide a filename to execute that file");
            process::exit(1);
        }
    }
}

fn load_and_execute_file(filename: &str) -> Result<(), String> {
    use crate::ast::Parser;
    use crate::interpreter::Interpreter;
    use crate::lexer::Tokenizer;
    use crate::typechecker::TypeChecker;
    use std::fs;

    // Read the file contents
    let contents = fs::read_to_string(filename)
        .map_err(|e| format!("Failed to read file '{}': {}", filename, e))?;

    // Process the file contents
    let mut tokenizer = Tokenizer::new("");
    let tokens = tokenizer
        .tokenize(&contents)
        .map_err(|e| format!("Tokenization error: {}", e))?;

    let mut parser = Parser::new(tokens);
    let program = parser.parse().map_err(|e| format!("Parse error: {}", e))?;

    let mut type_checker = TypeChecker::new();
    let _typed_program = type_checker
        .check_program(&program)
        .map_err(|e| format!("Type error: {}", e))?;

    // Execute the program with the interpreter
    let mut interpreter = Interpreter::new();
    let _result = interpreter
        .interpret_program(&program)
        .map_err(|e| format!("Runtime error: {}", e))?;

    Ok(())
}
