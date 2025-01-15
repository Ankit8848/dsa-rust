/*
output-
1
12
123
1234
12345
*/
fn main() {
    let star = 5;

    for i in 1..=star {
        for j in 1..=i {
            print!("{}", j);
        }
        println!();
    }
}
