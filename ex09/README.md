# Set Evaluation Project: Exercise 09 from "Ready, Set, Boole!"

This README covers **Exercise 09 - Set Evaluation**, evaluating an RPN Boolean expression with variables `A`–`Z` as sets and operators as set operations. It focuses on new concepts specific to this exercise.

## Project Overview

Evaluate an RPN Boolean expression with variables as sets of integers, returning the resulting set as `Vec<i32>`.

- **Function Prototype**: `fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32>`
- **Algorithm**: Parse RPN with a stack, apply set operations (union, intersection, complement, etc.) relative to the universal set.

## Real-World Applications and Use Cases

**Set evaluation** extends Boolean logic to work with collections, making it incredibly powerful for practical applications:

### **Database and Query Systems**
- **SQL WHERE clauses**: `customers WHERE (age > 30) AND (city = 'Berlin')` becomes set intersection
- **Search engines**: Combine result sets from different search terms using set operations
- **Data filtering**: Apply complex filters across multiple datasets simultaneously

### **Machine Learning and Data Science**
- **Feature selection**: Find optimal feature combinations using set operations
- **Clustering validation**: Compare cluster memberships across different algorithms
- **A/B testing**: Analyze user segment overlaps and differences

### **Software Engineering**
- **Access control**: Combine user permissions, roles, and resource sets
- **Configuration management**: Merge and validate configuration sets across environments
- **Dependency resolution**: Solve package dependency conflicts using set algebra

### **Game Development and AI**
- **Pathfinding**: Combine traversable terrain sets with obstacle avoidance
- **AI decision making**: Evaluate action possibilities using set-based rules
- **Procedural generation**: Combine constraint sets for world building

### **Mathematical Modeling**
- **Probability theory**: Work with event spaces and sample sets
- **Graph algorithms**: Vertex and edge set manipulations
- **Optimization problems**: Constraint satisfaction over solution sets

**Key insight**: This exercise demonstrates how Boolean algebra generalizes beyond true/false to any collection-based problem where you need to combine, filter, or analyze groups of items systematically.

## Implementation
### Algorithm Explanation

- **Set Evaluation**:
  - Compute universal set with `flat_map`.
  - Parse RPN, mapping variables to sets, applying operations (`!` → complement, `&` → intersection, etc.).
- Example: `AB&` with `[[0, 1, 2], [0, 3, 4]]` → `[0]`.
- **C Analogy**: Like C stack parser with arrays for set operations.

## New Rust Concepts

- **HashSet for Set Operations**: Use `HashSet<i32>` for efficient union, intersection, and complement, like C's array-based set operations.
- **Flat Map**: Use `flat_map` to merge sets into universal set, like C's loop to combine arrays.
