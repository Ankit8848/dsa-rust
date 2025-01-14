// if you want to convert user input into a specific type.
// use parse method.

use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a number:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Parse the input string into an integer
    let number: i32 = input.trim().parse().expect("Please enter a valid number!");

    println!("You entered: {}", number);
}

// input.trim().parse(): Converts the trimmed string into the desired type.