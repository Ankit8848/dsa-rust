// std::io module provides to read user input. 
// The std::io::stdin() function provides access to std input.

use std::io;

fn main() {
    let mut input = String::new(); // Create a mutable string to store input

    println!("Enter your name:");

    // Read input from the user
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    println!("Hey, {}!", input.trim()); // Trim whitespace and print
}