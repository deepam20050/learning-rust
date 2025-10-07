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

    println!("{}", f64::consts::PI);

    let mut i: i32 = 1;
    loop {
        i = i.checked_mul(10).expect("Multiplication overflowed");
    }

}