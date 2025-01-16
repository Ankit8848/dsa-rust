//Rust also supports reading from and writing to files using the std::fs module.

// writing to a file
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = File::create("output.txt")?;
    file.write_all(b"Hello, file!")?;
    Ok(())
}

// reading from a file
/*
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> std::io::Result<()> {
    let file = File::open("input.txt")?;
    let reader = io::BufReader::new(file);

    for line in reader.lines() {
        println!("{}", line?);
    }

    Ok(())
}
*/
