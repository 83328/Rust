# Set Evaluation Project: Exercise 09 from "Ready, Set, Boole!"

This README covers **Exercise 09 - Set Evaluation**, evaluating an RPN Boolean expression with variables `A`–`Z` as sets and operators as set operations. It focuses on new concepts specific to this exercise.

## Project Overview

Evaluate an RPN Boolean expression with variables as sets of integers, returning the resulting set as `Vec<i32>`.

- **Function Prototype**: `fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32>`
- **Algorithm**: Parse RPN with a stack, apply set operations (union, intersection, complement, etc.) relative to the universal set.

## Implementation
### Algorithm Explanation

- **Set Evaluation**:
  - Compute universal set with `flat_map`.
  - Parse RPN, mapping variables to sets, applying operations (`!` → complement, `&` → intersection, etc.).
- Example: `AB&` with `[[0, 1, 2], [0, 3, 4]]` → `[0]`.
- **C Analogy**: Like C stack parser with arrays for set operations.

## New Rust Concepts

- **HashSet for Set Operations**: Use `HashSet<i32>` for efficient union, intersection, and complement, like C’s array-based set operations.
- **Flat Map**: Use `flat_map` to merge sets into universal set, like C’s loop to combine arrays.
