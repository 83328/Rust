use std::collections::{HashMap, HashSet};

fn sat(formula: &str) -> bool {
    // Extract all unique variables from the formula
    let variables: HashSet<char> = formula
        .chars()
        .filter(|&c| c.is_alphabetic())
        .collect();
    
    let var_list: Vec<char> = variables.into_iter().collect();
    let num_vars = var_list.len();
    
    // If no variables, evaluate the formula directly
    if num_vars == 0 {
        return eval_formula(formula, &HashMap::new());
    }
    
    // Try all possible combinations of truth values (2^n combinations)
    for i in 0..(1 << num_vars) {
        let mut assignment = HashMap::new();
        
        // Generate truth assignment for this iteration
        for (j, &var) in var_list.iter().enumerate() {
            assignment.insert(var, (i >> j) & 1 == 1);
        }
        
        // Evaluate formula with this assignment
        if eval_formula(formula, &assignment) {
            return true;
        }
    }
    
    false
}

fn eval_formula(formula: &str, assignment: &HashMap<char, bool>) -> bool {
    let mut stack: Vec<bool> = Vec::new();

    for ch in formula.chars() {
        match ch {
            'A'..='Z' => {
                // Get the value from assignment, default to false if not found
                let value = assignment.get(&ch).copied().unwrap_or(false);
                stack.push(value);
            }
            '!' => {
                if let Some(val) = stack.pop() {
                    stack.push(!val);
                } else {
                    return false; // Invalid formula
                }
            }
            '&' => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a && b);
                } else {
                    return false; // Invalid formula
                }
            }
            '|' => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a || b);
                } else {
                    return false; // Invalid formula
                }
            }
            '^' => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a ^ b); // XOR
                } else {
                    return false; // Invalid formula
                }
            }
            '>' => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(!a || b); // Implication: A -> B = !A | B
                } else {
                    return false; // Invalid formula
                }
            }
            '=' => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a == b); // Equivalence
                } else {
                    return false; // Invalid formula
                }
            }
            _ => {
                // Ignore whitespace and other characters
                if !ch.is_whitespace() {
                    eprintln!("Warning: Unknown character '{}' in formula", ch);
                }
            }
        }
    }
    
    // Should have exactly one value left on the stack
    if stack.len() == 1 {
        stack[0]
    } else {
        false // Invalid formula
    }
}

fn main() {
    // Test cases from the subject
    println!("{}", sat("AB|"));   // Expected: true
    println!("{}", sat("AB&"));   // Expected: true  
    println!("{}", sat("AA!&"));  // Expected: false
    println!("{}", sat("AA^"));   // Expected: false
}
