fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    let n = set.len();
    let mut result = Vec::new();
    for i in 0..(1 << n) {
        let mut subset = Vec::new();
        for j in 0..n {
            if (i >> j) & 1 == 1 {
                subset.push(set[j]);
            }
        }
        result.push(subset);
    }
    result
}

fn main() {
    let test_cases = [
        vec![1, 2],
        vec![1],
        vec![],
        vec![1, 2, 3],
    ];
    for input in test_cases {
        let subsets = powerset(input.clone());
        let subset_str: Vec<String> = subsets.iter()
            .map(|subset| format!("[{}]", subset.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")))
            .collect();
        println!("{:?} -> [{}]", input, subset_str.join(", "));
    }
}