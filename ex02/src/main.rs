fn gray_code(n: u32) -> u32 {
    n ^ (n >> 1) // XOR with right-shifted n
}

fn main() {
    let tests = [
        (0, 0),   // 0b0 → 0b0
        (1, 1),   // 0b1 → 0b1
        (2, 3),   // 0b10 → 0b11
        (3, 2),   // 0b11 → 0b10
        (4, 6),   // 0b100 → 0b110
        (4294967295, 2147483648), // u32::MAX
    ];

    for (n, expected) in tests {
        let result = gray_code(n);
        println!("Gray({}) = {}, expected: {}", n, result, expected);
        assert_eq!(result, expected, "Test failed for {}", n);
    }
}