use crate::ast::Parser;
use crate::lexer::Tokenizer;
use crate::typechecker::TypeChecker;
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
        if let Some(cmd) = line.strip_prefix(':') {
            match cmd {
                "help" => {
                    self.show_help();
                    true
                }
                "clear" => {
                    // TODO: Better clear screen implementation
                    print!("{}[2J{}[H", 27 as char, 27 as char);
                    true
                }
                _ if cmd.starts_with("load ") => {
                    let filename = cmd.strip_prefix("load ").unwrap().trim();
                    match self.load_file(filename) {
                        Ok(result) => println!("{}", result),
                        Err(error) => println!("Error loading file: {}", error),
                    }
                    true
                }
                _ => {
                    println!("Unknown command: :{}", cmd);
                    println!("Type ':help' for available commands.");
                    true
                }
            }
        } else {
            match line {
                "help" | ":help" => {
                    self.show_help();
                    true
                }
                "clear" | ":clear" => {
                    print!("{}[2J{}[H", 27 as char, 27 as char);
                    true
                }
                _ => false,
            }
        }
    }

    fn show_help(&self) {
        println!("Corrosion Language REPL Commands:");
        println!("  help, :help       - Show this help message");
        println!("  clear, :clear     - Clear the screen");
        println!("  :load <filename>  - Load and execute a Corrosion file");
        println!("  exit, quit        - Exit the REPL");
        println!("  <expression>      - Evaluate a Corrosion expression");
        println!();
    }

    fn load_file(&mut self, filename: &str) -> Result<String, String> {
        use std::fs;

        // Read the file contents
        let contents = fs::read_to_string(filename)
            .map_err(|e| format!("Failed to read file '{}': {}", filename, e))?;

        // Process the file contents using the same pipeline as process_line
        match self.process_content(&contents) {
            Ok(result) => Ok(format!("Successfully loaded '{}': {}", filename, result)),
            Err(error) => Err(format!("Error processing '{}': {}", filename, error)),
        }
    }

    fn process_content(&mut self, content: &str) -> Result<String, String> {
        // Step 1: Tokenize the input using the tokenizer
        let mut tokenizer = Tokenizer::new("");
        let tokens = tokenizer.tokenize(content).map_err(|e| e.to_string())?;

        // Step 2: Parse tokens into an AST
        let mut parser = Parser::new(tokens);
        let program = parser.parse().map_err(|e| e.to_string())?;

        // Step 3: Type check the AST
        let mut type_checker = TypeChecker::new();
        let typed_program = type_checker
            .check_program(&program)
            .map_err(|e| e.to_string())?;

        // For now, show the typed AST
        Ok(format!("Typed AST: {:#?}", typed_program))
    }

    fn process_line(&mut self, input: &str) -> Result<String, String> {
        self.process_content(input)
    }
}

impl Default for Repl {
    fn default() -> Self {
        Self::new()
    }
}
