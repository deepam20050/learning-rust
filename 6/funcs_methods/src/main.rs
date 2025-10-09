fn main() {
    let mut numbers = Vec::<i32>::new(); // type-associated function call

    // ::<> is called turbofish

    let ramp = (0 .. 5).collect::<Vec<i32>>();

    // <T> doesn't work
    // we need to use ::<T> 

    let a: i32 = 1;
    let b: i32 = 10;

    let rangeA = a..b; // end exclusive
    let rangeB = a..=b; // end inclusive
}