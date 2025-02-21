fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let idx = 5;

    // Safe way to access vector elements using get()
    match vec.get(idx) {
        Some(val) => println!("Value at index {}: {}", idx, val),
        None => println!("Index {} is out of bounds", idx),
    }
} 