/*
output-
*****
*****
*****
*****
*****
*/ 
fn main() {
    let rows = 5;
    let cols = 5;

    for _ in 0..rows {
        for _ in 0..cols {
            print!("*");
        }
        println!();
    }
}
