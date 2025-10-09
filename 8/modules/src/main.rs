mod calculator;

fn main() {
    let num_a = calculator::Number { x: 2, y: 3 };
    let num_b: calculator::Number = calculator::Number { x: 5, y: 6 };

    let num_c = calculator::add(&num_a, &num_b);

    println!("{:?}", num_c);
}
