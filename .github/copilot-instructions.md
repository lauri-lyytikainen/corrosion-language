# Corrosion Language - AI Coding Agent Instructions

## Project Overview

This is a Rust-based programming language implementation called "Corrosion". The project follows a traditional compiler architecture with distinct phases: lexical analysis, parsing, type checking, and interpretation.

Corrosion is a functional programming language with support for:

- **Static typing** with type inference
- **Immutable data structures** (lists, pairs)
- **First-class functions** with closures
- **Recursion** via fixed point operator (`fix`)
- **Control flow** with for loops and range iteration
- **Interactive development** via REPL

## Architecture & Module Structure

### Core Pipeline Flow

```
REPL → Lexer → Parser → AST → TypeChecker → Interpreter
```

- **`src/repl/`** - Interactive Read-Eval-Print Loop for language experimentation
- **`src/lexer/`** - Tokenization of source code into lexical tokens
- **`src/parser/`** - Converts tokens into Abstract Syntax Tree (AST)
- **`src/ast/`** - AST node definitions and tree structures
- **`src/typechecker/`** - Static type analysis and validation
- **`src/interpreter/`** - Runtime execution of type-checked AST
- **`src/main.rs`** - Entry point that starts the REPL

### Module Organization Pattern

Each phase should be implemented as a separate module with:

- `mod.rs` - Public API and main structures
- Individual files for specific functionality (e.g., `tokens.rs`, `expressions.rs`)
- Clear interfaces between phases using well-defined data structures

## Development Workflow

### Build & Run Commands

```bash
# Build the project
cargo build

# Run the REPL (main interface)
cargo run

# Run tests
cargo test

# Check without building
cargo check
```

### REPL Usage

The REPL provides an interactive environment for testing language features:

- Type expressions to evaluate them immediately
- Use `:help` to see available commands
- Use `:clear` to clear the screen
- Use `:load filename.corr` to load and execute files
- Type `exit` or `quit` to exit
- Each line is processed through the full compilation pipeline
- **Test recursion**: Try `fix(fn(f) { fn(x) { x } })(42);`
- **Test loops**: Try `for i in range(1, 5) { print(i); };`
- **Test data structures**: Try `cons(1, [2, 3]);` or `fst((10, 20));`

### Project Conventions

1. **Error Handling**: Use `Result<T, E>` types throughout the pipeline

   - Define custom error types for each phase (LexError, ParseError, TypeError, RuntimeError)
   - Propagate errors up the compilation pipeline with context

2. **Testing Strategy**:

   - Unit tests for individual components in each module
   - Integration tests in `tests/` for end-to-end language features
   - Test files should use `.corr` extension (assumed language extension)

3. **AST Design**:
   - Use Rust enums for AST node types
   - Include source location information (line, column) for error reporting
   - Consider using `Box<T>` for recursive structures to avoid infinite size

## Language Features

### Core Language Constructs

1. **Variables and Functions**

   - `let` declarations for immutable variables
   - `fn` for function definitions with closures
   - First-class functions and higher-order functions

2. **Data Types**

   - **Primitives**: `Int`, `Bool`, `Unit`
   - **Lists**: Homogeneous collections `[1, 2, 3]`
   - **Pairs**: Heterogeneous tuples `(42, true)`
   - **Functions**: `Int -> Int`, `(Int, Bool) -> Int`

3. **List Operations**

   - `cons(element, list)` - Prepend element to list
   - `head(list)` - Get first element
   - `tail(list)` - Get all elements except first

4. **Pair Operations**

   - `fst(pair)` - Get first element
   - `snd(pair)` - Get second element

5. **Control Flow**

   - **For loops**: `for x in collection { ... }`
   - **Range iteration**: `for i in range(1, 10) { ... }`
   - **Nested loops**: Support for multiple levels of iteration

6. **Recursion**

   - **Fixed point operator**: `fix(fn(self) { ... })` enables Y-combinator recursion
   - **Self-referential functions**: Functions can call themselves via the `self` parameter
   - **Higher-order recursion**: Recursive functions that return other functions

