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
- `type` - Type inspection
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

If a statement is missing a semicolon, a parse error will occur:

```rust
print(1)
```

Output:

```
"Error: Parse error: Unexpected token at line 4, column 8: Expected ';', found Eof"
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

The `print` statement can print any value type, including integers, booleans, lists, pairs, and functions.
If yo try to print nothing, it will throw a parse error:

```rust
print();
```

Output:

```
Error: Parse error: Unexpected token at line 1, column 7: expression, found RightParen
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

### Type expression

You can inspect the type of any expression using the `type` function.
It returns a string representation of the type:

```rust
print(type(42));              // Int
print(type(true));            // Bool
print(type([1, 2, 3]));       // List Int
print(type((10, 20)));        // (Int, Int)
print(type(fn(x) { x + 1 })); // Int -> Int
```

If you try to use `type` without an argument, it will throw a parse error:

```rust
type();
```

Output:

```
Error: Parse error: Unexpected token at line 1, column 6: expression, found RightParen
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

Using arithmetic operations for non-integer types will result in a type error:

```rust
let result = true + 5;  // Type error
```

Output:

```
Error: Type error: Invalid binary operation at line 1, column 14: 'Bool' Add 'Int'
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

If you try to compare incompatible types, it will result in a type error:

```rust
let result = 10 == true;  // Type error
```

Output:

```
Error: Type error: Invalid binary operation at line 1, column 14: 'Int' Equal 'Bool'
```

### Logical Operations

```rust
let and_op = true && false;   // Logical AND: false
let or_op = true || false;    // Logical OR: true
let not_op = !true;           // Logical NOT: false
```

If you try to use logical operations on non-boolean types, it will result in a type error:

```rust
let result = 10 && true;  // Type error
```

Output:

```
Error: Type error: Invalid binary operation at line 1, column 14: 'Int' LogicalAnd 'Bool'
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

Output:

```rust
6
11
Hello, Alice
Hello, Bob
```

If the parameter type is incorrect, it will result in a type error:

```rust
fn square(x: Int) {
    x * x
}
let result = square(true);  // Type error
```

Output:

```
Error: Type error: Type mismatch at line 4, column 14: expected 'Int', found 'Bool'
```

Corrosion can also infer parameter types when they are not specified, based on how the function is used.

```rust
let substract_one = fn(x) { x - 1 };  // Type inferred as Int -> Int
let result = substract_one(true);  // Type error
```

Output:

```
Error: Type error: Type mismatch at line 2, column 14: expected 'Int', found 'Bool'
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

Variables can not be used outside their scope:

```rust
let foo = fn(x: Int) {
    let y = x + 1;
    fn(z: Int) { y + z }  // 'y' is captured here
};

let bar = foo(5);
print(y) ;  // Error: 'y' is not defined in this scope
```

Output:

```
Error: Type error: Undefined variable 'y' at line 6, column 7
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

If you try to create a list with mixed types, it will result in a type error:

```rust
let mixed = [1, true, "string"];  // Type error
```

Output:

```
Error: Type error: Type mismatch at line 1, column 17: expected 'Int', found 'Bool'
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
print(type(coordinate));  // (Int, Int)
print(type(mixed_pair));  // (Int, Bool)
print(type(nested_pair)); // ((Int, Int), (Int, Int))
```

Output:

```rust
(Int, Int)
(Int, Bool)
((Int, Int), (Int, Int))
```

If you try to create a pair with more or fewer than two elements, it will result in a parse error:

```rust
let invalid_pair = (1, 2, 3);  // Parse error
```

Output:

```
Error: Parse error: Unexpected token at line 1, column 25: Expected ')' after pair, found Comma
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

If you try to `cons` an element of a different type than the list, it will result in a type error:

```rust
let numbers = [1, 2, 3];
let invalid = cons(true, numbers);  // Type error
```

Output:

