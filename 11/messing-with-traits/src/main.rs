use std::fs::File;
use std::io::Write;

/*
 * &mut dyn Write :
 * a mutable reference to any value that implements the Write trait
 */
fn say_hello(out: &mut dyn Write) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

fn min<T: Ord>(value1: T, value2: T) -> T {
    if value1 <= value2 { value1 } else { value2 }
}

fn main() -> std::io::Result<()> {
    // Rust converts ordinary references into trait objects when needed
    let mut local_file: File = File::create("hello.txt")?;
    // here &mut local_file creates a temporary reference
    say_hello(&mut local_file)?;

    let mut bytes = vec![];
    say_hello(&mut bytes)?;
    assert_eq!(bytes, b"hello world\n");

    let value1: u64 = 1_u64 << 56;
    let value2: u64 = 1_u64 << 60;

    println!("{}", min(value1, value2));

    let mut buf: Vec<u8> = vec![];
    // let write: dyn Write = buf; -- doesn't work
    // variable of type Write (OutputStream in Java)
    // is actually a reference to any object that
    // implements OutputStream.
    // In rust, we must make references explicit
    let writer: &mut dyn Write = &mut buf;
    // no downcasting supported for trait object
    // &mut dyn Write to Vec<u8>

    Ok(())
}
