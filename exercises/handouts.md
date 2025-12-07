# Exercise Handouts

## Exercise 1: List Sum Calculator

Write a program that takes a list of integers as input and calculates the sum of all the integers in the list.
If the list is empty, the program should return 0.

**Skills practiced:** Recursion, list operations (`head`, `tail`), conditional expressions

Examples:

- Input: [1, 2, 3, 4, 5]
  Output: 15
- Input: []
  Output: 0

---

Example Solution found in `exercises/exercise-1.corr`

## Exercise 2: Higher-Order Functions - Map

Implement a `map` function that takes a function and a list, and applies the function to each element of the list, returning a new list with the transformed values.
As functions do not support multiple arguments directly, use currying.

Examples:

- Input: `map(fn(x) { x * 2 })([1, 2, 3, 4])`
  Output: `[2, 4, 6, 8]`
- Input: `map(fn(x) { x + 10 })([5, 15, 25])`
  Output: `[15, 25, 35]`
- Input: `map(fn(x) { x })([])`
  Output: `[]`

---

Example Solution found in `exercises/exercise-2.corr`

## Exercise 3: Pair Manipulation - Swap and Distance

Write two functions:

1. `swapPair`: Takes a pair `(a, b)` and returns a new pair `(b, a)` with elements swapped
2. `distance`: Takes a pair of integers `(x, y)` representing coordinates and calculates the Manhattan distance from origin (0, 0), which is `|x| + |y|`

**Skills practiced:** Pair operations (`fst`, `snd`), pair construction, arithmetic

Examples:

- `swapPair((1, 2))` → `(2, 1)`
- `swapPair(("hello", 42))` → `(42, "hello")`
- `distance((3, 4))` → `7`
- `distance((-5, 2))` → `7`

---

Example Solution found in `exercises/exercise-3.corr`

## Exercise 4: String Length Comparison

Write a function `longerString` that takes a pair of strings and returns the one that is longer. If they are the same length, return the first string.

**Skills practiced:** String operations (`length`), conditional expressions, function parameters

Examples:

- Input: `longerString(("hello", "world"))`
  Output: `"hello"` (both length 5, return first)
- Input: `longerString(("cat", "elephant"))`
  Output: `"elephant"`
- Input: `longerString(("programming", "code"))`
  Output: `"programming"`

---

Example Solution found in `exercises/exercise-4.corr`

## Exercise 5: List Filter and Sum of Evens

Implement two functions:

1. `filter`: Takes a predicate function and a list, returns a new list with only elements that satisfy the predicate
2. `sumEvens`: Uses your `filter` function to sum only the even numbers from a list

**Skills practiced:** Higher-order functions, predicates, function composition, recursion

Examples:

- `filter(fn(x) { x > 0 }, [-2, 3, -1, 4, 5])` → `[3, 4, 5]`
- `sumEvens([1, 2, 3, 4, 5, 6])` → `12` (2 + 4 + 6)
- `sumEvens([1, 3, 5])` → `0`

---

Sample Solution found in `exercises/exercise-5.corr`
