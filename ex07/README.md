# Satisfiability Project: Exercise 07 from "Ready, Set, Boole!"

This README covers **Exercise 07 - Satisfiability**, determining if an RPN Boolean expression with `A`, `B`, `C` and operators `&`, `|`, `^`, `!`, `>`, `=` is satisfiable. It focuses on new concepts specific to this exercise.

## Project Overview

Determine if an RPN Boolean expression is satisfiable, returning a `bool`. Invalid formulas return `false`.

- **Function Prototype**: `fn sat(formula: &str) -> bool`
- **Algorithm**: Extract variables, try all truth assignments (2^n), evaluate using RPN parsing.

## Implementation
### Algorithm Explanation
- **SAT Check**:
  - Collect unique variables with `HashSet`.
  - Generate 2^n truth assignments using bit shifts.
  - Evaluate RPN formula with `HashMap` for assignments.
- Example: `AA!&` → `false` (infix: `A & !A`, unsatisfiable).
- **C Analogy**: Like C array for variables, bit manipulation for assignments, and stack for expression evaluation.

## New Rust Concepts

- **HashSet and HashMap**: Use `HashSet` for unique variables and `HashMap` for truth assignments, like C’s hash table for key-value pairs.
- **Bit Manipulation**: Use `1 << n` and `>>` for generating truth combinations, like C’s bitwise operations.
