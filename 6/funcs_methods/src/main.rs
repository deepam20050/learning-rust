fn main() {
    let mut numbers = Vec::<i32>::new(); // type-associated function call

    // ::<> is called turbofish

    let ramp = (0 .. 5).collect::<Vec<i32>>();

    // <T> doesn't work
    // we need to use ::<T> 
}
