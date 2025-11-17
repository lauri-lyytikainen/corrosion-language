use crate::lexer::Tokenizer;
use crate::ast::Parser;
use std::io::{self, Write};

pub struct Repl {
    // TODO: Add fields for maintaining REPL state
    // - History of commands
    // - Current environment/scope
    // - Configuration options
    version: &'static str,
}

impl Repl {
    pub fn new() -> Self {
        Self {
            version: env!("CARGO_PKG_VERSION"),
        }
    }

    pub fn run(&mut self) {
        println!("Corrosion Language REPL v{}", self.version);
        println!("Type 'exit' or 'quit' to exit\n");

        let mut input = String::new();

        loop {
            print!("> ");
            io::stdout().flush().unwrap();

            input.clear();
            match io::stdin().read_line(&mut input) {
                Ok(_) => {
                    let line = input.trim();

                    if line == "exit" || line == "quit" {
                        println!("Goodbye!");
                        break;
                    }

                    if line.is_empty() {
                        continue;
                    }

                    if self.handle_command(line) {
                        continue;
                    }

                    match self.process_line(line) {
                        Ok(result) => println!("{}", result),
                        Err(error) => println!("Error: {}", error),
                    }
                }
                Err(error) => {
                    println!("Error reading input: {}", error);
                    break;
                }
            }
        }
    }

    fn handle_command(&mut self, line: &str) -> bool {
        match line {
            "help" | ":help" => {
                self.show_help();
                true
            }
            "clear" | ":clear" => {
                // TODO: Better clear screen implementation
                print!("{}[2J{}[H", 27 as char, 27 as char);
                true
            }
            _ => false,
        }
    }

    fn show_help(&self) {
        println!("Corrosion Language REPL Commands:");
        println!("  help, :help     - Show this help message");
        println!("  clear, :clear   - Clear the screen");
        println!("  exit, quit      - Exit the REPL");
        println!("  <expression>    - Evaluate a Corrosion expression");
        println!();
    }

    fn process_line(&mut self, input: &str) -> Result<String, String> {
        // Step 1: Tokenize the input using the tokenizer
        let mut tokenizer = Tokenizer::new("");
        let tokens = tokenizer.tokenize(input).map_err(|e| e.to_string())?;

        // Step 2: Parse tokens into an AST
        let mut parser = Parser::new(tokens);
        let program = parser.parse().map_err(|e| e.to_string())?;

        // For now, show both tokens and AST
        Ok(format!("AST: {:#?}", program))

        // TODO: Continue with the compilation pipeline
        // 3. Type check the AST
        // 4. Interpret/evaluate the AST
    }
}

impl Default for Repl {
    fn default() -> Self {
        Self::new()
    }
}
