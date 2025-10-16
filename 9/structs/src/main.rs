struct GrayscaleMap {
    pixels: Vec<u8>,
    size: (usize, usize),
}

// structs are private by default
// visible to only module where they're declared
// and its submodules. To make a struct visible outside
// its module. Prefix it with `pub`
// pub struct GrayscaleMap {
//     pixels: Vec<u8>,
//     size: (usize, usize),
// }

fn new_map(size: (usize, usize), pixels: Vec<u8>) -> GrayscaleMap {
    assert_eq!(pixels.len(), size.0 * size.1);
    GrayscaleMap { pixels, size }
}

struct Broom {
    name: String,
    height: u32,
    health: u32,
    position: (f32, f32, f32),
    intent: BroomIntent,
}

#[derive(Clone, Copy)]
enum BroomIntent {
    FetchWater,
    DumpWater,
}

fn chop(b: Broom) -> (Broom, Broom) {
    let mut broom1 = Broom {
        height: b.height / 2,
        ..b
    };

    // String doesn't support Copy
    // String supports clone
    let mut broom2 = Broom {
        name: broom1.name.clone(),
        ..broom1
    };

    broom1.name.push_str(" I");
    broom2.name.push_str(" II");

    (broom1, broom2)
}

// this is a tuple like struct
struct Bounds(usize, usize);
/*
 * Individual elements of a tuple-like struct maybe public or not
 */

// newtypes - structs with a single component that you define
// to get a stricter type checking
struct Ascii(Vec<u8>);

// unit-like structs
// no memory allocation or machine code is generated
struct Onesuch;

pub struct Queue {
    older: Vec<char>,
    younger: Vec<char>,
}

impl Queue {
    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if (self.younger.is_empty()) {
                return None;
            }
            use std::mem::swap;
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }
        self.older.pop()
    }
}

impl Queue {
    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }
    
    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }
}

fn main() {
    let width = 1024;
    let height = 576;
    let image = GrayscaleMap {
        pixels: vec![0; width * height],
        size: (width, height),
    };
    assert_eq!(image.pixels.len(), width * height);
    assert_eq!(image.size.0 * image.size.1, width * height);

    let hokey = Broom {
        name: String::from("Hokey"),
        height: 60,
        health: 100,
        position: (100.0, 200.0, 0.0),
        intent: BroomIntent::FetchWater,
    };

    let (hokey1, hokey2) = chop(hokey);
    assert_eq!(hokey1.name, "Hokey I");
    assert_eq!(hokey2.name, "Hokey II");

    let image_bounds = Bounds(1024, 768);
    assert_eq!(image_bounds.0 * image_bounds.1, 1024 * 768);

    let o = Onesuch;

    let mut q = Queue {
        older: Vec::new(),
        younger: Vec::new(),
    };
    q.push('0');
    q.push('1');
    assert_eq!(q.pop(), Some('0'));

    q.push('x');
    assert_eq!(q.pop(), Some('1'));
    assert_eq!(q.pop(), Some('x'));
    assert_eq!(q.pop(), None);
    assert!(q.is_empty());
    {
        let mut q = Queue {
            older: Vec::new(),
            younger: Vec::new()
        };
        
        q.push('P');
        q.push('D');
        assert_eq!(q.pop(), Some('P'));
        q.push('X');
        
        let (older, younger) = q.split();
        // println!("{}", q.is_empty());
        // q cannot be used since it has been moved
        assert_eq!(older, vec!['D']);
        assert_eq!(younger, vec!['X']);
    }
}
