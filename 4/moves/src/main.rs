#[derive(Debug)]
struct Person {
    name: Option<String>,
    birth: i32,
}

fn main() {
    let mut composers = Vec::new();
    composers.push(Person {
        name: Some("Rajesh".to_string()),
        birth: 1525
    });

    // let first_name = std::mem::replace(&mut composers[0].name, None);

    // println!("{:?}", first_name);
    // println!("{:?}", composers);


    // same effect as 

    let first_name = composers[0].name.take();

    println!("{:?}", first_name);
    println!("{:?}", composers);
}
