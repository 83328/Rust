# Conjunctive Normal Form Project: Exercise 06 from "Ready, Set, Boole!"

This README covers **Exercise 06 - Conjunctive Normal Form**, converting an RPN Boolean expression with `A`, `B`, `C` and operators `&`, `|`, `^`, `!`, `>`, `=` into its CNF in RPN. It focuses on new concepts specific to this exercise.

## Project Overview

Convert an RPN Boolean expression to Conjunctive Normal Form (CNF) in RPN, returning a `String`. Invalid formulas return an empty string. The Conjunctive Normal Form (CNF) means that in the output, every negation must be located right after a variable and every conjunction must be located at the end of the formula.

- **Function Prototype**: `fn conjunctive_normal_form(formula: &str) -> String`
- **Algorithm**: Convert to NNF, then distribute `|` over `&` for CNF.
- **Complexity**: Time O(n * 2^k), Space O(n), where `n` is formula length, `k` is nested `&` count.

## Implementation
### Algorithm Explanation
- **CNF Conversion**:
  - `to_nnf`: Transform to NNF using stack-based RPN parsing.
  - `to_cnf`: Distribute `|` over `&` (e.g., `A | (B & C)` → `(A | B) & (A | C)`), extract literals, and join with operators.
- Example: `AB|C&!` → `A!B!&C!|` (infix: `!((A | B) & C)` → `(!A & !B) | !C`).
- **C Analogy**: Like C parser with tokenization for literals and operator joining.

## New Rust Concepts

- **Literal Extraction**: Use `peekable` iterator to parse literals (e.g., `A`, `A!`), like C’s `strtok` for tokenizing.
- **Joining Literals**: Combine `Vec<String>` with `join` and `repeat`, like C’s `strcat` for building expressions.
