fn main() {
    let x = b'A';
    let y = 65;
    println!("{}", x == y);

    assert_eq!(10_i8 as u16, 10u16);

    println!("{}", (-4i32).abs());

    assert_eq!(10_u8.checked_add(20), Some(30));

    let mut i: i32 = 1;
    loop {
        i = i.checked_mul(10).expect("Multiplication overflowed");
    }

}