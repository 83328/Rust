## Key Rust Concepts for C Programmers

Rust shares similarities with C but emphasizes safety and modern features. Here are the key terms used in the code, compared to C:

- **`fn`**: Function definition, like `int func()` in C.
  - Example: `fn adder(a: u32, b: u32) -> u32` vs. `unsigned int adder(unsigned int a, unsigned int b)`.
  - Return type follows `->`, unlike C's preceding type.
- **`let`**: Variable declaration, like `int x` in C.
  - Immutable by default, unlike C's mutable variables.
  - Example: `let x = a;` is like `unsigned int x = a;`.
- **`mut`**: Makes a variable mutable, like non-`const` in C.
  - Example: `let mut x = a;` allows `x` to change.
- **`u32`**: Unsigned 32-bit integer, like `unsigned int` in C (32-bit architecture).
  - Rust's types are explicit (`i32`, `u64`, etc.), unlike C's platform-dependent `int`.
- **`while`**: Same as C's `while`.
  - Example: `while y != 0` is identical.
- **`for`**: Iterates over collections, more expressive than C's `for`.
  - Example: `for (a, b, expected) in tests` unpacks tuples, unlike C's index-based loops.
- **`println!`**: Macro for printing, like `printf`.
  - Example: `println!("{} + {}", a, b)` vs. `printf("%u + %u\n", a, b)`.
  - The `!` indicates a macro, unlike C's functions.
- **`assert_eq!`**: Macro to check equality, like `assert(a == b)` in C.
  - Example: `assert_eq!(result, expected)` panics if false.
- **Implicit Return**: Last expression in a function is returned without `return`.
  - Example: `x` in `adder` is returned, unlike C's `return x;`.
- **Tuples**: Like lightweight `struct` in C.
  - Example: `(0, 0, 0)` is a tuple, unpacked in `for (a, b, expected)`.
- **No Semicolon for Return**: Omit `;` for the last expression to return it.
  - Example: `x` has no `;`, but `x = x ^ y;` needs one.
- **`match`**: Pattern matching, like enhanced `switch` in C.
  - Example: `match ch { 'A' => ..., '&' => ..., _ => ... }` is more powerful than C's `switch`.
- **`Vec<T>`**: Dynamic array, like `malloc`'d arrays in C but memory-safe.
  - Example: `Vec<bool>` grows automatically, unlike fixed C arrays.
- **`String`**: Heap-allocated string, like `char*` with automatic memory management.
  - Example: `String::new()` vs. manual `malloc` and `strcpy` in C.
- **`&str`**: String slice, like `const char*` in C but with length tracking.
  - Example: Function parameters use `&str` for read-only string access.
- **`Option<T>`**: Nullable type, safer than C's `NULL` pointers.
  - Example: `Option<char>` can be `Some('A')` or `None`, preventing null dereference.
- **`Result<T, E>`**: Error handling, better than C's error codes.
  - Example: `Result<i32, String>` forces error handling, unlike C's ignored return values.

### Extended Rust vs. C Comparison

| **Aspect** | **Rust** | **C** |
|------------|----------|-------|
| **Memory Safety** | ✅ Automatic (ownership system) | ❌ Manual (`malloc`/`free`, prone to leaks) |
| **Null Pointer Safety** | ✅ `Option<T>` prevents null dereference | ❌ Segfaults from `NULL` access |
| **Buffer Overflows** | ✅ Bounds checking (runtime in debug) | ❌ Manual bounds checking required |
| **Type Safety** | ✅ Strong static typing | ⚠️ Weaker (implicit conversions, `void*`) |
| **Concurrency** | ✅ Data race prevention at compile time | ❌ Manual synchronization (error-prone) |
| **Error Handling** | ✅ `Result<T, E>` forces handling | ❌ Easy to ignore return codes |
| **Package Management** | ✅ Built-in Cargo | ❌ Manual dependency management |
| **Build System** | ✅ Cargo handles everything | ❌ Complex Makefiles, linking issues |
| **Cross-compilation** | ✅ Simple with Cargo | ❌ Complex toolchain setup |
| **Documentation** | ✅ Built-in `cargo doc` | ❌ External tools (Doxygen, etc.) |
| **Testing** | ✅ Built-in `cargo test` | ❌ External frameworks (CUnit, etc.) |
| **Performance** | ✅ Zero-cost abstractions | ✅ Direct hardware control |
| **Runtime** | ✅ No garbage collector | ✅ No runtime overhead |
| **Compile Time** | ❌ Slower compilation | ✅ Fast compilation |
| **Learning Curve** | ❌ Steep (ownership, lifetimes) | ✅ Simpler concepts initially |
| **Legacy Code** | ❌ Cannot directly use C libraries | ✅ Vast ecosystem, easy interop |
| **Embedded Systems** | ⚠️ Growing support (`no_std`) | ✅ Universal embedded support |
| **Industry Adoption** | ⚠️ Growing (Mozilla, Dropbox, Discord) | ✅ Ubiquitous (OS, drivers, embedded) |
| **Debugging** | ✅ Better error messages | ❌ Cryptic segfaults, undefined behavior |
| **Refactoring** | ✅ Compiler catches breaking changes | ❌ Runtime errors after changes |

### When to Choose Rust vs. C

**Choose Rust when:**
- Building new projects where safety is important
- Working with complex data structures and algorithms
- Need memory safety without garbage collection
- Want modern tooling (package manager, testing, docs)
- Building web services, CLI tools, or system utilities
- Team development where compile-time checks prevent bugs

**Choose C when:**
- Working on existing C codebases
- Embedded systems with strict constraints
- Need maximum performance control
- Interfacing with hardware or drivers
- Working in environments where Rust toolchain isn't available
- Quick prototypes or simple systems programming

### Rust vs. C Differences in Practice

**Memory Management:**
```rust
// Rust: automatic memory management
let mut vec = Vec::new();
vec.push(42);
// Memory automatically freed when vec goes out of scope
```
```c
// C: manual memory management
int* arr = malloc(sizeof(int) * 10);
arr[0] = 42;
free(arr);  // Must remember to free!
```

**Error Handling:**
```rust
// Rust: forced error handling
let result = std::fs::read_to_string("file.txt");
match result {
    Ok(content) => println!("{}", content),
    Err(e) => eprintln!("Error: {}", e),
}
```
```c
// C: easy to ignore errors
FILE* f = fopen("file.txt", "r");
if (f == NULL) {  // Often forgotten!
    // Handle error
}
```

**String Handling:**
```rust
// Rust: safe string operations
let mut s = String::from("Hello");
s.push_str(" World");  // No buffer overflow possible
```
```c
// C: manual buffer management
char s[100];
strcpy(s, "Hello");    // Potential buffer overflow
strcat(s, " World");   // Must ensure buffer is large enough
```

### For This Project (Boolean Algebra)

Rust's advantages for this specific project:
- **Pattern matching** makes parsing boolean expressions cleaner
- **Memory safety** prevents stack corruption in recursive algorithms
- **Strong typing** catches logical errors at compile time
- **Built-in testing** simplifies verification of boolean logic
- **No undefined behavior** ensures bitwise operations work predictably

The mathematical nature of boolean algebra makes Rust's safety guarantees particularly valuable - you can focus on the logic rather than worrying about memory bugs.