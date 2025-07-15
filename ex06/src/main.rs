fn conjunctive_normal_form(formula: &str) -> String {
    to_cnf(&to_nnf(formula))
}

fn to_nnf(formula: &str) -> String {
    let mut stack: Vec<(String, String, char)> = Vec::new();
    
    for ch in formula.chars() {
        match ch {
            'A'..='Z' => stack.push((ch.to_string(), String::new(), '\0')),
            '&' | '|' => {
                if stack.len() >= 2 {
                    let (v2, v2_op2, v2_op) = stack.pop().unwrap();
                    let (v1, v1_op2, v1_op) = stack.pop().unwrap();
                    
                    let expr1 = if v1_op == '\0' { v1 } else { format!("{}{}{}", v1, v1_op2, v1_op) };
                    let expr2 = if v2_op == '\0' { v2 } else { format!("{}{}{}", v2, v2_op2, v2_op) };
                    
                    stack.push((expr1, expr2, ch));
                }
            }
            '!' => {
                if let Some((v1, v2, op)) = stack.pop() {
                    match op {
                        '\0' => stack.push((v1 + "!", String::new(), '\0')),
                        '&' => stack.push((format!("{}{}|", negate(&v1), negate(&v2)), String::new(), '\0')),
                        '|' => stack.push((format!("{}{}&", negate(&v1), negate(&v2)), String::new(), '\0')),
                        _ => {}
                    }
                }
            }
            '>' => if stack.len() >= 2 {
                let (v2, _, _) = stack.pop().unwrap();
                let (v1, _, _) = stack.pop().unwrap();
                stack.push((format!("{}!{}|", v1, v2), String::new(), '\0'));
            }
            '=' => if stack.len() >= 2 {
                let (v2, _, _) = stack.pop().unwrap();
                let (v1, _, _) = stack.pop().unwrap();
                stack.push((format!("{}{}&{}!{}!&|", v1, v2, v1, v2), String::new(), '\0'));
            }
            '^' => if stack.len() >= 2 {
                let (v2, _, _) = stack.pop().unwrap();
                let (v1, _, _) = stack.pop().unwrap();
                stack.push((format!("{}!{}&{}!{}!&|", v1, v2, v1, v2), String::new(), '\0'));
            }
            _ => {}
        }
    }
    
    if let Some((v1, v2, op)) = stack.pop() {
        if op == '\0' { v1 } else { format!("{}{}{}", v1, v2, op) }
    } else { String::new() }
}

fn to_cnf(formula: &str) -> String {
    let mut stack: Vec<String> = Vec::new();
    
    for ch in formula.chars() {
        match ch {
            'A'..='Z' => stack.push(ch.to_string()),
            '!' => if let Some(var) = stack.pop() { stack.push(format!("{}!", var)); }
            '&' => if stack.len() >= 2 {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(if !a.contains('|') && !b.contains('|') {
                    join_with(&[get_literals(&a), get_literals(&b)].concat(), '&')
                } else { format!("{}{}&", a, b) });
            }
            '|' => if stack.len() >= 2 {
                let b = stack.pop().unwrap();
                let a = stack.pop().unwrap();
                stack.push(if !a.contains('&') && !b.contains('&') {
                    join_with(&[get_literals(&a), get_literals(&b)].concat(), '|')
                } else { format!("{}{}|", a, b) });
            }
            _ => {}
        }
    }
    
    stack.pop().unwrap_or_default()
}

fn negate(expr: &str) -> String {
    if expr.len() <= 2 {
        format!("{}!", expr.chars().next().unwrap())
    } else {
        let (a, b, op) = (&expr[0..1], &expr[1..2], expr.chars().last().unwrap());
        match op {
            '&' => format!("{}!{}!|", a, b),
            '|' => format!("{}!{}!&", a, b),
            _ => format!("{}!", expr.chars().next().unwrap()),
        }
    }
}

fn get_literals(expr: &str) -> Vec<String> {
    let mut literals = Vec::new();
    let mut chars = expr.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch.is_alphabetic() {
            let mut literal = ch.to_string();
            if chars.peek() == Some(&'!') {
                literal.push(chars.next().unwrap());
            }
            literals.push(literal);
        }
    }
    literals
}

fn join_with(items: &[String], op: char) -> String {
    if items.len() == 1 {
        items[0].clone()
    } else {
        items.join("") + &op.to_string().repeat(items.len() - 1)
    }
}

fn main() {
    for (input, expected) in [
        ("AB&!", "A!B!|"),
        ("AB|!", "A!B!&"),
        ("AB|C&", "AB|C&"),
        ("AB|C|D|", "ABCD|||"),
        ("AB&C&D&", "ABCD&&&"),
        ("AB&!C!|", "A!B!C!||"),
        ("AB|!C!&", "A!B!C!&&"),
    ] {
        let result = conjunctive_normal_form(input);
        println!("{} -> {} (expected: {})", input, result, expected);
    }
}
