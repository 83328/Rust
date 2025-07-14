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