use std::io;

fn main() {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input);
    println!("input: {}", input);
}