#![allow (unused_variables)]

#[derive(Debug)]
struct CubeStat {
    id: u64,
}

#[derive(Debug)]
enum StatusMessage {
    Ok,
}

fn check_status(sat_id: CubeStat) -> CubeStat {
    println!("{:?}: {:?}", sat_id, StatusMessage::Ok);
    sat_id
}

fn main() {
    let sat_a = CubeStat { id: 0 };
    let sat_b = CubeStat { id: 1 };
    let sat_c = CubeStat { id: 2 };
    
    let sat_a = check_status(sat_a);
    let sat_b = check_status(sat_b);
    let sat_c = check_status(sat_c);

    // println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
    
    let a_status = check_status(sat_a);
    let b_status = check_status(sat_b);
    let c_status = check_status(sat_c);

    println!("a: {:?}, b: {:?}, c: {:?}", a_status, b_status, c_status);
}
