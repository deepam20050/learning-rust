use proconio::input;
use proconio::marker::{Chars};

fn main() {
    input! {
        s: Chars
    }

    // let mut ss = s;
    // ss.sort();

    // println!("{}", 
    //     if ss[0] == ss[1] {
    //         ss[ss.len() - 1]
    //     } else {
    //         ss[0]
    //     });

    let mut counts = [0; 26];
    for c in s {
        counts[(c as u8 - b'a') as usize] += 1;
    }

    for index in 0..counts.len() {
        if counts[index] == 1 {
            let c = char::from_u32(index as u32 + 97);
            match c {
                Some(ch) => println!("{}", ch),
                None => eprintln!("{} Invalid", index)
            }
        }
    }

}
