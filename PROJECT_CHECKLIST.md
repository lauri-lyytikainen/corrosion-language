# 🛠️ CS-C2170 Project Checklist (4000p Cap)

This checklist organizes the requirements and points from the project handout. The total standard points are 4000, with an additional 1800 possible bonus points (B).

---

## 1. Getting Started (Total: 200p + 0B)

- [x] **Pre-requisite:** This section is a pre-requisite for getting points from every other section.
- [x] The documentation is a Markdown or PDF file (e.g., `README.md` or `README.pdf`).

### 1.1 Language Introduction (100p)

- [x] A high-level overview of the language (e.g., is it functional/imperative, and what kind of features it has).

### 1.2 Running the Project (100p)

- [x] Clear instructions for compiling and running the project on a common operating system.

---

## 2. Tutorial and Exercises (Total: 900p + 0B)

- [x] A Markdown or PDF file with code examples and programming exercises (e.g., `TUTORIAL.md`).

### 2.1 Tutorial/Tour (700p)

- [x] A set of working code examples which cover all language features.
- [x] Examples can be run by copying and pasting them into the REPL or a source code file.
- [x] **Documentation Note:** The tutorial must cover all of the implemented features and attempt to cover all code paths (including failure cases).

### 2.2 Exercises (200p)

- [ ] Problem statements, handouts, and solution code for at least five programming exercises.
- [ ] Each programming exercise focuses on a different feature of the language.
- [ ] The exercises must be different from the tutorial examples.

---

## 3. Core Language: Commands and Operations (Total: 1200p + 800B)

### 3.1 Variables (200p)

- [x] Can store values in variables (using e.g. `let`) (100p).
- [x] Can access values stored in variables in the right scope (100p).

### 3.2 Integers and Booleans (150p)

- [x] Integer and boolean literals (50p).
- [x] Conditional logic, e.g., `if-then-else` (50p).
- [x] Basic arithmetic and comparison operations (50p).

### 3.3 Functions (200p)

- [x] Can define implicit functions (lambda abstractions) (100p).
- [x] Can apply functions to terms (100p).

### 3.4 Pairs (100p)

- [x] Can construct pairs (50p).
- [x] Can destructure pairs into first and second elements (50p).

### 3.5 Lists/Arrays (150p)

- [x] Can construct and combine lists/arrays (100p).
- [x] Can access array elements or destructure a list into head and tail (50p).

### 3.6 Sums (100p)

- [x] Can construct sums (with e.g. `inl`, `inr`) (50p).
- [x] Can pattern match sums (50p).

### 3.7 Recursion (100p)

- [x] Can recursively apply functions using e.g. a fixed-point operator (100p).

### 3.8 Declarations, Modules, and Imports (200p)

- [x] Can declare named functions or constants, which can be referred to from other declarations (100p).
- [x] Can import code from another file (100p).

### 3.9 Bonus: Output/Printing (+100 B)

- [x] Printing supports basic data types (50p).
- [x] Printing supports all possible values (including functions) (50p).

### 3.10 Bonus: Loops (+100 B)

- [x] Can create `for` or `while` loops (100p).

### 3.11 Bonus: Tuples (+100 B)

- [ ] Can construct tuples with literals (50p).
- [ ] Can access or destructure tuple elements (50p).

### 3.12 Bonus: Strings and Characters (+100 B)

- [x] Can construct and combine strings (50p).
- [x] Can access individual characters from strings (50p).

### 3.13 Bonus: Records (+100 B)

- [ ] Can construct records with literals (50p).
- [ ] Can access or destructure record fields (50p).

### 3.14 Bonus: Custom/Algebraic Data Types (+300 B)

- [ ] Can construct n-ary sums/custom types (100p).
- [ ] Can pattern match n-ary sums/custom types (200p).

---

## 4. Core Language: Type System (Total: 800p + 400B)

- [x] The type system should cover all terms (expressions) of the language.
- [x] Documentation/tour exhaustively covers all code paths done by the type checker.

### 4.1 Static Type Checking (200p)

- [x] A static type checker runs before the code is evaluated/executed (100p).
- [x] Every valid term in the language has a type (100p).

### 4.2 Integers and Booleans (100p)

- [x] Integers and booleans have types (50p).
- [x] Integer and boolean operations are typed correctly (25p).
- [x] Type checker rejects invalid terms (where types are mismatched) (25p).

### 4.3 Functions (150p)

- [x] Function types express both domain and codomain (50p).
- [x] Function definitions (lambdas) are type checked (25p).
- [x] Type checker rejects invalid function definitions (25p).
- [x] Function applications are type checked (25p).
- [x] Type checker rejects invalid function applications (25p).

### 4.4 Pairs (100p)

- [x] Pairs have types which express both element types (50p).
- [x] Projection operators (first and second) are typed correctly (25p).
- [x] Type checker rejects invalid terms (25p).

### 4.5 Lists/Arrays (100p)

- [x] Lists/arrays have types which express the element type (50p).
- [x] List/array operations are typed correctly (25p).
- [x] Type checker rejects invalid terms (25p).

### 4.6 Sums (100p)

- [x] Sums have types which express both variant types (50p).
- [x] Pattern matching on sum terms is typed correctly (25p).
- [ ] Type checker rejects invalid terms (25p).

### 4.7 Recursion (50p)

- [x] Recursion is typed correctly (25p).
- [x] Type checker rejects invalid terms (25p).

### 4.8 Bonus: Tuples (+100 B)

- [ ] Tuples have types which express the types of each element (50p).
- [ ] Tuple operators are typed correctly (25p).
- [ ] Type checker rejects invalid terms (25p).

### 4.9 Bonus: Records (+100 B)

- [ ] Records have types which express the types of each field (50p).
- [ ] Record operators are typed correctly (25p).
- [ ] Type checker rejects invalid terms (25p).

### 4.10 Bonus: Custom/Algebraic Data Types (+200 B)

- [ ] ADTs define their own types (50p).
- [ ] ADT constructors have correct types (50p).
- [ ] Pattern matching on the terms of an ADT is typed correctly (50p).
- [ ] Type checker rejects invalid terms (50p).

---

## 5. Working Interpreter (Total: 900p + 0B)

### 5.1 REPL (600p)

- [x] Evaluation works when code is given as a literal.
- [x] Supports basic computational features (doesn't have to support function declarations, loops, or other multi-line language constructs).
- [x] If there's a type checker, the code is type checked before being evaluated.

### 5.2 Can run code from a file (100p)

- [x] Evaluates the whole file as a term, or evaluates a specific function declaration such as `main`.

### 5.3 Can start a REPL from a file (100p)

- [x] All declarations from the file are available in the REPL.

### 5.4 Informative error messages (100p)

- [x] Error messages provide useful information about the issue (50p).
- [x] Source code line and column information (50p).

---

## 6. Bonus (Total: 0p + 600B)

### 6.1 Comments (+100 B)

- [x] Supports single-line comments (50p).
- [x] Supports multi-line comments (50p).

### 6.2 Polymorphism (+200 B)

- [x] Able to define polymorphic identity function (e.g., `id: a -> a`) (100p).
- [x] Type checking/unification works correctly (e.g., list type inference).

### 6.3 Code Generation (+300 B)

- [ ] The language can be compiled to another language (300p).
