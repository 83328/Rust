# Powerset Project: Exercise 08 from "Ready, Set, Boole!"

This README covers **Exercise 08 - Powerset**, generating the powerset of a set of integers. It focuses on new concepts specific to this exercise.

## Project Overview

Generate all possible subsets of a `Vec<i32>` with no duplicates, returning a `Vec<Vec<i32>>`.

- **Function Prototype**: `fn powerset(set: Vec<i32>) -> Vec<Vec<i32>>`
- **Algorithm**: Use bit manipulation to generate 2^n subsets.

## Implementation
### Algorithm Explanation

- **Powerset Generation**: Iterate 0 to 2^n, use bitmask to select elements for each subset.
- Example: `vec![1, 2]` → `[[], [1], [2], [1, 2]]`.
- **C Analogy**: Like C function using bitmask to populate dynamic array of arrays.

## New Rust Concepts

- **Dynamic Vector Allocation**: Use `Vec<Vec<i32>>` for nested subsets, like C’s dynamically allocated 2D array.
- **Bitwise Subset Generation**: Use `1 << n` and `>>` to generate subsets, like C’s bit operations.
