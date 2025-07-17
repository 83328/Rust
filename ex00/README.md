# Adder Project: Exercise 00 from "Ready, Set, Boole!"

This README guides a beginner with a C programming background through implementing **Exercise 00 - Adder** from the "Ready, Set, Boole!" document in Rust. It covers setting up a Rust project, writing and running the Adder function, and understanding key Rust concepts with comparisons to C.

## Project Overview

**Exercise 00 - Adder** requires writing a function to add two unsigned 32-bit integers using only bitwise operations (AND, OR, XOR, shifts) and basic control structures, without arithmetic operators like `+`. This mimics how hardware performs addition, similar to low-level C programming with bitwise operators.

- **Function Prototype**: `fn adder(a: u32, b: u32) -> u32`
- **Allowed Operations**:
  - Bitwise: `&` (AND), `|` (OR), `^` (XOR), `<<` (left shift), `>>` (right shift)
  - Assignment (`=`), comparison operators (`==`, `!=`, etc.)
  - Increment (`++`) only for loop indices (not used in this solution)
- **Deliverable**: A Rust function and a `main` function to test it, akin to a C test harness.
- **Repository**: Place this code in your git repository for peer evaluation.

## Prerequisites

You need Rust installed. Rust is a compiled language like C, but it uses **Cargo** (a build tool and package manager) instead of `gcc` or `make`.

