# Corrosion Programming Language

Corrosion is a statically typed interpreted functional programming language built with Rust, featuring static typing, immutable data structures, and an interactive development environment. It combines the safety of static typing with the expressiveness of functional programming paradigms.

## Note

> The TUTORIAL.md file contains a comprehensive guide to getting started with Corrosion

> The `exercises` directory includes a few programming exercises to practice and demonstrate Corrosion language

## Overview

Corrosion was designed for the [Modern and Emerging Programming Languages](https://fitech101.aalto.fi/en/courses/modern-and-emerging-programming-languages) course, serving as an educational implementation that demonstrates core compiler construction principles and functional programming concepts.

### Key Features

- **Static Type System** - Compile-time type checking with intelligent type inference
- **Functional Programming** - First-class functions, closures, and immutable data structures
- **Recursion Support** - Fixed point operator (`fix`) enabling Y-combinator patterns
- **Control Flow** - If/else expressions and for loops with range iteration and collection processing
- **Interactive REPL** - Experiment with code in real-time
- **Pattern Matching** - Destructure data with `fst`/`snd` for pairs and `head`/`tail` for lists
- **Rich Comments** - Single-line (`//`) and multi-line (`/* */`) comment support
- **Fast Compilation** - Built with Rust for performance and reliability

## Architecture

Corrosion follows a traditional compiler pipeline ensuring correctness at each stage:

```
Source Code -> Lexer -> Parser -> AST -> Type Checker -> Interpreter
```

### Components

| Component        | Purpose                                       |
| ---------------- | --------------------------------------------- |
| **Lexer**        | Converts source text into tokens              |
| **Parser**       | Builds Abstract Syntax Tree (AST) from tokens |
| **Type Checker** | Validates types and performs inference        |
| **Interpreter**  | Executes the type-checked program             |
| **REPL**         | Interactive development environment           |

## Installation

### Prerequisites

- **Rust 1.70 or later**

### Building from Source

```bash
# Clone the repository
git clone https://github.com/lauri-lyytikainen/corrosion-language.git
cd corrosion-language

# Build the project
cargo build --release
```

## Getting Started

### Interactive REPL

Start the interactive environment to experiment with the language:

```bash
cargo run

# Or run the executable directly
./target/release/corrosion-language
```

The REPL allows you to:

- Evaluate expressions immediately
- Declare variables and functions
- Test language features interactively
- Get instant feedback on type errors

### Running Programs

Execute Corrosion programs from files:

```bash
# Run a program file
cargo run program.corr
# or ./target/release/corrosion-language program.corr

# View help in REPL
help
```

### File Extension

Corrosion source files use the `.corr` extension by convention.

## Language Overview

### Type System

Corrosion features a rich type system with automatic type inference:

- **Primitive Types**: `Int`, `Bool`, `Unit`, `String`
- **Function Types**: eg. `Int -> Bool`, `(Int, Bool) -> Int`
- **List Types**: `List Int`, `List Bool`, `List String`
- **Pair Types**: `(Int, Bool)`, `(String, List Int)`
- **Sum Types**: eg. `Int + Bool` (union types)
- **Recursive Types**: Support for recursive data structures

### Data Structures

#### Lists

Immutable sequences with functional operations:

- Construction: `[1, 2, 3]`
- Combination: `cons(0, [1, 2, 3])` → `[0, 1, 2, 3]`
- Destructuring: `head([1, 2, 3])` → `1`, `tail([1, 2, 3])` → `[2, 3]`

#### Pairs

Immutable tuples for combining two values:

- Construction: `(42, true)`
- Projection: `fst((42, true))` → `42`, `snd((42, true))` → `true`

#### Functions

First-class functions with lexical scoping and optional parameter typing:

- Anonymous: `fn(x) { x + 1 }` or `fn(x: Int) { x + 1 }`
- Named: `fn increment(x: Int) { x + 1 }`
- Higher-order functions supported
- Closures capture their environment

### Syntax Highlights

- **Variables**: `let name = value;`
- **Named Functions**: `fn name(param: Type) -> Type { body }`
- **Anonymous Functions**: `fn(param: Type) { body }`

- **Imports**: `import "module.corr" as alias;`
- **Qualified Access**: `module.member`
- **Type Annotations**: `let x: Int = 42;`
- **Function Calls**: `function(argument)`
- **Conditionals**: `if condition { ... } else { ... }`
- **For Loops**: `for item in collection { ... };`
- **Range Iteration**: `for i in range(1, 10) { ... };`
- **Recursion**: `fix(fn(self) { fn(x) { ... } })`
- **Comments**: `// single line` and `/* multi-line */`
- **Operators**: Arithmetic (`+`, `-`, `*`, `/`), comparison (`==`, `<`, etc.), logical (`&&`, `||`, `!`)

## Development

### Project Structure

```
corrosion-language/
├── src/
│   ├── main.rs              # Entry point and CLI
│   ├── repl/                # Interactive REPL implementation
│   ├── lexer/               # Tokenization and lexical analysis
│   ├── ast/                 # Abstract Syntax Tree definitions
│   ├── parser/              # Parsing logic and grammar
│   ├── typechecker/         # Type inference and validation
│   └── interpreter/         # Runtime execution engine
├── tests/                   # Integration tests
├── examples/                # Example programs
├── Cargo.toml               # Project configuration
├── README.md                # Project documentation
└── TUTORIAL.md              # Getting started guide
```

### Testing

Comprehensive test suite covering all language components:

```bash
# Run all tests
cargo test

# Run specific component tests
cargo test lexer       # Lexical analysis tests
cargo test ast         # Parser and AST tests
cargo test typechecker # Type system tests
cargo test interpreter # Runtime tests

# Run with output
cargo test -- --nocapture
```

## Performance

- **Compilation**: Fast compilation times suitable for interactive development
- **Runtime**: Efficient interpretation with optimized data structures
- **Memory**: Safe memory management inherited from Rust
- **Startup**: Quick startup time for REPL

## Acknowledgments

- Built with [Rust](https://www.rust-lang.org/) and the [nom](https://github.com/Geal/nom) parsing library
