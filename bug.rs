fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let third = vec[2]; // This line will panic at runtime
    println!("The third element is {}", third);
}