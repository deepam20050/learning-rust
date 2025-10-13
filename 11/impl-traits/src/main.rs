// a dummy canvas
struct Canvas {
    area: Box<Vec<Vec<i32>>>,
}

trait Visible {
    fn draw(&self, canvas: &mut Canvas);

    fn hit_test(&self, x: i32, y: i32) -> bool;
}

// these are subtraits
trait Game: Visible {
    fn foo(&self);
}

struct Boo;

impl Game for Boo {
    fn foo(&self) {
        println!("executing foo")
    }
}

impl Visible for Boo {
    fn draw(&self, canvas: &mut Canvas) {
        println!("executing draw")
    }
    fn hit_test(&self, x: i32, y: i32) -> bool {
        println!("executing hit_test");
        return true;
    }
}

struct Broom {
    name: String,
}

// to implement a trait
// this only contains implementations of methods
// for Visible trait
impl Visible for Broom {
    fn draw(&self, canvas: &mut Canvas) {
        println!("Drawing completed")
    }

    fn hit_test(&self, x: i32, y: i32) -> bool {
        println!("Testing done");
        return true;
    }
}

// if we want to add a helper method, we would have to
// define it in a separate impl block
impl Broom {
    fn get_broom_name(&self) -> () {
        println!("{}", self.name)
    }
}

fn main() {
    println!("Hello, world!");
}
