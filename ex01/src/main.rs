// Adder function from Exercise 00
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

// Multiplier function using Russian Peasant Algorithm
// Time: O(log b * log a), Space: O(1)
fn multiplier(a: u32, b: u32) -> u32 {
    let mut result = 0;
    let mut x = a;
    let mut y = b;
    while y > 0 {
        if y & 1 == 1 { // If y is odd, add x to result
            result = adder(result, x);
        }
        x = x << 1; // Double x
        y = y >> 1; // Halve y
    }
    result
}

fn main() {
    let tests = [
        (0, 0, 0),
        (2, 3, 6),
        (5, 4, 20),
        (0, 42, 0),
        (10, 0, 0),
        (4294967295, 2, 4294967294),
    ];

    for (a, b, expected) in tests {
        let result = multiplier(a, b);
        println!("{} * {} = {}, expected: {}", a, b, result, expected);
        assert_eq!(result, expected, "Test failed for {} * {}", a, b);
    }
}
