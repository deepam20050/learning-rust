#[derive(Debug)]
pub struct Number {
    pub x: i32,
    pub y: i32, 
}

pub fn add(number_a: &Number, number_b: &Number) -> Number {
    Number {
        x: number_a.x + number_b.x,
        y: number_a.y + number_b.y
    }
}