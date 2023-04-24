// Ownership and borrowing concept

fn main() {
    let s = String::from("hello world");
    let len = calculate_length(&s);
    println!("The length of '{}' is {}.", s, len);
}

fn calculate_length(s1: &String) -> usize {
    s1.len()
}
