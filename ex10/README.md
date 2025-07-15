# Curve Project: Exercise 10 from "Ready, Set, Boole!"

This README covers **Exercise 10 - Curve**, mapping 2D coordinates to a unique value in `[0, 1]` using a space-filling curve. It focuses on new concepts specific to this exercise.

## Project Overview

Map coordinates `(x, y)` in `[0, 2^16 - 1]^2` to a unique `f64` in `[0, 1]` using a bijective function.

- **Function Prototype**: `fn map(x: u16, y: u16) -> f64`
- **Algorithm**: Interleave bits of `x` and `y` (Z-order curve), normalize to `[0, 1]`.
- **Complexity**: Time O(1), Space O(1).

## What is a Space-Filling Curve?

A **space-filling curve** is a mathematical concept that maps multi-dimensional space (like a 2D plane) into a single dimension (like a line). Think of it as a way to "flatten" a 2D grid into a 1D sequence while preserving spatial relationships.

### Why Do We Need This?

Imagine you have a 2D grid of points and you want to:
- Store them in a 1D array
- Process them in an order that keeps nearby points close together
- Convert 2D coordinates into a single number for indexing

A naive approach would be **row-major order**: `(0,0), (0,1), (0,2), ..., (1,0), (1,1), ...`
But this breaks spatial locality - `(0,2)` and `(1,0)` are far apart in the sequence but close in 2D space!

## The Z-Order Curve (Morton Order)

The **Z-Order curve** is one of the most popular space-filling curves because it's:
- **Simple to implement** (just bit interleaving)
- **Fast to compute** (O(1) time)
- **Preserves locality** reasonably well

### How It Works

The Z-Order curve gets its name because it traces a "Z" pattern when you look at how it visits points:

```
2D Grid (4x4):           Z-Order traversal:
+---+---+---+---+        +---+---+---+---+
| 0 | 1 | 4 | 5 |        | 0→| 1 | 4→| 5 |
+---+---+---+---+         ↓   ↗   ↓   ↗
| 2 | 3 | 6 | 7 |        | 2 | 3 | 6 | 7 |
+---+---+---+---+         ↓     ↗ ↓     ↗
| 8 | 9 |12 |13 |        | 8→| 9 |12→|13 |
+---+---+---+---+         ↓   ↗   ↓   ↗
|10 |11 |14 |15 |        |10 |11 |14 |15 |
+---+---+---+---+
```

### The Bit Interleaving Trick

The magic happens through **bit interleaving**:

1. Take coordinates `x = 3` (binary: `11`) and `y = 2` (binary: `10`)
2. Interleave their bits: `x₁y₁x₀y₀` = `1110` = 14
3. This gives us the Z-order index!

**Example with 4-bit numbers:**
```
x = 3 = 0011₂
y = 2 = 0010₂

Interleaving:
Position: ...y₃x₃y₂x₂y₁x₁y₀x₀
x bits:   ...  0  0  1  1
y bits:   ...0  0  1  0
Result:   00011010₂ = 26
```

### Why Choose Z-Order?

1. **Locality Preservation**: Points close in 2D space tend to be close in the Z-order sequence
2. **Simplicity**: Just bit manipulation - no complex math
3. **Efficiency**: O(1) computation, no lookup tables needed
4. **Bijectivity**: Every (x,y) maps to exactly one Z-value and vice versa
5. **Well-studied**: Used in databases, graphics, and spatial indexing

### Alternative Curves

Other space-filling curves exist:
- **Hilbert Curve**: Better locality preservation but more complex to compute
- **Peano Curve**: Simpler but worse locality
- **Gray Code**: Good for certain applications but not general-purpose

We chose Z-Order because it strikes the perfect balance between simplicity and performance for this exercise.

## Implementation

### Algorithm Explanation
- **Z-Order Curve**: Interleave bits of `x` and `y` into a 32-bit value, divide by `2^32 - 1` for `[0, 1]`.
- Example: `map(1, 0)` → `4/4294967295 ≈ 9.31e-10`, `map(65535, 65535)` → `1.0`.

### Step-by-Step Process
1. **Input**: Two 16-bit coordinates `(x, y)`
2. **Bit Interleaving**: Create 32-bit Z-order value by interleaving bits
3. **Normalization**: Divide by `2^32 - 1` to get value in `[0, 1]`
4. **Output**: Unique `f64` value representing the spatial position

### Code Walkthrough
```rust
for i in 0..16 {
    z |= ((x as u32 >> i) & 1) << (2 * i);     // x bit to even positions (0,2,4,...)
    z |= ((y as u32 >> i) & 1) << (2 * i + 1); // y bit to odd positions (1,3,5,...)
}
```

This loop extracts each bit from `x` and `y` and places them in alternating positions in the result.

## New Rust Concepts

- **Bit Interleaving**: Use shifts (`>>`, `<<`) and bitwise operations (`&`, `|`) to manipulate individual bits
- **Type Casting**: Convert between `u16`, `u32`, and `f64` for different operations
- **f64 Precision**: Handle floating-point normalization to ensure exact `[0, 1]` range

## Real-World Applications

Z-Order curves are used in:
- **Database indexing** (spatial databases like PostGIS)
- **Computer graphics** (texture mapping, Z-buffer algorithms)
- **Geographic Information Systems** (GIS)
- **Game development** (spatial partitioning, quadtrees)
- **Image processing** (Morton-order pixel storage)
