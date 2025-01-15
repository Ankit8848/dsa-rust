/*
output-
*
**
***
****
*****
*/
fn main() {
    let star = 5;

    for i in 0..=star {
        for _ in 0..i{
            print!("*");
        }
        println!();
    }
}