# Negation Normal Form Project: Exercise 05 from "Ready, Set, Boole!"

This README covers **Exercise 05 - Negation Normal Form**, converting an RPN Boolean expression with `A`, `B`, `C` and operators `&`, `|`, `^`, `!`, `>`, `=` into its NNF in RPN. It focuses on new concepts specific to this exercise.

## Project Overview

Convert an RPN Boolean expression to Negation Normal Form (NNF) in RPN, returning a `String`. Invalid formulas return an empty string with an error message.

- **Function Prototype**: `fn negation_normal_form(formula: &str) -> String`
- **Algorithm**: Stack-based RPN parsing, transforming operators to NNF with `&`, `|`, `!`.
- **Complexity**: Time O(n), Space O(n), where `n` is formula length.

## Implementation
### Algorithm Explanation
- **NNF Conversion**: Parse RPN with stack, transform:
  - `!`: Use helper to transform `XY&` to `X!Y!|`, `XY|` to `X!Y!&`, variables to `X!`.
  - `>`: `A B >` → `A! B |`.
  - `=`: `A B =` → `A B & A! B! & |`.
  - `^`: `A B ^` → `A !B & !A B & |`.
  - `&`, `|`: Combine operands.
- Example: `AB|C&!` → `A!B!&C!|` (infix: `!((A | B) & C)` → `!A & !B | !C`).
- **C Analogy**: Like C stack parser with helper function for expression transformations.

## New Rust Concepts

- **Helper Function**: Use inner function (`nnf_expr`) for NNF transformations, like a C static function for parsing subexpressions.
