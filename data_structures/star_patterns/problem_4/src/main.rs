/*
output-
1
22
333
4444
55555
*/
fn main() {
    let star = 5;

    for i in 1..=star {
        for _ in 1..=i {
            print!("{}", i);
        }
        println!();
    }
}