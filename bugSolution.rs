fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);

    match vec.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }
} 