1. **Install Rustup** (Rust's installer, similar to `apt` for C tools):
   - **Linux/macOS**:
     ```bash
     curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
     ```
   - **Windows**: Download and run `rustup-init.exe` from [rustup.rs](https://rustup.rs).
   - Follow prompts to install the stable toolchain.
2. **Verify Installation**:
   ```bash
   rustc --version
   cargo --version
   ```
   - Expect output like `rustc 1.81.0` and `cargo 1.81.0`. If not, check your PATH or reinstall.

## Setting Up the Project

1. **Create a Rust Project**:
   ```bash
   cargo new adder
   cd adder
   ```
   - This creates:
     - `src/main.rs`: Main Rust file (like `main.c`).
     - `Cargo.toml`: Project configuration (like a `Makefile`).
   - `src/main.rs` starts with a "Hello, World!" program:
     ```rust
     fn main() {
         println!("Hello, world!");
     }
     ```
   - `Cargo.toml` looks like:
     ```toml
     [package]
     name = "adder"
     version = "0.1.0"
     edition = "2021"
     ```

## Implementation

```rust
// Function to add two u32 numbers using bitwise operations, like `unsigned int adder(unsigned int a, unsigned int b)` in C
fn adder(a: u32, b: u32) -> u32 {
    // Declare mutable variables, like `unsigned int x = a` in C
    let mut x = a; // `mut` allows changes, unlike C's default mutable variables
    let mut y = b;
    // Loop until no carry, similar to a C while loop
    while y != 0 {
        let carry = (x & y) << 1; // Bitwise AND and left shift, same as C's `&` and `<<`
        x = x ^ y;                // Bitwise XOR for sum without carry, same as C's `^`
        y = carry;                // Update carry for next iteration
    }
    x // Last expression is returned, no `return x;` needed
}

// Main function to test the adder, like `int main(void)` in C
fn main() {
    // Array of test cases, like `struct { unsigned int a, b, expected; } tests[]` in C
    let tests = [
        (0, 0, 0),      // Zero case
        (1, 1, 2),      // Simple addition
        (5, 3, 8),      // Multiple bits
        (0, 42, 42),    // One zero
        (4294967295, 1, 0), // u32::MAX + 1, wraps around like in C
    ];

    // Iterate over test cases, like a C for loop
    for (a, b, expected) in diversify

System: in tests {
        let result = adder(a, b);
        // Print results, like `printf` in C
        println!("{} + {} = {}, expected: {}", a, b, result, expected);
        // Assert equality, like `assert` in C
        assert_eq!(result, expected, "Test failed for {} + {}", a, b);
    }
}
```

### How It Works
- **Bitwise Addition**:
  - `x ^ y` computes the sum without carry (e.g., `1 ^ 1 = 0`, with carry).
  - `(x & y) << 1` computes the carry, shifted left.
  - The loop repeats until no carry remains (`y == 0`).
- **C Comparison**:
  - Bitwise operators (`&`, `|`, `^`, `<<`, `>>`) are identical to C.
  - The algorithm mimics hardware addition, a concept familiar to low-level C programmers.
- **Testing**: The `main` function tests various cases, including edge cases like `u32::MAX`.

## Running the Code

1. **Build**:
   ```bash
   cargo build
   ```
   - Compiles the code, like `gcc main.c -o main`.
   - Output executable is in `target/debug/adder`.
2. **Run**:
   ```bash
   cargo run
   ```
   - Builds and runs, producing output like:
     ```
     0 + 0 = 0, expected: 0
     1 + 1 = 2, expected: 2
     5 + 3 = 8, expected: 8
     0 + 42 = 42, expected: 42
     4294967295 + 1 = 0, expected: 0
     ```
   - If `assert_eq!` fails, it crashes with an error, like C’s `assert`.
3. **Release Mode** (optional, for faster execution):
   ```bash
   cargo run --release
   ```

## Key Rust Concepts for C Programmers

Rust shares similarities with C but emphasizes safety and modern features. Here are the key terms used in the code, compared to C:

- **`fn`**: Function definition, like `int func()` in C.
  - Example: `fn adder(a: u32, b: u32) -> u32` vs. `unsigned int adder(unsigned int a, unsigned int b)`.
  - Return type follows `->`, unlike C’s preceding type.
- **`let`**: Variable declaration, like `int x` in C.
  - Immutable by default, unlike C’s mutable variables.
  - Example: `let x = a;` is like `unsigned int x = a;`.
- **`mut`**: Makes a variable mutable, like non-`const` in C.
  - Example: `let mut x = a;` allows `x` to change.
- **`u32`**: Unsigned 32-bit integer, like `unsigned int` in C (32-bit architecture).
  - Rust’s types are explicit (`i32`, `u64`, etc.), unlike C’s platform-dependent `int`.
- **`while`**: Same as C’s `while`.
  - Example: `while y != 0` is identical.
- **`for`**: Iterates over collections, more expressive than C’s `for`.
  - Example: `for (a, b, expected) in tests` unpacks tuples, unlike C’s index-based loops.
- **`println!`**: Macro for printing, like `printf`.
  - Example: `println!("{} + {}", a, b)` vs. `printf("%u + %u\n", a, b)`.
  - The `!` indicates a macro, unlike C’s functions.
- **`assert_eq!`**: Macro to check equality, like `assert(a == b)` in C.
  - Example: `assert_eq!(result, expected)` panics if false.
- **Implicit Return**: Last expression in a function is returned without `return`.
  - Example: `x` in `adder` is returned, unlike C’s `return x;`.
- **Tuples**: Like lightweight `struct` in C.
  - Example: `(0, 0, 0)` is a tuple, unpacked in `for (a, b, expected)`.
- **No Semicolon for Return**: Omit `;` for the last expression to return it.
  - Example: `x` has no `;`, but `x = x ^ y;` needs one.

### Rust vs. C Differences
- **Memory Safety**: Rust prevents C bugs like null pointers via ownership, not relevant here but good to know.
- **No Undefined Behavior**: Bitwise operations and overflow are safe (wraps for `u32`).
- **Cargo**: Simplifies building/running vs. manual `gcc` or `make`.
- **Macros**: `println!` and `assert_eq!` are more powerful than C’s `#define`.
- **Type Annotations**: Required for function parameters (e.g., `a: u32`), unlike C’s optional types.

## Troubleshooting

- **Cargo Not Found**: Ensure `~/.cargo/bin` is in your PATH.
- **Compile Errors**: Rust’s errors are detailed. Example: Forgetting `mut` causes “cannot assign to immutable variable.”
- **Test Failures**: Debug bitwise logic with small inputs (e.g., `1 + 1`). Use `println!` for debugging.
- **C Habits**: Avoid C-style memory management (`malloc`/`free`); Rust handles this automatically.
