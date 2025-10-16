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

fn main() {
    // turbo fish lessgoo
    let mut q = Queue::<char>::new();
    assert!(q.is_empty());
}
