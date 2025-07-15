fn map(x: u16, y: u16) -> f64 {
    // Interleave bits of x and y to form a 32-bit Z-order value
    let mut z: u32 = 0;
    for i in 0..16 {
        z |= ((x as u32 >> i) & 1) << (2 * i);     // x bit to even positions
        z |= ((y as u32 >> i) & 1) << (2 * i + 1); // y bit to odd positions
    }

    // Normalize to [0, 1] - divide by 2^32 - 1 to ensure max value maps to exactly 1.0
    (z as f64) / 4294967295.0  // 2^32 - 1
}

fn main() {
    let test_cases = [
        (0, 0),
        (1, 0),
        (0, 1),
        (1, 1),
        (65535, 65535),
    ];
    for (x, y) in test_cases {
        let result = map(x, y);
        println!("map({}, {}) -> {}", x, y, result);
    }
}