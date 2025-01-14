// Handle parse input Errors with match or if let

use std::io;

fn main() {
    let mut input = String::new();

    println!("Enter a number:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    // Handle parsing errors gracefully
    match input.trim().parse::<i32>() {
        Ok(number) => println!("You entered: {}", number),
        Err(_) => println!("That's not a valid number!"),
    }
}