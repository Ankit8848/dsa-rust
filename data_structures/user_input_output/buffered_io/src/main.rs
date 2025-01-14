// For advanced I/O operations, Rust provides buffered readers and writers in the std::io module.

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut handle = stdin.lock(); // Buffered reader

    println!("Enter some text:");

    let mut input = String::new();
    handle.read_line(&mut input).expect("Failed to read line");

    println!("You entered: {}", input.trim());
}
// stdin.lock(): Creates a buffered reader for efficient input handling.
