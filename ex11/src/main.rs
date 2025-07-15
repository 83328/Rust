fn map(x: u16, y: u16) -> f64 {
    let mut z: u32 = 0;
    for i in 0..16 {
        z |= ((x as u32 >> i) & 1) << (2 * i);     // x bit to even positions
        z |= ((y as u32 >> i) & 1) << (2 * i + 1); // y bit to odd positions
    }
    // Consistent normalization: divide by 2^32 - 1
    (z as f64) / 4294967295.0
}

fn reverse_map(n: f64) -> (u16, u16) {
    if n < 0.0 || n > 1.0 {
        eprintln!("Error: Input {} out of range [0, 1]", n);
        return (0, 0);
    }

    // Convert n to 32-bit integer (consistent with map)
    let z = (n * 4294967295.0).round() as u32;

    // De-interleave bits to reconstruct x and y
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    for i in 0..16 {
        x |= (((z >> (2 * i)) & 1) as u16) << i;     // Even bits to x
        y |= (((z >> (2 * i + 1)) & 1) as u16) << i; // Odd bits to y
    }
    (x, y)
}

fn main() {
    // Test bijectivity with systematic examples
    let coords = [(0, 0), (1, 0), (0, 1), (1, 1), (65535, 65535), (12345, 6789)];
    
    println!("Testing bijectivity:");
    for (x, y) in coords {
        let n = map(x, y);
        let (x2, y2) = reverse_map(n);
        let success = x == x2 && y == y2;
        println!("({}, {}) -> {} -> ({}, {}) [{}]", 
                 x, y, n, x2, y2, if success { "✓" } else { "✗" });
    }
    
    // Test edge cases
    println!("\nTesting edge cases:");
    for n in [0.0, 1.0, -0.1, 1.1] {
        let (x, y) = reverse_map(n);
        println!("reverse_map({}) -> ({}, {})", n, x, y);
    }
}