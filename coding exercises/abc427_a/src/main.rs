use proconio::input;

fn main() {
    input! {
        mut s: String,
    }
    let mut s_len = s.len();
    s_len /= 2;
    s.remove(s_len);
    println!("{}", s)
}
