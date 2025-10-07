use core::f64;

fn main() {
    let x = b'A';
    let y = 65;
    println!("{}", x == y);

    assert_eq!(10_i8 as u16, 10u16);

    println!("{}", (-4i32).abs());

    assert_eq!(10_u8.checked_add(20), Some(30));

    // Converting from floating to integer results in truncation
    // assert_eq!(11.52_f32 as u32, 10u32);

    // rust doesn't implicitly convert u32 to u64
    // we'd need to explicitly mention any such conversion

    println!("{}", f64::consts::PI);
    
    // implicit conversions of strings/integers etc to bool
    // are not allowed in rust unlike C/C++/Python
    assert_eq!(false as u64, 0);
    assert_eq!(true as u64, 1);    

    // String represents its text as a sequence of UTF-8 bytes
    // not as an array of characters

    let text = "I see the eigenvalue in thine eye";
    let (head, tail) = text.split_at(21);
    assert_eq!(head, "I see the eigenvalue ");
    assert_eq!(tail, "in thine eye");

    // this is a single element tuple
    // the last `,` is necessary
    let single_element_tuple = ("lonely hearts", )

    let mut i: i32 = 1;
    loop {
        i = i.checked_mul(10).expect("Multiplication overflowed");
    }


}