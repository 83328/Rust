# Gray Code Project: Exercise 02 from "Ready, Set, Boole!"

This README covers **Exercise 02 - Gray**, converting a 32-bit unsigned integer to its Gray code. It focuses on new concepts specific to this exercise, assuming familiarity with prior READMEs (Exercises 00 and 01).

## Project Overview

Implement a function to convert a `u32` number to its Gray code using bitwise operations.

- **Function Prototype**: `fn gray(n: u32) -> u32`
- **Allowed Operations**: Bitwise `&`, `|`, `^`, `<<`, `>>`, assignment (`=`), comparisons.
- **Algorithm**: Compute `n ^ (n >> 1)` to get the Gray code, where adjacent values differ by one bit.
- **Complexity**: Time O(1), Space O(1).

## What is Gray Code?

Gray code is a **binary numeral system** where two successive values differ in only one bit. This property makes it incredibly useful for reducing errors in digital systems.

### Normal Binary vs. Gray Code

**Normal Binary Counting:**
```
0: 000
1: 001  ← 1 bit different from 0
2: 010  ← 2 bits different from 1! 
3: 011  ← 1 bit different from 2
4: 100  ← 3 bits different from 3!
```

**Gray Code:**
```
0: 000
1: 001  ← 1 bit different from 0
2: 011  ← 1 bit different from 1
3: 010  ← 1 bit different from 2  
4: 110  ← 1 bit different from 3
```

**Every adjacent number differs by exactly one bit!**

## Real-World Applications

### 1. **Rotary Encoders**
Used in:
- **Car steering wheel position sensors**
- **Computer mouse scroll wheels**
- **Industrial machinery positioning**
- **Robotic joint encoders**

**Why Gray code?** When rotating from position 3 to 4:
- Normal binary: `011 → 100` (all 3 bits change)
- Gray code: `010 → 110` (only 1 bit changes)
- If electrical noise occurs during transition, Gray code prevents large position errors

### 2. **Digital Circuit Design**
- **Counters**: Prevents glitches when multiple bits would change simultaneously
- **State machines**: Safer transitions between states in FSMs
- **Clock domain crossing**: Reduces metastability issues in FPGA/ASIC design
- **Synchronous circuits**: Minimizes power consumption during transitions

### 3. **Analog-to-Digital Converters (ADCs)**
- **Medical equipment**: ECG, MRI scanners for precise measurements
- **Audio systems**: High-quality DACs/ADCs in studio equipment
- **Sensor interfaces**: Temperature, pressure, and other analog sensors
- **Oscilloscopes**: Converting analog waveforms to digital representation

### 4. **Error Correction and Detection**
- **File systems**: Some filesystems use Gray code for metadata integrity
- **Network protocols**: Error-correcting codes in data transmission
- **Memory systems**: ECC RAM and error detection in storage devices
- **Satellite communications**: Deep space communication systems

### 5. **Computer Graphics**
- **Dithering algorithms**: Creates smooth color gradients in images
- **Image compression**: JPEG and other formats use Gray code sequences
- **3D graphics**: Smoother color transitions in rendering pipelines
- **Display technology**: LCD and OLED pixel addressing

### 6. **Genetic Algorithms and AI**
- **Optimization problems**: Mutation operations are more gradual
- **Neural networks**: Some encoding schemes for genetic programming
- **Evolutionary computation**: Better exploration of solution spaces
- **Machine learning**: Feature encoding in some ML algorithms

### 7. **Historical and Legacy Systems**
- **Telegraph systems**: Frank Gray's original application at Bell Labs (1940s)
- **Early computers**: IBM and other manufacturers used Gray code counters
- **Mechanical calculators**: Reduced wear on mechanical switching systems
- **Television systems**: Early TV scanning and synchronization circuits

## Why This Matters for Programming

### Problem with Normal Binary
```rust
// Imagine reading a 3-bit sensor rapidly during transition
let position = read_sensor(); 

// Normal binary: 3 → 4 transition
// 011 → 100  (all bits change simultaneously)
// If read during transition: might get 000, 001, 010, 111, etc.
// This could cause huge position errors!
```

### Solution with Gray Code
```rust
// Gray code: 3 → 4 transition  
// 010 → 110  (only leftmost bit changes)
// If read during transition: worst case is off by 1 position
// Much safer and more predictable!
```

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

### Step-by-Step for `n = 4`:
```
n = 4        = 100₂
n >> 1       = 010₂  (right shift by 1)
n ^ (n >> 1) = 100₂ ^ 010₂ = 110₂ = 6
```

## Fun Fact: Tower of Hanoi Connection

The **Tower of Hanoi** puzzle naturally follows Gray code! The sequence of moves corresponds exactly to Gray code bit patterns. This isn't coincidence - both involve changing one thing at a time optimally.

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

- **Single Expression Function**: A function can be a single expression without braces if it's concise.
  - Example: `fn gray(n: u32) -> u32 { n ^ (n >> 1) }` omits `return` and braces.
- **Bitwise Operations**: Same as C, but with stronger type safety.

## Troubleshooting

- **Test Failures**: Verify Gray code values manually (e.g., `2 → 0b10 ^ 0b01 = 0b11 = 3`). Add `println!("n: {:b}, n>>1: {:b}, result: {:b}", n, n >> 1, result);` for binary output.
- **Bitwise Errors**: Ensure `^` and `>>` are used correctly, as in C.

## Modern Usage

Gray code is everywhere in engineering today:
- **Your computer mouse wheel** uses Gray code for smooth scrolling
- **Car fuel gauges** use Gray code sensors for accurate readings
- **GPS satellites** use Gray code in their communication protocols
- **Industrial robots** use Gray code encoders for precise positioning

It's a fundamental tool for making digital systems more reliable! 🔧

Run `cargo run` to test!