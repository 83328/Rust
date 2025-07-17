// Evaluate an RPN Boolean expression with A-Z variables and all operators
fn eval_formula(formula: &str, values: &[u32; 26]) -> Option<u32> {
    let mut stack: Vec<u32> = Vec::new();
    
    for ch in formula.chars() {
        match ch {
            'A'..='Z' => {
                let index = (ch as u8 - b'A') as usize;
                stack.push(values[index]);
            }
            '!' => {
                if stack.is_empty() {
                    eprintln!("Error: Insufficient operands for ! in '{}'", formula);
                    return None;
                }
                let v = stack.pop().unwrap();
                stack.push(if v == 0 { 1 } else { 0 }); // NOT operation
            }
            '&' | '|' | '^' | '>' | '=' => {
                if stack.len() < 2 {
                    eprintln!("Error: Insufficient operands for {} in '{}'", ch, formula);
                    return None;
                }
                let v2 = stack.pop().unwrap();
                let v1 = stack.pop().unwrap();
                let result = match ch {
                    '&' => v1 & v2,                    // AND
                    '|' => v1 | v2,                    // OR
                    '^' => v1 ^ v2,                    // XOR
                    '>' => if v1 == 0 { 1 } else { v2 }, // Material implication: !A | B
                    '=' => if v1 == v2 { 1 } else { 0 }, // Equivalence
                    _ => unreachable!(),
                };
                stack.push(result);
            }
            _ => {
                eprintln!("Error: Invalid character '{}' in '{}'", ch, formula);
                return None;
            }
        }
    }
    
    if stack.len() != 1 {
        eprintln!("Error: Invalid formula '{}'", formula);
        None
    } else {
        Some(stack[0])
    }
}

// Print truth table for an RPN Boolean expression
fn print_truth_table(formula: &str) -> bool {
    // Find which variables are used in the formula
    let mut used_vars = Vec::new();
    for ch in formula.chars() {
        if ch.is_ascii_uppercase() && !used_vars.contains(&ch) {
            used_vars.push(ch);
        }
    }
    used_vars.sort();
    
    if used_vars.is_empty() {
        return false;
    }
    
    // Print header
    print!("|");
    for var in &used_vars {
        print!(" {} |", var);
    }
    println!(" = |");
    
    print!("|");
    for _ in &used_vars {
        print!("---|");
    }
    println!("---|");
    
    // Generate all combinations
    let num_vars = used_vars.len();
    for i in 0..(1 << num_vars) {
        let mut values = [0u32; 26];
        
        // Set values for used variables
        for (j, &var) in used_vars.iter().enumerate() {
            let var_index = (var as u8 - b'A') as usize;
            values[var_index] = (i >> (num_vars - 1 - j)) & 1;
        }
        
        match eval_formula(formula, &values) {
            Some(result) => {
                print!("|");
                for &var in &used_vars {
                    let var_index = (var as u8 - b'A') as usize;
                    print!(" {} |", values[var_index]);
                }
                println!(" {} |", result);
            }
            None => return false,
        }
    }
    true
}

fn main() {
    let tests = [
        ("AB&", true),      // A & B
        ("AB|", true),      // A | B  
        ("AB^", true),      // A ^ B (XOR)
        ("A!", true),       // !A (NOT)
        ("AB>", true),      // A > B (implication)
        ("AB=", true),      // A = B (equivalence)
        ("ABC||", true),    // A | B | C
        ("AB&C|", true),    // (A & B) | C
    ];

    for (formula, expected) in tests {
        println!("Truth table for '{}':", formula);
        let result = print_truth_table(formula);
        println!("Result: {}, expected: {}\n", result, expected);
        assert_eq!(result, expected, "Test failed for {}", formula);
    }
}