fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let idx = 5;

    // This will panic at runtime if idx is out of bounds
    let val = vec[idx];

    println!("Value at index {}: {}", idx, val);
}