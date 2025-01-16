
/*
different ways to print output on screen in rust

- Use format!() when you need to create a formatted string for later use.
    
- Use print!() or println!() for regular console output.
    
- Use eprint!() or eprintln!() for error messages or debugging output.
*/

fn main() {
    let name = "Ankit";
    let age = 25;

    // format! - returns a String
    let message = format!("Hello, {}! You are {} years old.", name, age);
    println!("{}", message); // Prints: Hello, Ankit! You are 25 years old.

    // print! - prints to stdout without a newline
    print!("Hello, "); // No newline
    print!("{}! ", name); // No newline
    println!("You are {} years old.", age); // Prints: Hello, Ankit! You are 25 years old.

    // eprint! - prints to stderr without a newline
    eprint!("Error: "); // No newline
    eprintln!("Something went wrong!"); // Prints: Error: Something went wrong!
}