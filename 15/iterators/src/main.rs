fn triangle(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..=n {
        sum += i;
    }
    sum
}

fn triangle_fold(n: i32) -> i32 {
    (1..=n).fold(0, |sum, item| sum + item)
}

// use std::iter::Iterator;

// trait Iterator {
//     type Item;
//     fn next(&mut self) -> Option<Self::Item>;
// }

fn main() {
    println!("{}", triangle(10_000));
    println!("{}", triangle_fold(10_000));
}
