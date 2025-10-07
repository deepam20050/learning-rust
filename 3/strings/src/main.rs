fn main() {
    let speech = "\"Hello!\" 'suppp??";
    println!("{}", speech);

    println!("In the room the women come and go,
            Singing of Mount Abora");

    println!("It was a bright, cold day in April, and \
        there were four of us-\
        more or less.");


    let default_win_path = r"C:\Program Files\Gorillas";

    let modified_win_path = r##"C:""\n"Gorilla"##;

    println!("{}", default_win_path); 
    println!("{}", modified_win_path);

    let method = b"GET";
    println!("{:?}", method);

    assert_eq!(method, &[b'G', b'E', b'T']);

    let noodles = "noodles".to_string();
    let oodles = &noodles[1..];
    let poodles = "ಅ_ಅ";


    // len() method returns the number of bytes in the string
    // not the number of characters
    println!("{}", noodles.len());
    println!("{}", noodles.chars().count());
    println!("{}", poodles.len());
    println!("{}", poodles.chars().count());


    let mut s = "Hello";

    // s[0] = 'c'; -> impossible to modify a &str

    // String is similar to Vec
    
    let bits = vec!["Go", "to", "Delhi"];
    let joined_string = bits.join(" ");
    println!("{}", joined_string);
    println!("{}", bits.concat());

    decode(vec![b'A', b'B']);
}


// type aliases - similar to typedef or using in C++
type Bytes = Vec<u8>;

fn decode(data: Bytes) {
    println!("Decoded => {:?}", data);
}
