# Multiplier Project: Exercise 01 from "Ready, Set, Boole!"

This README covers **Exercise 01 - Multiplier** for a C programmer new to Rust, focusing on new concepts and implementation details specific to this exercise. It assumes familiarity with the Exercise 00 README (e.g., Rust setup, `adder` function, basic keywords).

## Project Overview

Implement a function to multiply two unsigned 32-bit integers (`a * b`) using bitwise operations and the `adder` function from Exercise 00, without arithmetic operators like `*`.

- **Function Prototype**: `fn multiplier(a: u32, b: u32) -> u32`
- **Allowed Operations**: Bitwise `&`, `|`, `^`, `<<`, `>>`, assignment (`=`), comparisons (`==`, `!=`), and `adder`.
- **Algorithm**: Russian Peasant Algorithm, using bit shifts and additions.
- **Complexity**: Time O(log b * log a), Space O(1).

## Implementation
The `multiplier` function uses the Russian Peasant Algorithm, which repeatedly doubles one number and halves the other, adding to the result when the second number is odd. This mimics multiplication through repeated addition.

```rust
// Adder from Exercise 00
fn adder(a: u32, b: u32) -> u32 {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let carry = (x & y) << 1;
        x = x ^ y;
        y = carry;
    }
    x
}

// Multiplier using Russian Peasant Algorithm
fn multiplier(a: u32, b: u32) -> u32 {
    let mut result = 0;
    let mut x = a;
    let mut y = b;
    while y > 0 {
        if y & 1 == 1 { // Check if y is odd
            result = adder(result, x);
        }
        x = x << 1; // Double x
        y = y >> 1; // Halve y
    }
    result
}

fn main() {
    let tests = [
        (0, 0, 0),
        (2, 3, 6),
        (5, 4, 20),
        (0, 42, 0),
        (10, 0, 0),
        (4294967295, 2, 4294967294), // u32::MAX * 2
    ];

    for (a, b, expected) in tests {
        let result = multiplier(a, b);
        println!("{} * {} = {}, expected: {}", a, b, result, expected);
        assert_eq!(result, expected, "Test failed for {} * {}", a, b);
    }
}
```

### Algorithm Explanation
- **Russian Peasant Algorithm**:
  - Loop while `b > 0`:
    - If `b` is odd (`b & 1`), add `a` to result using `adder`.
    - Double `a` (`a << 1`), halve `b` (`b >> 1`).
  - Example: `2 * 3`:
    - Step 1: `b = 3`, odd, `result = 0 + 2 = 2`, `a = 4`, `b = 1`.
    - Step 2: `b = 1`, odd, `result = 2 + 4 = 6`, `a = 8`, `b = 0`.
    - Result: `6`.

## Running the Code

1. **Build**:
   ```bash
   cargo build
   ```
2. **Run**:
   ```bash
   cargo run
   ```
   - Output:
     ```
     0 * 0 = 0, expected: 0
     2 * 3 = 6, expected: 6
     5 * 4 = 20, expected: 20
     0 * 42 = 0, expected: 0
     10 * 0 = 0, expected: 0
     4294967295 * 2 = 4294967294, expected: 4294967294
     ```

## New Rust Concepts

- **`if`**: Conditional, identical to C’s `if`.
  - Example: `if y & 1 == 1` checks if `y` is odd, like `if (y % 2)` in C.
- **Bitwise Operations in Multiplication**:
  - `& 1`: Tests the least significant bit, like `y % 2` in C.
  - `<< 1`: Doubles a number, like `x * 2`.
  - `>> 1`: Halves a number, like `y / 2`.
- **Function Composition**: Calling `adder` within `multiplier`, similar to C function calls but with Rust’s safety guarantees.

## Troubleshooting

- **Test Failures**: Add `println!` in the loop to debug (e.g., `println!("result: {}, x: {}, y: {}", result, x, y);`).
- **Adder Issues**: Ensure `adder` from Exercise 00 is correct, as it’s critical.

## Submission

- Save in `src/main.rs` in your git repository.
- Comment complexity for peer review: Time O(log b * log a), Space O(1).


Run `cargo run` to test Multiplier!