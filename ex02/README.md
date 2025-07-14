# Gray Code Project: Exercise 02 from "Ready, Set, Boole!"

This README covers **Exercise 02 - Gray**, converting a 32-bit unsigned integer to its Gray code, for a C programmer new to Rust. It focuses on new concepts specific to this exercise, assuming familiarity with prior READMEs (Exercises 00 and 01).

## Project Overview

Implement a function to convert a `u32` number to its Gray code using bitwise operations.

- **Function Prototype**: `fn gray(n: u32) -> u32`
- **Allowed Operations**: Bitwise `&`, `|`, `^`, `<<`, `>>`, assignment (`=`), comparisons.
- **Algorithm**: Compute `n ^ (n >> 1)` to get the Gray code, where adjacent values differ by one bit.
- **Complexity**: Time O(1), Space O(1).

## Implementation

The `gray` function computes the Gray code by XORing the number with itself right-shifted by one bit. This ensures that each bit in the result differs from the corresponding bit in the original number.

```rust
// Convert n to its Gray code
fn gray(n: u32) -> u32 {
    n ^ (n >> 1) // XOR with right-shifted n
}

fn main() {
    let tests = [
        (0, 0),   // 0b0 → 0b0
        (1, 1),   // 0b1 → 0b1
        (2, 3),   // 0b10 → 0b11
        (3, 2),   // 0b11 → 0b10
        (4, 6),   // 0b100 → 0b110
        (4294967295, 2147483647), // u32::MAX
    ];

    for (n, expected) in tests {
        let result = gray(n);
        println!("Gray({}) = {}, expected: {}", n, result, expected);
        assert_eq!(result, expected, "Test failed for {}", n);
    }
}
```

### Algorithm Explanation
- **Gray Code**: `n ^ (n >> 1)` flips bits based on the right-shifted value, ensuring adjacent numbers differ by one bit.
- Example: `n = 3` (0b11):
  - `n >> 1 = 1` (0b01)
  - `3 ^ 1 = 0b11 ^ 0b01 = 0b10 = 2`
- **C Analogy**: Like `return n ^ (n >> 1);` in C.

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
     Gray(0) = 0, expected: 0
     Gray(1) = 1, expected: 1
     Gray(2) = 3, expected: 3
     Gray(3) = 2, expected: 2
     Gray(4) = 6, expected: 6
     Gray(4294967295) = 2147483647, expected: 2147483647
     ```

## New Rust Concepts

- **Single Expression Function**: A function can be a single expression without braces if it’s concise.
  - Example: `fn gray(n: u32) -> u32 { n ^ (n >> 1) }` omits `return` and braces.

## Troubleshooting

- **Test Failures**: Verify Gray code values manually (e.g., `2 → 0b10 ^ 0b01 = 0b11 = 3`). Add `println!("n: {:b}, n>>1: {:b}, result: {:b}", n, n >> 1, result);` for binary output.
- **Bitwise Errors**: Ensure `^` and `>>` are used correctly, as in C.

Run `cargo run` to test!