// Evaluate a Boolean expression in Reverse Polish Notation (RPN)
// with 0, 1 and operators &, |, >, =
fn eval_formula(formula: &str) -> bool {
    let mut stack: Vec<bool> = Vec::new();
    
    for ch in formula.chars() {
        match ch {
            '0' => stack.push(false),
            '1' => stack.push(true),
            '&' => {
                if stack.len() < 2 {
                    eprintln!("Error: Not enough operands for & in '{}'", formula);
                    return false;
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a && b);
            }
            '|' => {
                if stack.len() < 2 {
                    eprintln!("Error: Not enough operands for | in '{}'", formula);
                    return false;
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(a || b);
            }
            '>' => {
                if stack.len() < 2 {
                    eprintln!("Error: Not enough operands for > in '{}'", formula);
                    return false;
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                // Material implication: A > B is equivalent to !A | B
                stack.push(!a || b);
            }
            '=' => {
                if stack.len() < 2 {
                    eprintln!("Error: Not enough operands for = in '{}'", formula);
                    return false;
                }
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                // Equivalence: A = B is true when both have the same value
                stack.push(a == b);
            }
            ' ' => {
                // Skip whitespace
                continue;
            }
            _ => {
                eprintln!("Error: Invalid character '{}' in '{}'", ch, formula);
                return false;
            }
        }
    }
    
    if stack.len() != 1 {
        eprintln!("Error: Invalid formula '{}' - stack has {} elements", formula, stack.len());
        return false;
    }
    
    stack[0]
}

fn main() {
    let tests = [
        ("10&", false),    // 1 AND 0 = false
        ("10|", true),     // 1 OR 0 = true  
        ("11>", true),     // 1 > 1 = !1 OR 1 = false OR true = true
        ("10=", false),    // 1 = 0 = false (not equivalent)
        ("1011||=", true), // 1 OR 0 = true, 1 OR 1 = true, true = true = true
    ];

    for (formula, expected) in tests {
        let result = eval_formula(formula);
        println!("eval_formula(\"{}\") = {}, expected: {}", formula, result, expected);
        assert_eq!(result, expected, "Test failed for {}", formula);
    }
    
    println!("All tests passed!");
}