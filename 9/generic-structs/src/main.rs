pub struct Queue<T> {
    older: Vec<T>,
    younger: Vec<T>,
}

/*
 * for any type T, here are some associated functions
 * available on Queue<T>
 */
impl<T> Queue<T> {
    // `Self` is a special type parameter that refers to whatever
    // type we're adding methods to
    //
    // pub fn new() -> Queue<T> {
    //     Queue {
    //         older: Vec::new(),
    //         younger: Vec::new(),
    //     }
    // }

    // a refined version
    pub fn new() -> Self {
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }

    pub fn push(&mut self, t: T) {
        self.younger.push(t)
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
}

// structs with lifetime parameters
struct Extrema<'elt> {
    greatest: &'elt i32,
    least: &'elt i32,
}

fn find_extrema<'s>(slice: &'s [i32]) -> Extrema<'s> {
    let mut greatest = &slice[0];
    let mut least = &slice[0];

    for i in 1..slice.len() {
        if slice[i] < *least {
            least = &slice[i];
        }
        if (slice[i] > *greatest) {
            greatest = &slice[i];
        }
    }

    Extrema { greatest, least }
}

fn main() {
    // turbo fish lessgoo
    let mut q = Queue::<char>::new();
    assert!(q.is_empty());

    let a = [0, -3, 0, 15, 48];
    let e = find_extrema(&a);
    assert_eq!(*e.least, -3);
    assert_eq!(*e.greatest, 48);
}
