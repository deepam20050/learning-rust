fn main() {
    /*
        There are 3 types of pointer types:
        [1] References
        [2] Boxes
        [3] Unsafe pointers
     */

    // &x (produces a reference to x) -> in rust lingo: borrows a reference to x
    // *x -> refer to the value that x points to
    println!("Hello, world!");

    /*
        &T  :   An immutable shared reference. You can have
                many shared references to a given value. But
                they are read-only. Modifying values is forbidden.


        &mut T  :   A mutable exclusive reference. You can read/modify
                    the underlying value it points to. But 
                    [1] As long as this reference exists, you may not
                        have any other refereces to that value.
                    [2] The only way to access the value is through the
                        mutable reference.
     */

    let t = (12, "eggs");
    let b = Box::new(t); // allocate a tuple in the heap
    // When b goes out of scope (unless moved or returned) the memory is freed


    /*
        Unsafe pointers aka raw pointers -> similar to C++ pointers 
        but Rust makes no effort in tracking where it points to

        You may only reference raw pointers in an unsafe Rust block
     */
}
