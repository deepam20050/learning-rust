fn main() {
    println!("Hello, world!");
    let m: u64 = 10;
    let n: u64 = 20;
    println!("{}", gcd(n, m));
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}