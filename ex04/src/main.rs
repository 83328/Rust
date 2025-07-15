// Evaluate an RPN Boolean expression with A, B, C (0 or 1) and operators &, |, ^
fn eval_formula(formula: &str, a: u32, b: u32, c: u32) -> Option<u32> {
    let mut stack: Vec<u32> = Vec::new();
    for ch in formula.chars() {
        match ch {
            'A' => stack.push(a),
            'B' => stack.push(b),
            'C' => stack.push(c),
            '&' | '|' | '^' => {
                if stack.len() < 2 {
                    eprintln!("Error: Insufficient operands in '{}'", formula);
                    return None;
                }
                let v2 = stack.pop().unwrap();
                let v1 = stack.pop().unwrap();
                stack.push(match ch {
                    '&' => v1 & v2,
                    '|' => v1 | v2,
                    '^' => v1 ^ v2,
                    _ => unreachable!(),
                });
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

// Print truth table for an RPN Boolean expression with A, B, C
fn print_truth_table(formula: &str) -> bool {
    println!("| A | B | C | = |");
    println!("|---|---|---|---|");
    for a in 0..=1 {
        for b in 0..=1 {
            for c in 0..=1 {
                match eval_formula(formula, a, b, c) {
                    Some(result) => println!("| {} | {} | {} | {} |", a, b, c, result),
                    None => return false,
                }
            }
        }
    }
    true
}

fn main() {
    let tests = [
        ("AB&C|", true), // (A & B) | C
    ];

    for (formula, expected) in tests {
        println!("Truth table for {}:", formula);
        let result = print_truth_table(formula);
        println!("Result: {}, expected: {}\n", result, expected);
        assert_eq!(result, expected, "Test failed for {}", formula);
    }
}