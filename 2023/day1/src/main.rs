use std::fs;

fn main() {
    println!("Hello, world!");
    let content: String = fs::read_to_string("src/input.txt").expect("Could not read input file");
    println!("content {}", content);
}
