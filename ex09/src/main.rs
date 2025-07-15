use std::collections::HashSet;

fn eval_set(formula: &str, sets: Vec<Vec<i32>>) -> Vec<i32> {
    let universal: HashSet<i32> = sets.iter().flat_map(|s| s.iter().copied()).collect();
    let mut stack: Vec<HashSet<i32>> = Vec::new();

    for ch in formula.chars() {
        match ch {
            'A'..='Z' => {
                let idx = (ch as u8 - b'A') as usize;
                if idx >= sets.len() {
                    eprintln!("Error: Insufficient sets for variable '{}'", ch);
                    return Vec::new();
                }
                stack.push(sets[idx].iter().copied().collect());
            }
            '!' => {
                if let Some(s) = stack.pop() {
                    stack.push(universal.difference(&s).copied().collect());
                } else {
                    eprintln!("Error: Insufficient operands for '!'");
                    return Vec::new();
                }
            }
            '&' => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a.intersection(&b).copied().collect());
                } else {
                    eprintln!("Error: Insufficient operands for '&'");
                    return Vec::new();
                }
            }
            '|' => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a.union(&b).copied().collect());
                } else {
                    eprintln!("Error: Insufficient operands for '|'");
                    return Vec::new();
                }
            }
            '^' => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    stack.push(a.symmetric_difference(&b).copied().collect());
                } else {
                    eprintln!("Error: Insufficient operands for '^'");
                    return Vec::new();
                }
            }
            '>' => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let not_a = universal.difference(&a).copied().collect::<HashSet<_>>();
                    stack.push(not_a.union(&b).copied().collect());
                } else {
                    eprintln!("Error: Insufficient operands for '>'");
                    return Vec::new();
                }
            }
            '=' => {
                if stack.len() >= 2 {
                    let b = stack.pop().unwrap();
                    let a = stack.pop().unwrap();
                    let a_and_b = a.intersection(&b).copied().collect::<HashSet<_>>();
                    let not_a = universal.difference(&a).copied().collect::<HashSet<_>>();
                    let not_b = universal.difference(&b).copied().collect::<HashSet<_>>();
                    let not_a_and_not_b = not_a.intersection(&not_b).copied().collect::<HashSet<_>>();
                    stack.push(a_and_b.union(&not_a_and_not_b).copied().collect());
                } else {
                    eprintln!("Error: Insufficient operands for '='");
                    return Vec::new();
                }
            }
            _ => {
                eprintln!("Error: Invalid character '{}'", ch);
                return Vec::new();
            }
        }
    }

    if stack.len() != 1 {
        eprintln!("Error: Invalid formula");
        Vec::new()
    } else {
        let mut result: Vec<i32> = stack.pop().unwrap().into_iter().collect();
        result.sort();
        result
    }
}

fn main() {
    let test_cases = [
        ("AB&", vec![vec![0, 1, 2], vec![0, 3, 4]]),
        ("AB|", vec![vec![0, 1, 2], vec![3, 4, 5]]),
        ("A!", vec![vec![0, 1, 2]]),
    ];
    for (formula, sets) in test_cases {
        let result = eval_set(formula, sets.clone());
        println!("{} with sets {:?} -> {:?}", formula, sets, result);
    }
}
