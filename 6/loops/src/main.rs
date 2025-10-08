fn main() {
    let mut strings: Vec<String> = vec!["Hi".to_string(), "Bye".to_string()];

    for s in strings { // each String is moved into s here -- pass by value
        println!("{}", s);
    }

    println!("{} ", strings.len()); // doesn't work

    // options ?
    
    // 1 -- pass by shared reference
    for rs in &strings {
        println!("{}", *rs);
    }

    // 2 -- pass by mutable reference
    for mrs in &mut strings {
        println!("{}", *mrs);
    }
}