```
Error: Type error: Type mismatch at line 2, column 20: expected 'Int', found 'Bool'
```

If you try to `cons` onto a non-list type, it will also result in a type error:

```rust
let not_a_list = 42;
let invalid = cons(1, not_a_list);  // Type error
```

Output:

```
Error: Type error: Type mismatch at line 2, column 23: expected 'List unknown', found 'Int'
```

### `head` - Getting the First Element

The `head` operation returns the first element of a list:

```rust
let numbers = [1, 2, 3];
let first = head(numbers);
print(first);  // Prints: 1
print(type(first));  // Prints: Int
```

Output:

```
1
Int
```

If you try to get the `head` of a non-list type, it will result in a type error:

```rust
let not_a_list = 42;
let invalid = head(not_a_list);  // Type error
```

Output:

```
Error: Type error: Type mismatch at line 2, column 15: expected 'List unknown', found 'Int'
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

```rust
let not_a_list = 42;
let invalid = tail(not_a_list);  // Type error
```

Output:

```
Error: Type error: Type mismatch at line 2, column 15: expected 'List unknown', found 'Int'
```

**Note**: `tail` on an empty list or non list-type will cause a runtime error.

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

Output:

```rust
5
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
print(type(x_coord));  // Prints: Int
```

Output:

```rust
10
Int
```

If you try to get the `fst` of a non-pair type, it will result in a type error:

```rust
let not_a_pair = 42;
let invalid = fst(not_a_pair);  // Type error
```

Output:

```
Error: Type error: Type mismatch at line 2, column 15: expected '(error, error)', found 'Int'
```

### `snd` - Getting the Second Element

```rust
let point = (10, 20);
let y_coord = snd(point);
print(y_coord);  // Prints: 20
print(type(y_coord));  // Prints: Int
```

Output:

```rust
20
Int
```

If you try to get the `snd` of a non-pair type, it will result in a type error:

```rust
let not_a_pair = 42;
let invalid = snd(not_a_pair);  // Type error
```

Output:

```
Error: Type error: Type mismatch at line 2, column 15: expected '(error, error)', found 'Int'
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

If the condition is not boolean, it will result in a type error:

```rust
let x = 5;
if x {
    print("This will cause a type error");
};
```

Output:

```
Error: Type error: Type mismatch at line 2, column 4: expected 'Bool', found 'Int'
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

The parameters must be integers, otherwise it will result in a type error:

```rust
range(true, false);  // Type error
```

Output:

```
Error: Type error: Type mismatch at line 1, column 1: expected 'Int', found 'Bool'
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

print(type(identity));  // Prints: FixedPoint
```

Output:

```rust
42
11
FixedPoint
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

If yout want to concatenate non-string types, convert them first using `toString`:

```rust
let number = 42;
let message = "The answer is: " + toString(number);
print(message);  // Prints: "The answer is: 42"
```

Output:

```rust
The answer is: 42
```

### String Operations

#### Getting String Length

```rust
let text = "Hello";
let len = length(text);
print(len);  // Prints: 5
```

If the argument is not a string, it will result in a type error:

```rust
let len = length(42);  // Type error
```

Output:

```
Error: Type error: Type mismatch at line 1, column 11: expected 'String', found 'Int'
```

#### Accessing Characters

```rust
let text = "Hello";
let first_char = char(text, 0);  // Get character at index 0
let last_char = char(text, 4);   // Get character at index 4

print(first_char);  // Prints: "H"
print(last_char);   // Prints: "o"
```

The paramaters must be a string and an integer index, otherwise it will result in a type error:

```rust
let invalid = char(42, true);  // Type error
```

Output:

```
Error: Type error: Type mismatch at line 1, column 15: expected 'String', found 'Int'
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
// This works, both operands are strings
let result = "Hello" + " World";

// This would cause a type error, mixing string and int
// let error = "Hello" + 123;

// This works, convert int to string first
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
