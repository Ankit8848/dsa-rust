/*
output-
*****
****
***
**
*
*/
fn main() {
    let rows = 5;

    for i in (1..=rows).rev() {
        for _ in 0..i {
            print!("*");
        }
        println!();
    }
}
