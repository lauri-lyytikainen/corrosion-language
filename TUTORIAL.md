# Corrosion Language Tutorial

Welcome to the Corrosion programming language! This tutorial will guide you through all the language features, from basic syntax to advanced functional programming concepts.

## Table of Contents

1. [Getting Started](#getting-started)
2. [Basic Syntax](#basic-syntax)
3. [Variables and Types](#variables-and-types)
4. [Expressions and Operations](#expressions-and-operations)
5. [Functions](#functions)
6. [Data Structures](#data-structures)
7. [List Operations](#list-operations)
8. [Pair Operations](#pair-operations)
9. [Control Flow](#control-flow)
10. [Recursion](#recursion)
11. [Type System](#type-system)
12. [Output](#output)
13. [String Operations](#string-operations)
14. [Comments](#comments)
15. [Error Handling](#error-handling)
16. [Advanced Topics](#advanced-topics)

## Getting Started

### Running the REPL

Start the interactive environment to follow along with this tutorial:

```bash
cargo run
```

The REPL (Read-Eval-Print Loop) allows you to experiment with code interactively. You can type expressions and see their results immediately.

### Running Programs

You can also save code in `.corr` files and run them:

```bash
cargo run program.corr
```

## Basic Syntax

### Keywords

Corrosion has the following reserved keywords:

- `let` - Variable declaration
- `fn` - Function definition
- `fix` - Fixed point operator for recursion
- `if`, `else` - Conditional expressions
- `for`, `in` - Loop constructs
- `range` - Range generation
- `true`, `false` - Boolean literals
- `print` - Output statement
- `cons`, `head`, `tail` - List operations
- `fst`, `snd` - Pair operations

### Statements

Every statement in Corrosion must end with a semicolon (`;`):

```rust
42;
true;
let x = 10;
print(x);
```

Output:

```rust
10
```

### Expressions and Output

To display values, use the `print` statement:

```rust
let x = 5;
let y = 10;
print(x + y);
```

Output:

```rust
15
```

## Variables and Types

### Variable Declaration

Variables are declared using the `let` keyword:

```rust
let x = 42;
let name = "Hello";
let flag = true;
```

### Type Annotations

You can explicitly specify types:

```rust
let x: Int = 42;
print(type(x));

let flag: Bool = true;
print(type(flag));

let numbers: List Int = [1, 2, 3];
print(type(numbers));
```

Output

```rust
Int
Bool
List Int
```

### Immutability

All variables in Corrosion are immutable by default. Once assigned, their values cannot be changed:

```rust
let x = 10;
let x = 20;
```

Output:

```rust
Error: Type error: Variable 'x' redefined at line 2, column 1
```

## Expressions and Operations

### Arithmetic Operations

```rust
let a = 10 + 5;     // Addition: 15
let b = 20 - 8;     // Subtraction: 12
let c = 6 * 7;      // Multiplication: 42
let d = 15 / 3;     // Division: 5
```

### Comparison Operations

```rust
let eq = 10 == 10;    // Equality: true
let neq = 5 != 3;     // Inequality: true
let lt = 5 < 10;      // Less than: true
let lte = 5 <= 5;     // Less than or equal: true
let gt = 10 > 5;      // Greater than: true
let gte = 10 >= 10;   // Greater than or equal: true
```

### Logical Operations

```rust
let and_op = true && false;   // Logical AND: false
let or_op = true || false;    // Logical OR: true
let not_op = !true;           // Logical NOT: false
```

### Operator Precedence

Operators follow standard mathematical precedence:

```rust
let result = 2 + 3 * 4;
print(result);  // 14, because multiplication has higher precedence than addition
let result2 = (2 + 3) * 4;
print(result2);  // 20
```

Output:

```rust
14
20
```

## Functions

### Function Definition

Functions are defined using the `fn` keyword. You can optionally specify parameter types for better type safety and documentation:

```rust
let add_one_lambda = fn(x) { x + 1 };
fn add_one(x) {  // Named function
    x + 1
}
print(add_one_lambda(5));  // 6
print(add_one(10));        // 11

// With explicit parameter types
let greet_lambda = fn(name: String) { concat("Hello, ", name) };
fn greet(name: String) { // Named function with typed parameter
    concat("Hello, ", name)
}
print(greet_lambda("Alice"));  // "Hello, Alice"
print(greet("Bob"));           // "Hello, Bob"
```

§ Output:

```rust
6
11
Hello, Alice
Hello, Bob
```

### Function Calls

Functions are called by passing arguments in parentheses:

```rust
fn add_one(x) {
    x + 1
}

let result = add_one(5);
print(result);
```

Output:

```rust
6
```

### Higher-Order Functions

Functions can take other functions as arguments:

```rust
fn add_one(x) {
    x + 1
}

let apply_twice = fn(f) {
    fn(x) { f(f(x)) }
};

let add_two = apply_twice(add_one);
let result = add_two(5);
print(result);  // Prints: 7
```

Output:

```rust
7
```

### Closures

Functions capture variables from their environment. Parameter types can be specified:

```rust
let multiplier = 3;
let multiply_by_three = fn(x: Int) { x * multiplier };
let result = multiply_by_three(4);
print(result);  // Prints: 12

// Closures that return functions (currying)
let make_calculator = fn(base: Int) {
    fn(factor: Int) { base * multiplier + factor }
};
let calculator = make_calculator(10);
let final_result = calculator(5);
print(final_result);  // Prints: 35 (10 * 3 + 5)
```

Output:

```rust
12
35
```

### When to Use Parameter Typing

Parameter typing is optional in Corrosion, but it's recommended in these situations:

- **Documentation**: Makes function interfaces clear to other developers
- **Type Safety**: Catches type errors early during compilation
- **Complex Functions**: When parameter types aren't obvious from context

```rust
// Good candidates for explicit typing
fn validate_input(data: String) { /* validation logic */ }
fn process_numbers(values: List Int) { /* processing */ }

// Type inference works well for simple cases
let double = fn(x) { x * 2 };  // Type is inferred as Int -> Int
```

## Data Structures

### Lists

Lists are ordered collections of elements of the same type:

```rust
let empty_list = [];
let numbers = [1, 2, 3, 4, 5];
let booleans = [true, false, true];
```

#### Nested Lists

Lists can contain other lists:

```rust
let matrix = [[1, 2], [3, 4], [5, 6]];
```

### Pairs

Pairs combine exactly two values (which can be of different types):

```rust
let coordinate = (10, 20);
let mixed_pair = (42, true);
let nested_pair = ((1, 2), (3, 4));
```

## List Operations

### `cons` - Prepending Elements

The `cons` operation adds an element to the front of a list:

```rust
let original = [2, 3, 4];
let extended = cons(1, original);
print(extended);  // Prints: [1, 2, 3, 4]
```

Output:

```rust
[1, 2, 3, 4]
```

### `head` - Getting the First Element

The `head` operation returns the first element of a list:

```rust
let numbers = [1, 2, 3];
let first = head(numbers);
print(first);  // Prints: 1
```

Output:

```rust
1
```

**Note**: `head` on an empty list will cause a runtime error.

### `tail` - Getting All But the First Element

The `tail` operation returns a new list with all elements except the first:

```rust
let numbers = [1, 2, 3, 4];
let rest = tail(numbers);
print(rest);  // Prints: [2, 3, 4]
```

Output:

```rust
[2, 3, 4]
```

p
**Note**: `tail` on an empty list will cause a runtime error.

#### List length

There is no built-in `length` function for lists, but you can use function like this to get the length of a list:

```rust
let numbers = [1, 2, 3, 4, 5];

let list_length = fix(fn(self) {
    fn(lst: List Int) {
        if lst == [] {
            0
        } else {
            1 + self(tail(lst))
        }
    }
});

print(list_length(numbers));

```

### List Processing Patterns

#### Decomposition and Reconstruction

```rust
let original = [1, 2, 3];
let first_elem = head(original);     // 1
let remaining = tail(original);      // [2, 3]
let reconstructed = cons(first_elem, remaining);  // [1, 2, 3]
```

#### Building Lists

```rust
let empty = [];
let one_item = cons(1, empty);          // [1]
let two_items = cons(2, one_item);      // [2, 1]
let three_items = cons(3, two_items);   // [3, 2, 1]
```

## Pair Operations

### `fst` - Getting the First Element

```rust
let point = (10, 20);
let x_coord = fst(point);
print(x_coord);  // Prints: 10
```

Output:

```rust
10
```

### `snd` - Getting the Second Element

```rust
let point = (10, 20);
let y_coord = snd(point);
print(y_coord);  // Prints: 20
```

Output:

```rust
20
```

### Nested Pair Access

```rust
let nested = ((1, 2), (3, 4));
let first_pair = fst(nested);    // (1, 2)
let inner_first = fst(first_pair);  // 1
let second_pair = snd(nested);   // (3, 4)
let inner_second = snd(second_pair);  // 4
```

## Control Flow

### If Expressions

Corrosion supports conditional execution with `if` expressions:

#### Basic If Expression

```rust
if condition {
    // code to execute if condition is true
};
```

#### If-Else Expression

```rust
let result = if x > 10 {
    "large"
} else {
    "small"
};
```

#### If Expression Rules

1. **Condition must be Bool**: The condition expression must evaluate to a boolean value:

```rust
if true {
    print("Always executes");
};

let x = 5;
if x > 0 {
    print("Positive number");
};

if false {
    print("This won't execute");
};
```

Output:

```rust
Always executes
Positive number
```

2. **Unit type for statements**: If expressions without else must return Unit:

```rust
// Valid - statement form
if condition {
    print("Something");
};

// Invalid - expression form without else
// let x = if condition { 42 };
```

#### Nested If Expressions

```rust
let grade = if score >= 90 {
    "A"
} else {
    if score >= 80 {
        "B"
    } else {
        if score >= 70 {
            "C"
        } else {
            "F"
        }
    }
};
```

#### Combining with Other Features

```rust
// If with function calls
let numbers = [1, 2, 3, 4, 5];
for num in numbers {
    if num > 3 {
        print(num);
    };
};

// If in function bodies
let abs = fn(x) {
    if x >= 0 {
        x
    } else {
        0 - x
    }
};
```

### For Loops

Corrosion supports for loops for iterating over ranges and collections:

```rust
// Print numbers from 1 to 10
for i in range(1, 11) {
    print(i);
};

// Range is exclusive of the end value
for x in range(0, 5) {
    print(x);  // Prints: 0, 1, 2, 3, 4
};
```

Output:

```rust
1
2
3
4
5
6
7
8
9
10
0
1
2
3
4
```

#### Iterating Over Lists

```rust
let numbers = [10, 20, 30, 40];
for num in numbers {
    print(num);  // Prints each number
};

let words = ["hello", "world"];
for word in words {
    print(word);
};
```

Output:

```rust
10
20
30
40
hello
world
```

#### Loop Body

The loop body can contain any valid statements:

```rust
let base = 5;
for i in range(1, 4) {
    let result = base * i;
    print(result);  // Prints: 5, 10, 15
};
```

Output:

```rust
5
10
15
```

#### Nested Loops

```rust
for i in range(1, 4) {
    for j in range(1, 3) {
        print((i, j));  // Prints pairs: (1,1), (1,2), (2,1), (2,2), (3,1), (3,2)
    };
};
```

Output:

```rust
(1, 1)
(1, 2)
(2, 1)
(2, 2)
(3, 1)
(3, 2)
```

### Range Function

The `range` function creates a sequence of integers:

```rust
// range(start, end) - generates numbers from start to end-1
let r1 = range(0, 5);   // [0, 1, 2, 3, 4]
let r2 = range(10, 13); // [10, 11, 12]
let r3 = range(5, 5);   // [] (empty range)
```

**Note**: The `range` function is exclusive of the end value, meaning `range(1, 4)` produces `[1, 2, 3]`.

## Recursion

### Fixed Point Operator

Corrosion supports recursion through the fixed point operator (`fix`), which implements the Y-combinator pattern:

#### Basic Syntax

```rust
fix(fn(recursive_function) {
    fn(argument) {
        // function body that can call recursive_function
    }
})
```

#### Simple Examples

```rust
// Identity function using fix
let identity = fix(fn(f) { fn(x) { x } });
print(identity(42));  // Prints: 42

// Function that adds 1
let add_one = fix(fn(f) { fn(x) { x + 1 } });
print(add_one(10));  // Prints: 11
```

Output:

```rust
42
11
```

#### Function Composition with Recursion

```rust
// Apply a function twice
let apply_twice = fix(fn(self) {
    fn(func) {
        fn(value) {
            func(func(value))
        }
    }
});

let double = fn(x) { x * 2 };
let quadruple = apply_twice(double);
print(quadruple(5));  // Prints: 20 (5 * 2 * 2)
```

Output:

```rust
20
```

#### Higher-Order Recursive Functions

```rust
// Create parameterized functions
let make_adder = fix(fn(self) {
    fn(n) {
        fn(x) {
            x + n
        }
    }
});

let add_10 = make_adder(10);
print(add_10(5));  // Prints: 15
```

Output:

```rust
15
```

#### Mathematical Foundation

The fixed point operator implements the Y-combinator:

- `fix(f)` returns a value `x` such that `f(x) = x`
- This enables recursive function definitions without explicit self-reference
- The pattern `fix(fn(self) { ... })` allows the function to call itself via `self`

#### Recursive Patterns

While Corrosion doesn't yet have conditional expressions, you can build recursive structures:

```rust
// Simple recursive patterns work well
let make_multiplier = fix(fn(self) {
    fn(factor) {
        fn(value) {
            value * factor
        }
    }
});

let times_three = make_multiplier(3);
print(times_three(10)); // Prints: 30
```

Output:

```rust
30
```

### Higher-Order Functions

The type system fully supports higher-order functions - functions that take other functions as parameters:

```rust
// Apply a function twice
let apply_twice = fn(f) { fn(x) { f(f(x)) } };

let increment = fn(x) { x + 1 };
print(apply_twice(increment)(5)); // Prints: 7

// Function composition
let compose = fn(f) { fn(g) { fn(x) { f(g(x)) } } };
let add_two = fn(n) { n + 2 };
print(compose(increment)(add_two)(10)); // Prints: 13
```

Output:

```rust
7
13
```

## Type System

### Basic Types

- **`Int`**: Integer numbers (`42`, `-17`, `0`)
- **`Bool`**: Boolean values (`true`, `false`)
- **`Unit`**: The unit type (represents "no value")

### Composite Types

#### Function Types

Function types show the input and output types:

```rust
// Int -> Int (takes Int, returns Int)
let add_one: Int -> Int = fn(x) { x + 1 };

// Int -> (Int -> Int) (curried: takes Int, returns function that takes Int and returns Int)
let add: Int -> (Int -> Int) = fn(x) { fn(y) { x + y } };

print(add_one(5));      //  6
print(add(3)(4));       //  7
```

Output:

```rust
6
7
```

#### List Types

List types specify the element type:

```rust
let numbers: List Int = [1, 2, 3];
let flags: List Bool = [true, false];
let lists: List (List Int) = [[1, 2], [3, 4]];
```

#### Pair Types

Pair types show both element types:

```rust
let point: (Int, Int) = (10, 20);
let mixed: (Int, Bool) = (42, true);
let complex: ((Int, Int), Bool) = ((1, 2), false);
```

### Type Inference

Corrosion can often infer types automatically:

```rust
let x = 42;          // Inferred as Int
let flag = true;     // Inferred as Bool
let numbers = [1, 2, 3];  // Inferred as List Int
```

### Type Checking

The type checker ensures type safety:

```rust
let x = 42;
let y = true;
// let result = x + y;  // Error: Cannot add Int and Bool
```

## Output

### Print Statement

Use the `print` statement to display values to the console:

```rust
print(42);           // Prints: 42
print(true);         // Prints: true
print([1, 2, 3]);    // Prints: [1, 2, 3]
print((10, 20));     // Prints: (10, 20)
```

The `print` statement accepts any value type and displays it in a readable format. It always returns the unit value `()`.

### Printing Variables and Expressions

```rust
let x = 100;
print(x);            // Prints: 100
print(x + 50);       // Prints: 150

let numbers = [1, 2, 3];
print(head(numbers)); // Prints: 1
print(tail(numbers)); // Prints: [2, 3]
```

## String Operations

### String Literals

Strings are created using double quotes:

```rust
let message = "Hello, World!";
let name = "Corrosion";
print(message);  // Prints: "Hello, World!"
```

### String Escape Sequences

Strings support escape sequences for special characters:

```rust
let multiline = "Line 1\nLine 2";
let with_tab = "Column 1\tColumn 2";
let with_quote = "She said: \"Hello!\"";
let with_backslash = "Path: C:\\Users\\name";

print(multiline);
// Prints:
// "Line 1
// Line 2"
```

### String Concatenation

Combine strings using the `+` operator:

```rust
let greeting = "Hello";
let name = "World";
let message = greeting + ", " + name + "!";
print(message);  // Prints: "Hello, World!"
```

Or use the `concat` function:

```rust
let result = concat("Hello", " World");
print(result);  // Prints: "Hello World"
```

### String Operations

#### Getting String Length

```rust
let text = "Hello";
let len = length(text);
print(len);  // Prints: 5
```

#### Accessing Characters

```rust
let text = "Hello";
let first_char = char(text, 0);  // Get character at index 0
let last_char = char(text, 4);   // Get character at index 4

print(first_char);  // Prints: "H"
print(last_char);   // Prints: "o"
```

### Converting Other Types to Strings

Use the `toString` function to convert any value to its string representation:

```rust
// Convert numbers
let num_str = toString(42);
print("Number: " + num_str);  // Prints: "Number: 42"

// Convert booleans
let bool_str = toString(true);
print("Boolean: " + bool_str);  // Prints: "Boolean: true"

// Convert lists
let list_str = toString([1, 2, 3]);
print("List: " + list_str);  // Prints: "List: [1, 2, 3]"

// Convert pairs
let pair_str = toString((10, 20));
print("Pair: " + pair_str);  // Prints: "Pair: (10, 20)"
```

### Practical String Examples

#### Building Dynamic Messages

```rust
let user_name = "Alice";
let score = 95;
let passed = score >= 70;

let message = "User " + user_name + " scored " + toString(score) +
              " points and " + (if passed { "passed" } else { "failed" });

print(message);
// Prints: "User Alice scored 95 points and passed"
```

#### Processing Text Data

```rust
let words = ["Hello", "beautiful", "world"];

// Build a sentence by concatenating words
let sentence = words[0] + " " + words[1] + " " + words[2] + "!";
print(sentence);  // Prints: "Hello beautiful world!"

// Get information about each word
for word in words {
    let info = "Word: '" + word + "' has " + toString(length(word)) + " characters";
    print(info);
}
// Prints:
// "Word: 'Hello' has 5 characters"
// "Word: 'beautiful' has 9 characters"
// "Word: 'world' has 5 characters"
```

#### Working with Complex Data

```rust
let person = ("John Doe", 30);
let hobbies = ["reading", "cycling", "cooking"];

let profile = "Name: " + fst(person) +
              ", Age: " + toString(snd(person)) +
              ", Hobbies: " + toString(hobbies);

print(profile);
// Prints: "Name: John Doe, Age: 30, Hobbies: [reading, cycling, cooking]"
```

### String Type Checking

String operations are type-safe - you can only concatenate strings with strings:

```rust
// ✅ This works - both operands are strings
let result = "Hello" + " World";

// ❌ This would cause a type error - mixing string and int
// let error = "Hello" + 123;

// ✅ This works - convert int to string first
let correct = "Hello" + toString(123);
```

## Comments

### Single-Line Comments

Use `//` for single-line comments:

```rust
let x = 42;  // This is a comment
// This entire line is a comment
```

### Multi-Line Comments

Use `/* */` for multi-line comments:

```rust
/*
 * This is a multi-line comment
 * that spans several lines
 */
let x = 42;

let y = /* inline comment */ 10;
```

### Comment Best Practices

```rust
// Function to calculate the factorial of a number
let factorial = fn(n) {
    /*
     * Base case: factorial of 0 or 1 is 1
     * Recursive case: n * factorial(n-1)
     */
    // Implementation would go here
    n
};
```

## Error Handling

### Compile-Time Errors

Type errors are caught at compile time:

```rust
let x: Int = true;  // Type error: expected Int, found Bool
```

### Runtime Errors

Some errors occur at runtime:

```rust
let empty = [];
let first = head(empty);  // Runtime error: Cannot get head of empty list
```

### Error Messages

Corrosion provides clear error messages with source location:

```
Error: Runtime error at line 2, column 13: Cannot get head of empty list
```

## Advanced Topics

### Recursive Data Structures

While not explicitly covered in basic syntax, you can work with recursive structures using the existing types:

```rust
// Binary tree represented as nested pairs
// Left child, Right child, or empty list for leaf
let leaf = [];
let tree = (1, (leaf, leaf));  // Node with value 1 and no children
```

### Functional Programming Patterns

#### Map-like Operations with Loops

```rust
// Transform each element in a list using loops
let transform_with_loop = fn(f) {
    fn(list) {
        // Process each element
        for item in list {
            let transformed = f(item);
            print(transformed);
        };
    }
};

let double = fn(x) { x * 2 };
let doubler = transform_with_loop(double);
doubler([1, 2, 3]);  // Prints: 2, 4, 6
```

#### Recursive Functional Patterns

```rust
// Apply a function to each element using recursion
let recursive_map = fix(fn(self) {
    fn(func) {
        fn(list) {
            // Base case would check if list is empty
            // Recursive case would process head and recurse on tail
            // (More powerful with conditional expressions)
            list
        }
    }
});

// Fold-like operations with fixed point
let recursive_fold = fix(fn(self) {
    fn(combiner) {
        fn(initial) {
            fn(list) {
                // Would recursively combine elements
                initial
            }
        }
    }
});
```

#### Combining Loops and Recursion

```rust
// Use recursion to build higher-order loop patterns
let repeat_n_times = fix(fn(self) {
    fn(n) {
        fn(action) {
            for i in range(0, n) {
                action(i);
            };
        }
    }
});

let print_number = fn(x) { print(x); };
let print_5_times = repeat_n_times(5);
print_5_times(print_number);  // Prints: 0, 1, 2, 3, 4
```

#### Advanced Control Flow Patterns

```rust
// Nested iteration with recursion
let matrix_processor = fix(fn(self) {
    fn(processor) {
        fn(rows) {
            fn(cols) {
                for r in range(0, rows) {
                    for c in range(0, cols) {
                        let result = processor((r, c));
                        print(result);
                    };
                };
            }
        }
    }
});

let coordinate_sum = fn(coord) { fst(coord) + snd(coord) };
let process_3x3 = matrix_processor(coordinate_sum);
process_3x3(3)(3);  // Prints coordinate sums for 3x3 grid
```

### Working with Complex Data

#### Processing Lists of Pairs

```rust
let points = [(1, 2), (3, 4), (5, 6)];
let first_point = head(points);        // (1, 2)
let x_coord = fst(first_point);        // 1
let remaining_points = tail(points);   // [(3, 4), (5, 6)]
```

#### Nested Function Calls

```rust
let numbers = [1, 2, 3, 4, 5];
let without_first = tail(numbers);     // [2, 3, 4, 5]
let second_element = head(without_first);  // 2

// Or in one expression:
let second = head(tail(numbers));      // 2
```

### Performance Considerations

- **Immutability**: All data structures are immutable, which means operations create new structures
- **List Operations**: `cons` is O(1), `head` is O(1), `tail` is O(1)
- **Function Calls**: Functions capture their environment (closures)

### Common Patterns

#### Checking List Contents

```rust
let numbers = [1, 2, 3];
// Check if list is empty by trying operations that would error
// (In a full implementation, you'd have better ways to check)
```

#### Building Complex Structures

```rust
// Building a list of pairs
let pairs = [(1, 2), (3, 4)];
let new_pair = (5, 6);
let extended_pairs = cons(new_pair, pairs);  // [(5, 6), (1, 2), (3, 4)]
```

#### Function Composition

```rust
let add_one = fn(x) { x + 1 };
let multiply_two = fn(x) { x * 2 };

// Compose functions
let add_then_multiply = fn(x) { multiply_two(add_one(x)) };
let result = add_then_multiply(5);  // (5 + 1) * 2 = 12
```

## Next Steps

Now that you've learned the Corrosion language basics:

1. **Experiment**: Try the examples in the REPL
2. **Build Programs**: Create `.corr` files with more complex logic
3. **Explore**: Combine different features to build interesting programs
4. **Learn More**: Study functional programming concepts to get the most out of Corrosion

## Summary

Corrosion is a functional programming language with:

- **Static typing** with type inference
- **Immutable data structures** (lists, pairs)
- **First-class functions** with closures
- **Recursion support** via fixed point operator
- **Control flow** with for loops and range iteration
- **Pattern matching** through destructuring operations
- **Interactive development** via REPL

The language combines functional programming principles with practical control flow constructs, enabling both elegant recursive algorithms and efficient iterative processing. The fixed point operator allows for sophisticated recursive patterns, while for loops provide straightforward iteration over collections and ranges.

Key features for different programming styles:

- **Functional**: Use `fix` for recursion, higher-order functions, and function composition
- **Iterative**: Use `for` loops with `range` for processing sequences
- **Hybrid**: Combine both approaches for complex data processing patterns

The type system ensures safety across all these paradigms. Happy coding!
