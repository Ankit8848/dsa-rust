/*
output-
12345
1234
123
12
1
*/
fn main() {
    let rows = 5;

    for i in (1..=rows).rev() {
        for j in 1..=i {
            print!("{}", j);
        }
        println!();
    }
}
