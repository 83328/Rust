fn negation_normal_form(formula: &str) -> String {
    let mut stack: Vec<(String, String, char)> = Vec::new();
    for ch in formula.chars() {
        match ch {
            'A' | 'B' | 'C' => stack.push((ch.to_string(), "".to_string(), '\0')),
            '&' | '|' => {
                if stack.len() < 2 { return err(formula, "Insufficient operands"); }
                let (v2, v2_op2, v2_op) = stack.pop().unwrap();
                let (v1, v1_op2, v1_op) = stack.pop().unwrap();
                
                // Reconstruct the full expressions
                let expr1 = if v1_op == '\0' { v1 } else { format!("{}{}{}", v1, v1_op2, v1_op) };
                let expr2 = if v2_op == '\0' { v2 } else { format!("{}{}{}", v2, v2_op2, v2_op) };
                
                stack.push((expr1, expr2, ch));
            }
            '^' => {
                if stack.len() < 2 { return err(formula, "Insufficient operands"); }
                let (v2, _, _) = stack.pop().unwrap();
                let (v1, _, _) = stack.pop().unwrap();
                stack.push((format!("{}!{}&{}!{}!&|", v1, v2, v1, v2), "".to_string(), '\0'));
            }
            '!' => {
                if stack.is_empty() { return err(formula, "Insufficient operands for '!'"); }
                let (v1, v2, op) = stack.pop().unwrap();
                if op == '\0' { 
                    stack.push((v1 + "!", "".to_string(), '\0')); 
                }
                else if op == '&' { 
                    // De Morgan's law: !(A & B) = !A | !B
                    let neg_v1 = negate(&v1);
                    let neg_v2 = negate(&v2);
                    stack.push((format!("{}{}|", neg_v1, neg_v2), "".to_string(), '\0')); 
                }
                else if op == '|' { 
                    // De Morgan's law: !(A | B) = !A & !B
                    let neg_v1 = negate(&v1);
                    let neg_v2 = negate(&v2);
                    stack.push((format!("{}{}&", neg_v1, neg_v2), "".to_string(), '\0')); 
                }
                else { return err(formula, "Invalid expression for '!'"); }
            }
            '>' => {
                if stack.len() < 2 { return err(formula, "Insufficient operands"); }
                let (v2, _, _) = stack.pop().unwrap();
                let (v1, _, _) = stack.pop().unwrap();
                stack.push((format!("{}!{}|", v1, v2), "".to_string(), '\0'));
            }
            '=' => {
                if stack.len() < 2 { return err(formula, "Insufficient operands"); }
                let (v2, _, _) = stack.pop().unwrap();
                let (v1, _, _) = stack.pop().unwrap();
                stack.push((format!("{}{}&{}!{}!&|", v1, v2, v1, v2), "".to_string(), '\0'));
            }
            _ => return err(formula, &format!("Invalid character '{}'", ch)),
        }
    }
    if stack.len() != 1 { 
        err(formula, "Invalid formula") 
    } else { 
        let (v1, v2, op) = stack.pop().unwrap();
        if op == '\0' { v1 } else { format!("{}{}{}", v1, v2, op) }
    }
}

fn negate(expr: &str) -> String {
    if expr.len() == 1 {
        return format!("{}!", expr);
    }
    
    // For simple expressions like "AB&" or "AB|"
    if expr.len() == 3 && expr.chars().nth(2).map_or(false, |c| c == '&' || c == '|') {
        let op = expr.chars().nth(2).unwrap();
        let a = &expr[0..1];
        let b = &expr[1..2];
        match op {
            '&' => format!("{}!{}!|", a, b),  // !(A & B) = !A | !B
            '|' => format!("{}!{}!&", a, b),  // !(A | B) = !A & !B
            _ => format!("{}!", expr)
        }
    } else {
        format!("{}!", expr)
    }
}

fn err(formula: &str, msg: &str) -> String {
    eprintln!("Error: {} in '{}'", msg, formula);
    String::new()
}

fn main() {
    ["AB&!", "AB|!", "AB>", "AB=", "AB|C&!"].iter().for_each(|&f| println!("{}", negation_normal_form(f)));
}