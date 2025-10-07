// this only works if the underlying fields implement
// the Copy trait
#[derive(Copy, Clone)]
struct Label {
    number: u32,
}

/*
    I borrowed a reference and so was able to print
*/
fn print(l: Label) -> () {
    println!("STAMP: {}", l.number);
}

fn main() {
    let l = Label { number : 3};
    print(l);
    // I borrowed a reference so was able to print
    println!("My label number is: {}", l.number);
}
