# Negation Normal Form Project: Exercise 05 from "Ready, Set, Boole!"

This README covers **Exercise 05 - Negation Normal Form**, converting an RPN Boolean expression with `A`, `B`, `C` and operators `&`, `|`, `^`, `!`, `>`, `=` into its NNF in RPN. It focuses on new concepts specific to this exercise.

## Project Overview

Convert an RPN Boolean expression to Negation Normal Form (NNF) in RPN, returning a `String`. Invalid formulas return an empty string with an error message.

- **Function Prototype**: `fn negation_normal_form(formula: &str) -> String`
- **Algorithm**: Stack-based RPN parsing, transforming operators to NNF with `&`, `|`, `!`.
- **Complexity**: Time O(n), Space O(n), where `n` is formula length.

## What is Negation Normal Form (NNF)?

**Negation Normal Form** is a standardized way to write Boolean expressions where:

1. **Negations (`!`) only appear directly on variables** - like `A!`, never on complex expressions like `AB&!`
2. **Only basic operators remain**: `&` (AND), `|` (OR), and `!` (NOT)
3. **Complex operators are eliminated**: No `>` (implication), `=` (equivalence), or `^` (XOR)

**Why is NNF useful?**
- **Automated reasoning**: Logic solvers work faster with standardized forms
- **Proof systems**: Easier to apply logical rules systematically  
- **Circuit design**: Simpler to convert to hardware gates
- **Algorithm optimization**: Many algorithms assume NNF input

**Example transformations:**
- `AB>` (A implies B) → `A!B|` (NOT A OR B)
- `AB&!` (NOT(A AND B)) → `A!B!|` (NOT A OR NOT B)
- `AB=` (A equals B) → `AB&A!B!&|` ((A AND B) OR (NOT A AND NOT B))

**Real-world analogy**: Like converting different currency formats to a standard (USD) - the value stays the same, but it's easier to work with.

## What are De Morgan's Laws?

**De Morgan's Laws** are fundamental rules for distributing negation over logical operators:

1. **First Law**: `!(A & B) = !A | !B`
   - "NOT(A AND B)" equals "NOT A OR NOT B"
   - If it's not true that both are true, then at least one is false

2. **Second Law**: `!(A | B) = !A & !B`  
   - "NOT(A OR B)" equals "NOT A AND NOT B"
   - If it's not true that at least one is true, then both are false

**Intuitive examples:**
- **First Law**: "It's not true that (it's raining AND sunny)" = "It's not raining OR it's not sunny"
- **Second Law**: "It's not true that (I'm hungry OR tired)" = "I'm not hungry AND I'm not tired"

**In programming:**
```rust
// Instead of: !(x > 0 && y > 0)
// Use:        x <= 0 || y <= 0

// Instead of: !(x == 5 || y == 3)  
// Use:        x != 5 && y != 3
```

**Why important**: These laws let us "push" negations down to individual variables, which is essential for NNF conversion.

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
