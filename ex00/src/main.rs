// Welcome to Rust
// - To start a new project use "cargo new <project_name>"
// - To run the project use "cargo run"

fn adder(a: u32, b: u32) -> u32 {
    let mut x = a;
    let mut y = b;
    while y != 0 {
        let carry = (x & y) << 1;
        x = x ^ y;
        y = carry;
    }
    x
}

fn main() {
    let tests = [
        (0, 0, 0),
        (1, 1, 2),
        (5, 3, 8),
        (0, 42, 42),
        (4294967295, 1, 0), // u32::MAX + 1
    ];

    for (a, b, expected) in tests {
        let result = adder(a, b);
        println!("{} + {} = {}, expected: {}", a, b, result, expected);
        assert_eq!(result, expected, "Test failed for {} + {}", a, b);
    }
}