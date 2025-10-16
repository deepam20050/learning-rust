use proconio::input;

fn get_sum(mut x: i32) -> i32 {
    let mut s = 0;
    while x > 0 {
        s += x % 10;
        x /= 10;
    }
    s
}

fn main() {
    input! {
        n: i32,
    }

    let mut sum = 1;
    for _ in 1..n {
        let new_val = get_sum(sum);
        sum += new_val;
    }

    println!("{}", sum);
}
