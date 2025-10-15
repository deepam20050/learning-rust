// enum Ordering {
//     Less,
//     Equal,
//     Greater,
// }

use std::cmp::Ordering;
use std::mem::size_of;
use std::time;

fn compare(n: i32, m: i32) -> Ordering {
    if n < m {
        Ordering::Less
    } else if n > m {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

// if we don't assign numbers, then Rust will assign
// numbers starting at 0
enum HttpStatus {
    Ok = 200,
    NotModified = 304,
    NotFound = 404,
}

fn http_status_from_u32(n: u32) -> Option<HttpStatus> {
    match n {
        200 => Some(HttpStatus::Ok),
        304 => Some(HttpStatus::NotModified),
        404 => Some(HttpStatus::NotFound),
        _ => None,
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TimeUnit {
    Seconds,
    Minutes,
    Hours,
    Days,
    Months,
    Years,
}

// we can define methods on Enums!
impl TimeUnit {
    fn plural(self) -> &'static str {
        match self {
            TimeUnit::Seconds => "seconds",
            TimeUnit::Minutes => "minutes",
            TimeUnit::Hours => "hours",
            TimeUnit::Days => "days",
            TimeUnit::Months => "months",
            TimeUnit::Years => "years",
        }
    }

    fn singular(self) -> &'static str {
        self.plural().trim_end_matches('s')
    }
}

fn main() {
    assert_eq!(HttpStatus::Ok as i32, 200);
    assert_eq!(size_of::<HttpStatus>(), 2);
    match http_status_from_u32(405) {
        Some(HttpStatus) => println!("Matched"),
        None => println!("No match!"),
    }
    match http_status_from_u32(200) {
        Some(HttpStatus) => println!("Matched"),
        None => println!("No match!"),
    }

    let time_unit = TimeUnit::Days;
    println!("{}", time_unit.plural());
    println!("{}", time_unit.singular());
}