7. **Type System**
   - Static type checking with inference
   - Function types: `T -> U`
   - Generic list types: `List T`
   - Pair types: `(T, U)`
   - Error reporting with source locations

### Syntax Examples

```corrosion
// Variables and functions
let x = 42;
let add = fn(a) { fn(b) { a + b } };

// Control flow
for i in range(1, 5) {
    print(i);
};

// Recursion
let factorial = fix(fn(self) {
    fn(n) {
        // Would use conditionals when available
        n  // Simplified for now
    }
});

// Data structures
let numbers = [1, 2, 3];
let point = (10, 20);
let nested = cons(1, [2, 3]);  // [1, 2, 3]
```

## Key Implementation Considerations

### REPL Implementation

- Interactive shell for testing language features during development
- Handles line-by-line input with immediate evaluation and feedback
- Supports REPL commands (`:help`, `:clear`) alongside language expressions
- Maintains session state and history for user convenience
- Entry point for the full compilation pipeline

### Lexer Implementation

- Define token types as Rust enums
- Track position information for error reporting
- Handle whitespace, comments, and string literals appropriately

### Parser Implementation

- Recursive descent parser is typical for this architecture
- Implement precedence climbing for expressions
- Provide meaningful error messages with source locations

### Type System

- Define type representations (primitives, functions, structs, etc.)
- Implement type inference where applicable
- Handle forward references and recursive types

### Interpreter

- Environment/scope management for variable bindings
- Value representation for runtime data
- Built-in functions and operators
- **Loop execution**: For loop iteration over ranges and collections
- **Recursion support**: Fixed point operator implementation with `FixedPoint` value type
- **Range generation**: Built-in `range(start, end)` function for numeric sequences

## File Patterns & Naming

- Use `snake_case` for file and module names
- Module files: `mod.rs` for public interface
- Implementation files: descriptive names like `expressions.rs`, `statements.rs`
- Test files: `tests.rs` within each module or `tests/` directory

## Dependencies Management

Currently using no external dependencies - prefer implementing core functionality with std library where possible. Consider adding dependencies for:

- Error handling (e.g., `thiserror`, `anyhow`)
- CLI parsing (e.g., `clap`) when adding command-line interface
- Testing utilities (e.g., `pretty_assertions`)

## Implementation Notes for Key Features

### For Loops Implementation

- **AST Node**: `Expression::For { variable, iterable, body, span }`
- **Parsing**: Handles `for variable in iterable { body }` syntax
- **Type Checking**: Ensures iterable is a list or range, body type checking in new scope
- **Interpretation**: Creates new scope with loop variable, iterates over collection

### Fixed Point Operator Implementation

- **AST Node**: `Expression::Fix { function, span }`
- **Value Type**: `Value::FixedPoint { function }` for recursive functions
- **Parsing**: `fix(function_expression)` syntax
- **Type Checking**: Ensures function has appropriate recursive type signature
- **Interpretation**: Lazy evaluation with Y-combinator semantics
- **Recursion**: Enables `fix(fn(self) { ... })` where `self` is the recursive call

### Range Function Implementation

- **AST Node**: `Expression::Range { start, end, span }`
- **Parsing**: `range(start_expr, end_expr)` syntax
- **Type Checking**: Both expressions must be `Int` type
- **Interpretation**: Generates list of integers from start to end-1 (exclusive end)

### Value Types Extended

```rust
enum Value {
    Int(i64),
    Bool(bool),
    Unit,
    List(Vec<Value>),
    Pair(Box<Value>, Box<Value>),
    Function { param: String, body: Box<Expression>, env: Environment },
    FixedPoint { function: Box<Value> },  // New for recursion
}
```

## Common Pitfalls to Avoid

- Don't mix compilation phases - keep lexer, parser, typechecker, interpreter separate
- Avoid circular dependencies between modules
- Include comprehensive error context for debugging language programs
- Plan for extensibility in AST design (visitors, transformations)
- **For loops**: Ensure proper scope management for loop variables
- **Recursion**: Prevent infinite loops in fixed point evaluation through lazy evaluation
- **Type checking**: Handle recursive function types correctly in the type system
