use std::fmt::Debug;
use std::fs::File;
use std::hash::Hash;
use std::io::Write;

fn say_hello<W: Write>(out: &mut W) -> std::io::Result<()> {
    out.write_all(b"hello world\n")?;
    out.flush()
}

// multiple traits for a type
fn top_ten<T: Debug + Hash + Eq>(value: &Vec<T>) {
    println!("{:?}", value)
}

// fn run_query<M, R>(map: M, Reduce: R) -> ()
// where M: Mapper + Serialize,
//     R: Reducer + Serialize
// {
//     println!("done")
// }

fn main() -> std::io::Result<()> {
    let mut local_file: File = File::create("hello.txt")?;
    // here &mut local_file creates a temporary reference
    say_hello(&mut local_file)?;

    let mut local_file_2: File = File::create("new hello.txt")?;
    say_hello(&mut local_file_2)?;

    let mut bytes = vec![];
    say_hello(&mut bytes)?;
    assert_eq!(bytes, b"hello world\n");

    // let v1 = (0 .. 1000).collect(); // not possible to infer type
    let v2 = (0..1000).collect::<Vec<i64>>();

    Ok(())
}
