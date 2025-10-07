use std::rc::Rc;

fn main() {
    let mut s: Rc<String> = Rc::new("Mukesh".to_string());
    let t: Rc<String> = s.clone();
    let u = t.clone();
    assert_eq!(t, s);
    assert_eq!(u, s);
    assert!(s.contains("kesh"));

    s = Rc::new("Suresh".to_string()); 

    println!("{}", u);
    println!("{}", s);
    println!("{}", t);
}
