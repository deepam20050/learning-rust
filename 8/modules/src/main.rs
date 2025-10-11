mod calculator;
use calculator::*;
mod plant_structures;

fn main() {
    let num_a = Number { x: 2, y: 3 };
    let num_b: calculator::Number = Number { x: 5, y: 6 };

    let num_c = add(&num_a, &num_b);

    let root = plant_structures::roots::Root { root_length: 1 };

    let leaf = plant_structures::leaves::Leaf {
        colour: "Green".to_string(),
    };

    let stem = plant_structures::stems::Stem { stem_length: 50 };

    println!("{:?} \n {:?} \n {:?}", root, leaf, stem);

    println!("{:?}", num_c);
}
