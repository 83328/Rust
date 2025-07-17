# Truth Table Project: Exercise 04 from "Ready, Set, Boole!"

This README covers **Exercise 04 - Truth Tables**, printing a truth table for an RPN Boolean expression with three variables (`A`, `B`, `C`).

## Project Overview

Generate a truth table for an RPN Boolean expression with `A`, `B`, `C`, returning `true` if successful, `false` for invalid formulas with an error message.

A truth table lists all combinations of variable values and their corresponding expression results. Every line stands for a unique combination of `A`, `B`, and `C` values, showing how the expression evaluates.

- **Function Prototype**: `fn print_truth_table(formula: &str) -> bool`
- **Algorithm**: Evaluate RPN formula using a stack, print table for all `A`, `B`, `C` combinations (8 rows).
- **Complexity**: Time O(4n), Space O(n), where `n` is the formula length.

### Algorithm Explanation
- **RPN Evaluation**: Use a `Vec<u32>` stack to process postfix notation:
  - Push values for `A`, `B`, `C`.
  - For operators (`&`, `|`, `^`), pop two values, apply operation, push result.
- **Truth Table**: Iterate over `A`, `B`, `C` ∈ `{0,1}`, print `| A | B | C | = |` with `0`/`1` results.
- Example: `AB&C|` → `(A & B) | C`, outputs `1` when `C = 1` or `A = 1`, `B = 1`.
- **C Analogy**: Like a C stack-based RPN parser with `printf` for table output.

## New Rust Concepts

- **RPN Parsing**: Stack-based evaluation for postfix notation, processing variables and operators sequentially, similar to a C array-based stack.

