fn main() {
    let mut v = vec![1, 2, 3, 4, 5];
    for &x in &v {
        println!("{}", x);
    }
    for x in &v {
        println!("{}", x);
    }
    for x in &mut v {
        *x += 1;
        println!("==> {}", x);
    }
    println!("{}", v.len());
    println!("{}", v.len());

    /*
     * println!() borrows its arguments. It doesn't take ownership unless you
     * do something like this:
     */
    println!("{}", "hello !!".to_string()); // but this "hello" String is temporary so you wouldn't be able to use it anyway
}
