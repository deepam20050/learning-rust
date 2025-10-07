fn main() {
    let array1: [u32; 6] = [1, 2, 3, 4, 5, 6];
    println!("{:?}", array1);

    let mut array2: [u32; 5] = [5, 4, 3, 3, 1];
    
    array2.sort();
    
    println!("{:?}", array2);

    println!("{}", array1.len() * array2.len());
    

    let mut primes = vec![2, 3, 5, 7, 11];

    println!("{}", primes.iter().product::<i32>());

    primes.push(13);

    println!("{}", primes.iter().product::<i32>());

    let v: Vec<i32> = (0..6).collect();
    println!("{:?}", v);

    // You cannot create a vector with initialized elements
    // using Vec::new(). Vec::new() only creates empty vectors

    let v: Vec<f64> = vec![0.0, 0.707, 1.0, 0.707];
    let a: [f64; 4] = [0.0, -0.707, -1.0, -0.707];

    let sv: &[f64] = &v;
    let sa: &[f64] = &a;

    print(sv);
    print(sa);

    print(&a);
    print(&v);

}

fn print(n: &[f64]) {
    for elt in n {
        println!("{}", elt);
    }
}