// enum Ordering {
//     Less,
//     Equal,
//     Greater,
// }

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::format;
use std::mem::size_of;

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

#[derive(Clone, Copy, Debug, PartialEq)]
enum RoughTime {
    // These are called tuple variants
    InThePast(TimeUnit, u32),
    JustNow,
    InTheFuture(TimeUnit, u32),
}

enum Shape {
    Rectangle { lenght: u32, width: u32 },
}

enum Json {
    Null,
    Boolean(bool),
    Number(f64),
    String(String),
    Array(Vec<Json>),
    Object(Box<HashMap<String, Json>>),
}

// Generic types
// enum Option<T> {
//     None,
//     Some(T),
// }
// enum Result<T, E> {
//     Ok(T),
//     Err(E),
// }

// Generic data structures
enum BinaryTree<T> {
    Empty,
    NonEmpty(Box<TreeNode<T>>),
}

struct TreeNode<T> {
    element: T,
    left: BinaryTree<T>,
    right: BinaryTree<T>,
}

fn rough_time_to_english(rt: RoughTime) -> String {
    match rt {
        RoughTime::InThePast(units, count) => format!("{} {} ago", count, units.plural()),
        RoughTime::JustNow => format!("just now"),
        RoughTime::InTheFuture(units, counts) => format!("{} {} from now", counts, units.plural()),
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

    let four_score_and_seven_years_ago = RoughTime::InThePast(TimeUnit::Years, 4 * 20 + 7);

    let three_hours_from_now = RoughTime::InTheFuture(TimeUnit::Hours, 3);

    let square = Shape::Rectangle {
        lenght: 5,
        width: 5,
    };

    let jupiter_tree = BinaryTree::NonEmpty(Box::new(TreeNode {
        element: "Jupiter",
        left: BinaryTree::Empty,
        right: BinaryTree::Empty,
    }));

    let three_hours_from_now_english = rough_time_to_english(three_hours_from_now);
    println!("{}", three_hours_from_now_english);
    
}
